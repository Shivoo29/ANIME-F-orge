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

/// GET /api/v1/search?q=query - Search animations
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

    // Get total count
    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM animations
        WHERE title ILIKE $1 OR description ILIKE $1
        "#,
    )
    .bind(&search_term)
    .fetch_one(pool.get_ref())
    .await?;

    // Search animations using full-text search
    let animations = sqlx::query_as::<_, Animation>(
        r#"
        SELECT * FROM animations
        WHERE title ILIKE $1 OR description ILIKE $1
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
    cfg.route("/search", web::get().to(search_animations));
}
