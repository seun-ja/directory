use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{config::Config, error::ApiError};

#[derive(Debug, Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
}

impl AppState {
    pub async fn build(config: &Config) -> Result<Self, ApiError> {
        let pg_pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(5))
            .max_connections(100) // Default max 100 in postgres image.
            .connect(&config.database_url) //
            .await?;

        Ok(Self { pg_pool })
    }
}
