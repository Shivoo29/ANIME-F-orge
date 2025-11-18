use crate::models::{
    Animation, AnimationResponse, Claims, PaginatedResponse, PaginationParams, UpdateProfileRequest,
    User, UserResponse,
};
use crate::utils::ApiError;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

/// GET /api/v1/users/:id/profile - Get user profile
pub async fn get_profile(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    let user_id = path.into_inner();

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

/// GET /api/v1/users/:id/animations - Get user's animations
pub async fn get_user_animations(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    params: web::Query<PaginationParams>,
) -> Result<impl Responder, ApiError> {
    let user_id = path.into_inner();
    let offset = params.offset();
    let limit = params.limit;

    // Verify user exists
    let _user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    // Get total count
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM animations WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(pool.get_ref())
        .await?;

    // Get user's animations
    let animations = sqlx::query_as::<_, Animation>(
        r#"
        SELECT * FROM animations
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let response: PaginatedResponse<AnimationResponse> = PaginatedResponse {
        data: animations.into_iter().map(|a| a.into()).collect(),
        page: params.page,
        limit: params.limit,
        total: total.0,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// PUT /api/v1/users/:id/profile - Update profile
pub async fn update_profile(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<UpdateProfileRequest>,
) -> Result<impl Responder, ApiError> {
    // Validate request
    body.validate()
        .map_err(|e| ApiError::ValidationError(format!("Validation failed: {}", e)))?;

    let user_id = path.into_inner();

    // Get user ID from JWT claims
    let extensions = req.extensions();
    let claims = extensions
        .get::<Claims>()
        .ok_or_else(|| ApiError::Unauthorized("No authentication token provided".to_string()))?
        .clone();

    let current_user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::Unauthorized("Invalid user ID in token".to_string()))?;

    // Check if user is updating their own profile
    if user_id != current_user_id {
        return Err(ApiError::Unauthorized(
            "You can only update your own profile".to_string(),
        ));
    }

    // Check if user exists
    let existing = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    // Build dynamic update query
    let mut query = String::from("UPDATE users SET ");
    let mut updates = Vec::new();
    let mut param_count = 1;

    if body.username.is_some() {
        updates.push(format!("username = ${}", param_count));
        param_count += 1;
    }

    if body.email.is_some() {
        updates.push(format!("email = ${}", param_count));
        param_count += 1;
    }

    if updates.is_empty() {
        return Ok(HttpResponse::Ok().json(UserResponse::from(existing)));
    }

    query.push_str(&updates.join(", "));
    query.push_str(&format!(" WHERE id = ${} RETURNING *", param_count));

    // Execute update
    let mut query_builder = sqlx::query_as::<_, User>(&query);

    if let Some(username) = &body.username {
        query_builder = query_builder.bind(username);
    }
    if let Some(email) = &body.email {
        query_builder = query_builder.bind(email);
    }

    query_builder = query_builder.bind(user_id);

    let user = query_builder.fetch_one(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/{id}/profile", web::get().to(get_profile))
            .route("/{id}/profile", web::put().to(update_profile))
            .route("/{id}/animations", web::get().to(get_user_animations)),
    );
}
