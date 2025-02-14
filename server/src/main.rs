mod retroboard;
use crate::retroboard::RetroBoard;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Action {
    AddLane { title: String },
    AddItem { lane_id: String, body: String },
    RemoveItem { lane_id: String, id: String },
    UpvoteItem { lane_id: String, id: String },
}

struct AppState {
    board: RwLock<RetroBoard>,
    tx: broadcast::Sender<String>,
}

impl AppState {
    fn process_action(&self, action: Action) {
        match action {
            Action::AddLane { title } => {
                println!("Adding lane: {}", title);
                let mut board = self.board.write().unwrap();
                board.add_lane(&title);
                board.save_to_file("./retroboard.json");
            }
            Action::AddItem { lane_id, body } => {
                println!("Adding item to lane {}: {}", lane_id, body);
                let mut board = self.board.write().unwrap();
                board.add_item(&lane_id, &body);
                board.save_to_file("./retroboard.json");
            }
            Action::RemoveItem { lane_id, id } => {
                println!("Removing item from lane {}: {}", lane_id, id);
                let mut board = self.board.write().unwrap();
                board.remove_item(&lane_id, &id);
                board.save_to_file("./retroboard.json");
            }
            Action::UpvoteItem { lane_id, id } => {
                println!("Upvoting item in lane {}: {}", lane_id, id);
                let mut board = self.board.write().unwrap();
                board.upvote_item(&lane_id, &id);
                board.save_to_file("./retroboard.json");
            }
        }
    }
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = stream.split();

    let mut rx = state.tx.subscribe();

    tracing::debug!("New client connected");
    let board = {
        let board = state.board.read().unwrap();
        serde_json::to_string(&*board).unwrap()
    };
    sender.send(Message::text(board)).await.unwrap();

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
            let action_result: Result<Action, serde_json::Error> = serde_json::from_str(&text);
            let action = match action_result {
                Ok(action) => action,
                Err(e) => {
                    tracing::error!("Failed to parse action: {:?}", e);
                    continue;
                }
            };

            let board = {
                state.process_action(action);
                let board = state.board.read().unwrap();
                serde_json::to_string(&*board).unwrap()
            };

            let broadcast_result = tx.send(board);
            match broadcast_result {
                Ok(_) => (),
                Err(e) => {
                    tracing::error!("Failed to broadcast message: {:?}", e);
                    break;
                }
            }
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
    let board = RetroBoard::load_from_file("./retroboard.json");
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState {
        board: RwLock::new(board),
        tx,
    });

    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    tracing::debug!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
