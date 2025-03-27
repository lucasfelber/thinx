use axum::{Router, extract::MatchedPath, http::Request};
use derive_builder::Builder;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing::info_span;

mod errors;
mod models;
mod routes;

#[derive(Builder, Clone)]
pub struct AppConfig {
    #[builder(setter(into))]
    pub url: String,
    pub postgres_pool: sqlx::PgPool,
}

impl AppConfig {
    pub async fn new() -> Self {
        // Read environment variables from .env file if present
        dotenvy::dotenv().ok();

        AppConfigBuilder::default()
            .url(std::env::var("URL").expect("URL not set"))
            .postgres_pool({
                let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
                tracing::info!("Initializing postgres connection to {}", database_url);

                PgPoolOptions::new()
                    .connect(&database_url)
                    .await
                    .expect("Postgres connection failed")
            })
            .build()
            .expect("Could not build app config")
    }

    pub async fn run_database_migrations(&self) {
        tracing::info!("Checking for database migrations");

        sqlx::migrate!()
            .run(&self.postgres_pool)
            .await
            .expect("Database migrations failed");
    }

    pub fn router(&self) -> Router {
        Router::new()
            // Add routes
            .merge(routes::router())
            // Put config into server state
            .with_state(self.clone())
            // Add tracing middleware
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                    )
                }),
            )
    }
}
