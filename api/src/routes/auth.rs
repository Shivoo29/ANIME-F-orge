use crate::models::{AuthResponse, Claims, LoginRequest, RegisterRequest, User, UserResponse};
use crate::utils::{create_jwt, ApiError};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

/// POST /api/v1/auth/register - Register a new user
pub async fn register(
    pool: web::Data<PgPool>,
    req: web::Json<RegisterRequest>,
) -> Result<impl Responder, ApiError> {
    // Validate request
    req.validate()
        .map_err(|e| ApiError::ValidationError(format!("Validation failed: {}", e)))?;

    // Check if user already exists
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1 OR username = $2",
    )
    .bind(&req.email)
    .bind(&req.username)
    .fetch_optional(pool.get_ref())
    .await?;

    if existing_user.is_some() {
        return Err(ApiError::Conflict(
            "User with this email or username already exists".to_string(),
        ));
    }

    // Hash password
    let password_hash = hash(&req.password, DEFAULT_COST)?;

    // Create user
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&req.username)
    .bind(&req.email)
    .bind(&password_hash)
    .fetch_one(pool.get_ref())
    .await?;

    // Generate JWT token
    let token = create_jwt(&user.id.to_string())?;

    Ok(HttpResponse::Created().json(AuthResponse {
        token,
        user: user.into(),
    }))
}

/// POST /api/v1/auth/login - Login user
pub async fn login(
    pool: web::Data<PgPool>,
    req: web::Json<LoginRequest>,
) -> Result<impl Responder, ApiError> {
    // Find user by email
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&req.email)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiError::Unauthorized("Invalid email or password".to_string()))?;

    // Verify password
    let password_valid = verify(&req.password, &user.password_hash)?;
    if !password_valid {
        return Err(ApiError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    // Generate JWT token
    let token = create_jwt(&user.id.to_string())?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: user.into(),
    }))
}

/// POST /api/v1/auth/logout - Logout user
pub async fn logout() -> Result<impl Responder, ApiError> {
    // In a JWT-based authentication system, logout is typically handled client-side
    // by removing the token. We'll just return a success message.
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}

/// GET /api/v1/auth/me - Get current user
pub async fn me(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let extensions = req.extensions();
    let claims = extensions
        .get::<Claims>()
        .ok_or_else(|| ApiError::Unauthorized("No authentication token provided".to_string()))?
        .clone();

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::Unauthorized("Invalid user ID in token".to_string()))?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/me", web::get().to(me)),
    );
}
