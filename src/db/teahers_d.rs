use sqlx::PgConnection;
use uuid::Uuid;

use crate::error::ApiError;

use super::TeacherSchema;

pub struct TeacherDb {
    conn: PgConnection,
}

impl TeacherDb {
    pub async fn teacher_data(&mut self, id: Uuid) -> Result<TeacherSchema, ApiError> {
        Ok(sqlx::query_as!(
            TeacherSchema,
            r#"
            SELECT * FROM teachers WHERE id = $1
            "#,
            id
        )
        .fetch_one(&mut self.conn)
        .await?)
    }
}
