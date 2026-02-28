// Auth middleware: session-based authentication.
//
// The backend handles the full Zitadel OIDC PKCE flow. After successful auth,
// the user_id is stored in an opaque httpOnly session cookie. The `AuthUser`
// extractor reads from this session — the frontend never sees tokens.

use actix_session::SessionExt;
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use std::future::{ready, Ready};

use crate::errors::AppError;

/// Extractor that requires a valid session with a user_id.
/// Use this in handler signatures to enforce authentication.
pub struct AuthUser {
    pub user_id: i32,
}

impl FromRequest for AuthUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session = req.get_session();

        match session.get::<i32>("user_id") {
            Ok(Some(user_id)) => ready(Ok(AuthUser { user_id })),
            Ok(None) => ready(Err(AppError::Unauthorized)),
            Err(_) => ready(Err(AppError::Unauthorized)),
        }
    }
}
