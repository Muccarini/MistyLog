use actix_session::Session;
use actix_web::{web, HttpResponse};
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata, CoreResponseType},
    reqwest::async_http_client,
    AuthenticationFlow, AuthorizationCode, CsrfToken, Nonce, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::config::AppConfig;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::models::user::{self, UserResponse};

/// GET /api/auth/login
///
/// Generates a PKCE challenge, stores verifier + state in the session,
/// and redirects the user to Zitadel's authorize endpoint.
pub async fn login(
    session: Session,
    oidc_client: web::Data<CoreClient>,
) -> Result<HttpResponse, AppError> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_state, nonce) = oidc_client
        .authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("openid".into()))
        .add_scope(Scope::new("profile".into()))
        .add_scope(Scope::new("email".into()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    // Store PKCE verifier, CSRF state, and nonce in the session
    session
        .insert("pkce_verifier", pkce_verifier.secret().clone())
        .map_err(|e| AppError::Internal(format!("Session error: {}", e)))?;
    session
        .insert("csrf_state", csrf_state.secret().clone())
        .map_err(|e| AppError::Internal(format!("Session error: {}", e)))?;
    session
        .insert("nonce", nonce.secret().clone())
        .map_err(|e| AppError::Internal(format!("Session error: {}", e)))?;

    Ok(HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish())
}

/// Query parameters returned by Zitadel on the callback redirect
#[derive(Debug, serde::Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    pub state: String,
}

/// GET /api/auth/callback
///
/// Zitadel redirects here after the user authenticates. We exchange the
/// authorization code for tokens using PKCE, extract user info from the
/// ID token, auto-provision the local user, and set user_id in the session.
/// Then redirect to the frontend.
pub async fn callback(
    db: web::Data<DatabaseConnection>,
    session: Session,
    oidc_client: web::Data<CoreClient>,
    query: web::Query<CallbackQuery>,
    cfg: web::Data<AppConfig>,
) -> Result<HttpResponse, AppError> {
    // Verify CSRF state
    let stored_state: String = session
        .get("csrf_state")
        .map_err(|e| AppError::Internal(format!("Session error: {}", e)))?
        .ok_or(AppError::BadRequest("Missing CSRF state in session".into()))?;

    if query.state != stored_state {
        return Err(AppError::BadRequest("CSRF state mismatch".into()));
    }

    // Retrieve PKCE verifier
    let pkce_verifier_secret: String = session
        .get("pkce_verifier")
        .map_err(|e| AppError::Internal(format!("Session error: {}", e)))?
        .ok_or(AppError::BadRequest(
            "Missing PKCE verifier in session".into(),
        ))?;
    let pkce_verifier = PkceCodeVerifier::new(pkce_verifier_secret);

    // Exchange code for tokens
    let token_response = oidc_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .map_err(|e| AppError::Internal(format!("Token exchange failed: {}", e)))?;

    // Extract claims from the ID token
    let id_token = token_response
        .id_token()
        .ok_or_else(|| AppError::Internal("No ID token in response".into()))?;

    let stored_nonce: String = session
        .get("nonce")
        .map_err(|e| AppError::Internal(format!("Session error: {}", e)))?
        .ok_or(AppError::BadRequest("Missing nonce in session".into()))?;

    let claims = id_token
        .claims(
            &oidc_client.id_token_verifier(),
            &openidconnect::Nonce::new(stored_nonce),
        )
        .map_err(|e| AppError::Internal(format!("ID token verification failed: {}", e)))?;

    let sub = claims.subject().to_string();
    let email = claims
        .email()
        .map(|e| e.to_string())
        .unwrap_or_default();
    let name = claims
        .name()
        .and_then(|n| n.get(None))
        .map(|n| n.to_string());
    let preferred_username = claims
        .preferred_username()
        .map(|u| u.to_string());

    // Auto-provision or update local user
    let user_model = match user::Entity::find()
        .filter(user::Column::Sub.eq(&sub))
        .one(db.get_ref())
        .await?
    {
        Some(existing) => {
            // Update user info from Zitadel if changed
            let mut active: user::ActiveModel = existing.into();
            let now = chrono::Utc::now().naive_utc();
            if !email.is_empty() {
                active.email = Set(email);
            }
            if let Some(ref n) = name {
                active.display_name = Set(Some(n.clone()));
            }
            if let Some(ref u) = preferred_username {
                active.username = Set(u.clone());
            }
            active.updated_at = Set(now);
            active.update(db.get_ref()).await?
        }
        None => {
            let now = chrono::Utc::now().naive_utc();
            let username = preferred_username.unwrap_or_else(|| sub.clone());
            let new_user = user::ActiveModel {
                sub: Set(sub.clone()),
                username: Set(username),
                email: Set(email),
                display_name: Set(name),
                avatar_url: Set(None),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };
            new_user.insert(db.get_ref()).await?
        }
    };

    // Clean up OIDC-related session data
    session.remove("pkce_verifier");
    session.remove("csrf_state");
    session.remove("nonce");

    // Set the session user_id — this is the only thing the frontend sees
    session
        .insert("user_id", user_model.id)
        .map_err(|e| AppError::Internal(format!("Session error: {}", e)))?;

    // Redirect to the frontend
    Ok(HttpResponse::Found()
        .append_header(("Location", cfg.frontend_url.as_str()))
        .finish())
}

/// GET /api/auth/me
///
/// Returns the current user profile from the session.
pub async fn me(
    db: web::Data<DatabaseConnection>,
    auth: AuthUser,
) -> Result<HttpResponse, AppError> {
    let user = user::Entity::find_by_id(auth.user_id)
        .one(db.get_ref())
        .await?
        .ok_or(AppError::Unauthorized)?;

    let response: UserResponse = user.into();
    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/auth/logout
///
/// Clears the session and optionally redirects to Zitadel's end_session endpoint.
pub async fn logout(
    session: Session,
    cfg: web::Data<AppConfig>,
) -> Result<HttpResponse, AppError> {
    session.purge();

    let end_session_url = format!(
        "{}/oidc/v1/end_session?post_logout_redirect_uri={}",
        cfg.zitadel_issuer.trim_end_matches('/'),
        urlencoding::encode(&cfg.zitadel_post_logout_uri),
    );

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out",
        "redirect": end_session_url
    })))
}
