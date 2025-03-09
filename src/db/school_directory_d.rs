use sqlx::PgConnection;
use uuid::Uuid;

use crate::error::ApiError;

use super::SchoolDirectorySchema;

pub struct SchoolDirectoryDb {
    conn: PgConnection,
}

impl SchoolDirectoryDb {
    pub async fn school_data(
        &mut self,
        school_id: Uuid,
    ) -> Result<SchoolDirectorySchema, ApiError> {
        Ok(sqlx::query_as!(
            SchoolDirectorySchema,
            r#"
            SELECT * FROM directory WHERE id = $1
            "#,
            school_id
        )
        .fetch_one(&mut self.conn)
        .await?)
    }
}
