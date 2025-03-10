use sqlx::PgPool;

use crate::error::ApiError;

use super::SchoolDirectorySchema;

pub struct SchoolDirectoryDb {
    pub pool: PgPool,
}

impl SchoolDirectoryDb {
    pub async fn school_data(
        &mut self,
        school_id: &str,
    ) -> Result<SchoolDirectorySchema, ApiError> {
        Ok(sqlx::query_as!(
            SchoolDirectorySchema,
            r#"
                SELECT * FROM directory WHERE id = $1
            "#,
            school_id
        )
        .fetch_one(&self.pool)
        .await?)
    }
}
