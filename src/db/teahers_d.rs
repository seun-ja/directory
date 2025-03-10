use sqlx::PgPool;

use crate::error::ApiError;

use super::TeacherSchema;

pub struct TeacherDb {
    pub pool: PgPool,
}

impl TeacherDb {
    pub async fn teacher_data(&mut self, id: &str) -> Result<TeacherSchema, ApiError> {
        Ok(sqlx::query_as!(
            TeacherSchema,
            r#"
                SELECT * FROM teachers WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?)
    }
}
