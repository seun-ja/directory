use axum::Json;

use crate::{app_state::AppState, db::SchoolDirectorySchema, error::ApiError};

pub fn school_data(_app_state: AppState) -> Result<Json<SchoolDirectorySchema>, ApiError> {
    todo!()
}
