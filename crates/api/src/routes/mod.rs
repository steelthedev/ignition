use axum::{
    Router,
    routing::{get, post},
};

pub mod health;
pub use health::*;

pub mod tx;
pub use tx::*;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(get_health))
        .route("/tx/submit", post(submit_tx))
}
