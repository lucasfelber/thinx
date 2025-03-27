use axum::extract::State;
use serde_json::{Value, json};
use utoipa_axum::{router::OpenApiRouter, routes};
use validator::Validate;

use crate::{
    AppConfig,
    errors::{AppResult, Json},
    models::thought::{Thought, ThoughtBody, ThoughtCommand},
};

pub fn router() -> OpenApiRouter<AppConfig> {
    OpenApiRouter::new().routes(routes!(create_thought))
}

#[utoipa::path(post, path = "/")]
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
