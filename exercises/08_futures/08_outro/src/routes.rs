use crate::model::{Ticket, TicketDraft, TicketId, TicketPatch};
use crate::store::{StoreError, TicketStore};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use std::sync::Arc;
use tokio::sync::RwLock;

type SharedStore = Arc<RwLock<TicketStore>>;

/// - `POST /tickets` creates a ticket
/// - `GET /tickets/{id}` retrieves a ticket
/// - `PATCH /tickets/{id}` updates a ticket
pub fn router() -> Router {
    Router::new()
        .route("/tickets", post(create_ticket))
        .route("/tickets/{id}", get(get_ticket).patch(patch_ticket))
        .with_state(SharedStore::default())
}

impl IntoResponse for StoreError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            StoreError::NotFound => (StatusCode::NOT_FOUND, "Ticket not found"),
            StoreError::Invalid(message) => (StatusCode::BAD_REQUEST, message),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

async fn create_ticket(
    State(store): State<SharedStore>,
    Json(draft): Json<TicketDraft>,
) -> Result<(StatusCode, Json<Ticket>), StoreError> {
    let ticket = store.write().await.add(draft)?;
    Ok((StatusCode::CREATED, Json(ticket)))
}

async fn get_ticket(
    State(store): State<SharedStore>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>, StoreError> {
    let store = store.read().await;
    let ticket = store.get(TicketId(id)).ok_or(StoreError::NotFound)?;
    Ok(Json(ticket.clone()))
}

async fn patch_ticket(
    State(store): State<SharedStore>,
    Path(id): Path<u64>,
    Json(patch): Json<TicketPatch>,
) -> Result<Json<Ticket>, StoreError> {
    let ticket = store.write().await.patch(TicketId(id), patch)?;
    Ok(Json(ticket))
}
