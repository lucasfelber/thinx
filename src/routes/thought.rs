use axum::extract::{Path, State};
use serde_json::{Value, json};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppConfig,
    errors::{AppResult, Json},
    models::thought::{Thought, ThoughtBody, ThoughtCommand},
};

pub fn router() -> OpenApiRouter<AppConfig> {
    OpenApiRouter::new()
        .routes(routes!(create_thought, read_thoughts))
        .routes(routes!(read_thought))
}

#[utoipa::path(post, path = "")]
async fn create_thought(
    State(app_config): State<AppConfig>,
    Json(req): Json<ThoughtBody<ThoughtCommand>>,
) -> AppResult<Value> {
    // Validate
    req.thought.validate()?;

    // Insert into database
    let mut transaction = app_config.postgres_pool.begin().await?;
    let thought = Thought::insert(&req.thought, &mut transaction).await?;
    transaction.commit().await?;

    Ok(Json(json!(thought)))
}

#[utoipa::path(get, path = "")]
async fn read_thoughts(
    State(app_config): State<AppConfig>,
) -> AppResult<Value> {
    let mut transaction = app_config.postgres_pool.begin().await?;
    let thoughts = Thought::get(&mut transaction).await?;
    transaction.commit().await?;

    Ok(Json(json!(thoughts)))
}

#[utoipa::path(get, path = "/{id}")]
async fn read_thought(
    State(app_config): State<AppConfig>,
    Path(id): Path<Uuid>,
) -> AppResult<Value> {
    let mut transaction = app_config.postgres_pool.begin().await?;
    let thought = Thought::get_id(&id, &mut transaction).await?;
    transaction.commit().await?;

    Ok(Json(json!(thought)))
}
