mod retroboard;
use crate::retroboard::RetroBoard;

use axum::{
    extract::{
        ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;
use serde::{Serialize, Deserialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Action {
    AddItem { lane_id: String, body: String },
    RemoveItem { lane_id: String, id: String },
    UpvoteItem { lane_id: String, id: String },
}

struct AppState {
    board: RwLock<RetroBoard>,
    tx: broadcast::Sender<String>,
}

impl AppState {
    fn process_action(mut self, action: Action) -> AppState {
        match action {
            Action::AddItem { lane_id, body } => {
                let board = self.board.get_mut().unwrap();
                board.add_item(&lane_id, &body);
                self
            }
            Action::RemoveItem { lane_id, id } => {
                let board = self.board.get_mut().unwrap();
                board.remove_item(&lane_id, &id);
                self
            }
            Action::UpvoteItem { lane_id, id } => {
                let board = self.board.get_mut().unwrap();
                board.upvote_item(&lane_id, &id);
                self
            }
        }
    }
}

async fn websocket_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = stream.split();

    let mut rx = state.tx.subscribe();

    tracing::debug!("New client connected");

    sender.send(Message::text("Welcome to the retro board!")).await.unwrap();

    // Spawn the first task that will receive broadcast messages and send messages over the websocket to our client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass (move) to the receiving task
    let tx = state.tx.clone();

    // Spawn a task that takes messages from the websocket,
    // and sends them to all broadcast subscribers
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let _ = tx.send(text.to_string());
        }
    });

    // If any one of the tasks runs to completion, we abort the other.
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    tracing::debug!("Client disconnected");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Setup app state
    let board = RetroBoard::default();
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState {
        board: RwLock::new(board),
        tx,
    });

    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();

    tracing::debug!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
