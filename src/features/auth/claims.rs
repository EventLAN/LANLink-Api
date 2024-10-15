use async_trait::async_trait;
use jsonwebtoken::{decode, Validation};
use axum_extra::TypedHeader;
use crate::features::auth::auth_errors::AuthError;
use serde::{ Serialize, Deserialize };
use axum::{ http::{ request::Parts }, extract::FromRequestParts, RequestPartsExt };
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use uuid::Uuid;
use crate::features::auth::keys::KEYS;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub user_public_id: Uuid,
    pub exp: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims where S: Send + Sync {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>().await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &KEYS.decoding,
            &Validation::default()
        ).map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}