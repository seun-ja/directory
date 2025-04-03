use sqlx::PgPool;

use crate::{error::ApiError, services::search::Searchable};

use super::SchoolDirectorySchema;

pub struct SchoolDirectoryDb {
    pub pool: PgPool,
}

impl SchoolDirectoryDb {
    pub async fn school_data(&mut self) -> Result<SchoolDirectory, ApiError> {
        sqlx::query_as!(SchoolDirectorySchema, r#"SELECT * FROM directory"#)
            .fetch_all(&self.pool)
            .await
            .map_err(ApiError::Sqlx)
            .map(|data| SchoolDirectory { data })
    }
    pub async fn get_school_data_by_term(
        &mut self,
        school_id: &str,
    ) -> Result<SchoolDirectorySchema, ApiError> {
        sqlx::query_as!(
            SchoolDirectorySchema,
            r#"
                SELECT * FROM directory WHERE id = $1
            "#,
            school_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(ApiError::Sqlx)
    }

    pub async fn store_school_data(&mut self, data: SchoolDirectorySchema) -> Result<(), ApiError> {
        sqlx::query!(
            r#"
                INSERT INTO directory
                (
            id, name, email, phone, address, state, local_government, postal_code, website,
            about, current_population, staff_strength, year_established, curriculum_offered,
            government_approved, subjects_taught, awards_recognition, management_board
                )
                Values
                (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18
                )
            "#,
            data.id,
            data.name,
            data.email,
            data.phone,
            data.address,
            data.state,
            data.local_government,
            data.postal_code,
            data.website,
            data.about,
            data.current_population,
            data.staff_strength,
            data.year_established,
            data.curriculum_offered,
            data.government_approved,
            &data.subjects_taught,
            &data.awards_recognition,
            &data.management_board,
        )
        .execute(&self.pool)
        .await
        .map_err(ApiError::Sqlx)
        .map(|_| ())
    }
}

pub struct SchoolDirectory {
    pub data: Vec<SchoolDirectorySchema>,
}

impl Searchable for SchoolDirectory {}
