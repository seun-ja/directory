use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    app_state::AppState,
    db::{TeacherSchema, teachers_d::TeacherDb},
    error::ApiError,
};

#[axum::debug_handler]
pub async fn teacher_data(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TeacherSchema>, ApiError> {
    TeacherDb {
        pool: app_state.pg_pool,
    }
    .teacher_data_by_id(&id)
    .await
    .map(Json)
}
