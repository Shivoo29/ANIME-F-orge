use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Animation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub file_url: String,
    pub thumbnail_url: Option<String>,
    pub source_code: Option<String>,
    pub duration: Option<BigDecimal>,
    pub views: i32,
    pub downloads: i32,
    pub likes: i32,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAnimationRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub file_url: String,
    pub thumbnail_url: Option<String>,
    pub source_code: Option<String>,
    pub duration: Option<f32>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAnimationRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AnimationResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub file_url: String,
    pub thumbnail_url: Option<String>,
    pub source_code: Option<String>,
    pub duration: Option<f32>,
    pub views: i32,
    pub downloads: i32,
    pub likes: i32,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Animation> for AnimationResponse {
    fn from(animation: Animation) -> Self {
        AnimationResponse {
            id: animation.id,
            user_id: animation.user_id,
            title: animation.title,
            description: animation.description,
            file_url: animation.file_url,
            thumbnail_url: animation.thumbnail_url,
            source_code: animation.source_code,
            duration: animation.duration.and_then(|d| d.to_string().parse::<f32>().ok()),
            views: animation.views,
            downloads: animation.downloads,
            likes: animation.likes,
            is_public: animation.is_public,
            created_at: animation.created_at,
            updated_at: animation.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    20
}

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.limit
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
}
