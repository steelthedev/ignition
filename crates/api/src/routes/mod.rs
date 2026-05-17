use axum::{
    Router,
    routing::{get, post},
};

pub mod health;
pub use health::*;

pub mod tx;
pub use tx::*;

pub mod ws;
pub use ws::*;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(get_health))
        .route("/tx/submit", post(submit_tx))
        .route("/tx/{id}", get(get_tx_status))
        .route("/ws", get(ws_handler))
}
