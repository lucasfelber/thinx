use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;

#[derive(Deserialize, ToSchema)]
pub struct ThoughtBody<T: ToSchema> {
    pub thought: T,
}

#[derive(Serialize)]
pub struct Thought {
    id: Uuid,
    content: String,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct ThoughtCommand {
    #[validate(length(min = 1, max = 20))]
    content: String,
}

impl Thought {
    pub async fn insert(
        command: &ThoughtCommand,
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Self, AppError> {
        let thought = sqlx::query_file_as!(Self, "queries/thought/insert.sql", command.content,)
            .fetch_one(&mut **transaction)
            .await?;

        Ok(thought)
    }

    pub async fn get(
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Vec<Self>, AppError> {
        let thoughts = sqlx::query_file_as!(Self, "queries/thought/get.sql")
            .fetch_all(&mut **transaction)
            .await?;

        Ok(thoughts)
    }
}
