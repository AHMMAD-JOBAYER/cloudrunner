use axum::{
    RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // teacher id
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(teacher_id: Uuid, expiration_hours: i64) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(expiration_hours);

        Self {
            sub: teacher_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn create_jwt(
    teacher_id: Uuid,
    secret: &str,
    expiration_hours: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(teacher_id, expiration_hours);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub struct AuthenticatedTeacher {
    pub teacher_id: Uuid,
}

impl<S> FromRequestParts<S> for AuthenticatedTeacher
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing or invalid authorization header".to_string(),
                )
            })?;

        // Get JWT secret from environment
        let secret = std::env::var("JWT_SECRET").map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server configuration error".to_string(),
            )
        })?;

        // Verify the JWT token
        let claims = verify_jwt(bearer.token(), &secret).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token".to_string(),
            )
        })?;

        // Parse teacher ID from claims
        let teacher_id = Uuid::parse_str(&claims.sub).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid token payload".to_string(),
            )
        })?;

        Ok(AuthenticatedTeacher { teacher_id })
    }
}
