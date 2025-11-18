use crate::models::{
    Animation, AnimationResponse, Claims, CreateAnimationRequest, PaginatedResponse,
    PaginationParams, UpdateAnimationRequest,
};
use crate::utils::ApiError;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

/// GET /api/v1/animations - List all public animations (paginated)
pub async fn list_animations(
    pool: web::Data<PgPool>,
    params: web::Query<PaginationParams>,
) -> Result<impl Responder, ApiError> {
    let offset = params.offset();
    let limit = params.limit;

    // Get total count of public animations
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM animations WHERE is_public = TRUE")
        .fetch_one(pool.get_ref())
        .await?;

    // Get paginated public animations
    let animations = sqlx::query_as::<_, Animation>(
        r#"
        SELECT * FROM animations
        WHERE is_public = TRUE
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
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

/// POST /api/v1/animations - Create new animation
pub async fn create_animation(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CreateAnimationRequest>,
) -> Result<impl Responder, ApiError> {
    // Validate request
    body.validate()
        .map_err(|e| ApiError::ValidationError(format!("Validation failed: {}", e)))?;

    // Get user ID from JWT claims
    let extensions = req.extensions();
    let claims = extensions
        .get::<Claims>()
        .ok_or_else(|| ApiError::Unauthorized("No authentication token provided".to_string()))?
        .clone();

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::Unauthorized("Invalid user ID in token".to_string()))?;

    // Create animation with new fields
    let duration_decimal = body.duration.and_then(|d| d.to_string().parse::<bigdecimal::BigDecimal>().ok());
    let is_public = body.is_public.unwrap_or(true);

    let animation = sqlx::query_as::<_, Animation>(
        r#"
        INSERT INTO animations (user_id, title, description, file_url, thumbnail_url, source_code, duration, is_public)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(&body.title)
    .bind(&body.description)
    .bind(&body.file_url)
    .bind(&body.thumbnail_url)
    .bind(&body.source_code)
    .bind(duration_decimal)
    .bind(is_public)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(AnimationResponse::from(animation)))
}

/// GET /api/v1/animations/:id - Get animation details
pub async fn get_animation(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    let animation_id = path.into_inner();

    // Increment view count
    let animation = sqlx::query_as::<_, Animation>(
        r#"
        UPDATE animations
        SET views = views + 1
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(animation_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound("Animation not found".to_string()))?;

    Ok(HttpResponse::Ok().json(AnimationResponse::from(animation)))
}

/// PUT /api/v1/animations/:id - Update animation
pub async fn update_animation(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<UpdateAnimationRequest>,
) -> Result<impl Responder, ApiError> {
    // Validate request
    body.validate()
        .map_err(|e| ApiError::ValidationError(format!("Validation failed: {}", e)))?;

    let animation_id = path.into_inner();

    // Get user ID from JWT claims
    let extensions = req.extensions();
    let claims = extensions
        .get::<Claims>()
        .ok_or_else(|| ApiError::Unauthorized("No authentication token provided".to_string()))?
        .clone();

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::Unauthorized("Invalid user ID in token".to_string()))?;

    // Check if animation exists and belongs to user
    let existing = sqlx::query_as::<_, Animation>("SELECT * FROM animations WHERE id = $1")
        .bind(animation_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiError::NotFound("Animation not found".to_string()))?;

    if existing.user_id != user_id {
        return Err(ApiError::Unauthorized(
            "You don't have permission to update this animation".to_string(),
        ));
    }

    // Build dynamic update query
    let mut query = String::from("UPDATE animations SET ");
    let mut updates = Vec::new();
    let mut param_count = 1;

    if let Some(title) = &body.title {
        updates.push(format!("title = ${}", param_count));
        param_count += 1;
    }

    if body.description.is_some() {
        updates.push(format!("description = ${}", param_count));
        param_count += 1;
    }

    if body.thumbnail_url.is_some() {
        updates.push(format!("thumbnail_url = ${}", param_count));
        param_count += 1;
    }

    if updates.is_empty() {
        return Ok(HttpResponse::Ok().json(AnimationResponse::from(existing)));
    }

    query.push_str(&updates.join(", "));
    query.push_str(&format!(" WHERE id = ${} RETURNING *", param_count));

    // Execute update
    let mut query_builder = sqlx::query_as::<_, Animation>(&query);

    if let Some(title) = &body.title {
        query_builder = query_builder.bind(title);
    }
    if let Some(description) = &body.description {
        query_builder = query_builder.bind(description);
    }
    if let Some(thumbnail_url) = &body.thumbnail_url {
        query_builder = query_builder.bind(thumbnail_url);
    }

    query_builder = query_builder.bind(animation_id);

    let animation = query_builder.fetch_one(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(AnimationResponse::from(animation)))
}

/// DELETE /api/v1/animations/:id - Delete animation
pub async fn delete_animation(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    let animation_id = path.into_inner();

    // Get user ID from JWT claims
    let extensions = req.extensions();
    let claims = extensions
        .get::<Claims>()
        .ok_or_else(|| ApiError::Unauthorized("No authentication token provided".to_string()))?
        .clone();

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::Unauthorized("Invalid user ID in token".to_string()))?;

    // Check if animation exists and belongs to user
    let existing = sqlx::query_as::<_, Animation>("SELECT * FROM animations WHERE id = $1")
        .bind(animation_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiError::NotFound("Animation not found".to_string()))?;

    if existing.user_id != user_id {
        return Err(ApiError::Unauthorized(
            "You don't have permission to delete this animation".to_string(),
        ));
    }

    // Delete animation
    sqlx::query("DELETE FROM animations WHERE id = $1")
        .bind(animation_id)
        .execute(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Animation deleted successfully"
    })))
}

/// GET /api/v1/animations/:id/download - Download animation file
pub async fn download_animation(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    let animation_id = path.into_inner();

    // Increment download count and get animation
    let animation = sqlx::query_as::<_, Animation>(
        r#"
        UPDATE animations
        SET downloads = downloads + 1
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(animation_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound("Animation not found".to_string()))?;

    // Return the file URL for the client to download
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "file_url": animation.file_url,
        "title": animation.title,
        "downloads": animation.downloads
    })))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/animations")
            .route("", web::get().to(list_animations))
            .route("", web::post().to(create_animation))
            .route("/{id}", web::get().to(get_animation))
            .route("/{id}", web::put().to(update_animation))
            .route("/{id}", web::delete().to(delete_animation))
            .route("/{id}/download", web::get().to(download_animation)),
    );
}
