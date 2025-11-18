use crate::models::{Animation, AnimationResponse, PaginatedResponse, PaginationParams};
use crate::utils::ApiError;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    q: String,
    #[serde(flatten)]
    pagination: PaginationParams,
}

/// GET /api/v1/marketplace/featured - Get featured animations
/// Featured animations are determined by high like count (>10 likes)
pub async fn get_featured(
    pool: web::Data<PgPool>,
    params: web::Query<PaginationParams>,
) -> Result<impl Responder, ApiError> {
    let offset = params.offset();
    let limit = params.limit;

    // Get total count of featured animations
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM animations WHERE is_public = TRUE AND likes > 10"
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Get featured animations (high likes, public only)
    let animations = sqlx::query_as::<_, Animation>(
        r#"
        SELECT * FROM animations
        WHERE is_public = TRUE AND likes > 10
        ORDER BY likes DESC, created_at DESC
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

/// GET /api/v1/marketplace/trending - Get trending animations
/// Trending animations are recent (last 7 days) with high view counts
pub async fn get_trending(
    pool: web::Data<PgPool>,
    params: web::Query<PaginationParams>,
) -> Result<impl Responder, ApiError> {
    let offset = params.offset();
    let limit = params.limit;

    // Get total count of trending animations
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM animations WHERE is_public = TRUE AND created_at > NOW() - INTERVAL '7 days'"
    )
    .fetch_one(pool.get_ref())
    .await?;

    // Get trending animations (recent with high views, public only)
    let animations = sqlx::query_as::<_, Animation>(
        r#"
        SELECT * FROM animations
        WHERE is_public = TRUE AND created_at > NOW() - INTERVAL '7 days'
        ORDER BY views DESC, likes DESC, created_at DESC
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

/// GET /api/v1/marketplace/search?q=query - Search animations
/// Searches through title and description using case-insensitive pattern matching
pub async fn search_animations(
    pool: web::Data<PgPool>,
    query: web::Query<SearchQuery>,
) -> Result<impl Responder, ApiError> {
    if query.q.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Search query cannot be empty".to_string(),
        ));
    }

    let search_term = format!("%{}%", query.q.trim());
    let offset = query.pagination.offset();
    let limit = query.pagination.limit;

    // Get total count of matching animations
    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM animations
        WHERE is_public = TRUE AND (title ILIKE $1 OR description ILIKE $1)
        "#,
    )
    .bind(&search_term)
    .fetch_one(pool.get_ref())
    .await?;

    // Search animations using pattern matching (public only)
    let animations = sqlx::query_as::<_, Animation>(
        r#"
        SELECT * FROM animations
        WHERE is_public = TRUE AND (title ILIKE $1 OR description ILIKE $1)
        ORDER BY
            CASE
                WHEN title ILIKE $1 THEN 1
                WHEN description ILIKE $1 THEN 2
                ELSE 3
            END,
            views DESC,
            created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(&search_term)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let response: PaginatedResponse<AnimationResponse> = PaginatedResponse {
        data: animations.into_iter().map(|a| a.into()).collect(),
        page: query.pagination.page,
        limit: query.pagination.limit,
        total: total.0,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/marketplace")
            .route("/featured", web::get().to(get_featured))
            .route("/trending", web::get().to(get_trending))
            .route("/search", web::get().to(search_animations)),
    );
}
