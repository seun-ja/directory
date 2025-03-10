use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    app_state::AppState,
    db::{SchoolDirectorySchema, school_directory_d::SchoolDirectoryDb},
    error::ApiError,
};

pub async fn school_data(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<SchoolDirectorySchema>, ApiError> {
    SchoolDirectoryDb {
        pool: app_state.pg_pool,
    }
    .school_data(&id)
    .await
    .map(Json)
}
