use axum::{Json, extract::State};

use crate::{AppState, services::entry_service};

#[utoipa::path(
    get,
    path = "/entries",
    responses(
        (
            status = 200,
            description = "Returns entries",
            body = [String]
        )
    )
)]
pub async fn get_entries(State(state): State<AppState>) -> Json<Vec<String>> {
    let entries = entry_service::get_entries(&state.db).await.unwrap();

    Json(entries)
}
