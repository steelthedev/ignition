use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};

use axum::extract::ws::{Message, WebSocket};

use crate::state::AppState;

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

pub async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let mut receiver = state.broadcaster.subscribe();

    while let Ok(event) = receiver.recv().await {
        let payload = serde_json::to_string(&event).unwrap();

        if socket.send(Message::Text(payload.into())).await.is_err() {
            break;
        }
    }
}
