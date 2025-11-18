use crate::models::Claims;
use crate::utils::ApiError;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;

/// Create a new JWT token for a user
pub fn create_jwt(user_id: &str) -> Result<String, ApiError> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = env::var("JWT_EXPIRATION")
        .unwrap_or_else(|_| "86400".to_string())
        .parse::<i64>()
        .unwrap_or(86400);

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::seconds(expiration)).timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| ApiError::InternalServerError(format!("Failed to create token: {}", e)))
}

/// Verify and decode a JWT token
pub fn verify_jwt(token: &str) -> Result<Claims, ApiError> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| ApiError::Unauthorized(format!("Invalid token: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_creation_and_verification() {
        // Set environment variables for testing
        std::env::set_var("JWT_SECRET", "test_secret_key_for_testing");
        std::env::set_var("JWT_EXPIRATION", "3600");

        let user_id = "test-user-123";

        // Create token
        let token = create_jwt(user_id).expect("Failed to create JWT");
        assert!(!token.is_empty());

        // Verify token
        let claims = verify_jwt(&token).expect("Failed to verify JWT");
        assert_eq!(claims.sub, user_id);
    }
}
