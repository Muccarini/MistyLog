use actix_session::Session;
use actix_web::{web, HttpResponse};
use openidconnect::{
    core::{CoreClient, CoreResponseType},
    AuthenticationFlow, CsrfToken, Nonce, PkceCodeChallenge,
    PkceCodeVerifier, Scope,
};
use sea_orm::{DatabaseConnection};

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

    // TODO: Implement proper token exchange with HTTP client
    // For now, return a stubbed response to get the server running
    Ok(HttpResponse::Found()
        .append_header(("Location", cfg.frontend_url.clone()))
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
