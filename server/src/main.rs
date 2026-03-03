mod retroboard;
mod tofile;
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
    AddLane {
        title: String,
    },
    AddItem {
        lane_id: String,
        body: String,
    },
    RemoveItem {
        lane_id: String,
        id: String,
    },
    UpvoteItem {
        lane_id: String,
        id: String,
    },
    MoveItem {
        from_lane_id: String,
        to_lane_id: String,
        item_id: String,
    },
    ReorderItem {
        lane_id: String,
        item_id: String,
        new_position: u64,
    },
    EditItem {
        lane_id: String,
        id: String,
        body: String,
    },
    MergeItems {
        lane_id: String,
        source_id: String,
        target_id: String,
        merged_body: String,
    },
}

struct AppState {
    board: RwLock<RetroBoard>,
    tx: broadcast::Sender<String>,
}

const BOARD_FILE: &str = "./retroboard.json";

impl AppState {
    /// Acquire a read lock on the board, recovering from a poisoned lock.
    fn read_board(&self) -> std::sync::RwLockReadGuard<'_, RetroBoard> {
        match self.board.read() {
            Ok(guard) => guard,
            Err(poisoned) => {
                tracing::error!("Board RwLock was poisoned — recovering");
                poisoned.into_inner()
            }
        }
    }

    /// Acquire a write lock on the board, recovering from a poisoned lock.
    fn write_board(&self) -> std::sync::RwLockWriteGuard<'_, RetroBoard> {
        match self.board.write() {
            Ok(guard) => guard,
            Err(poisoned) => {
                tracing::error!("Board RwLock was poisoned — recovering");
                poisoned.into_inner()
            }
        }
    }

    fn process_action(&self, action: Action) {
        match action {
            Action::AddLane { title } => {
                tracing::debug!("Adding lane: {}", title);
                let mut board = self.write_board();
                board.add_lane(&title);
                board.save_to_file(BOARD_FILE);
            }
            Action::AddItem { lane_id, body } => {
                tracing::debug!("Adding item to lane {}: {}", lane_id, body);
                let mut board = self.write_board();
                board.add_item(&lane_id, &body);
                board.save_to_file(BOARD_FILE);
            }
            Action::RemoveItem { lane_id, id } => {
                tracing::debug!("Removing item from lane {}: {}", lane_id, id);
                let mut board = self.write_board();
                board.remove_item(&lane_id, &id);
                board.save_to_file(BOARD_FILE);
            }
            Action::UpvoteItem { lane_id, id } => {
                tracing::debug!("Upvoting item in lane {}: {}", lane_id, id);
                let mut board = self.write_board();
                board.upvote_item(&lane_id, &id);
                board.save_to_file(BOARD_FILE);
            }
            Action::MoveItem {
                from_lane_id,
                to_lane_id,
                item_id,
            } => {
                tracing::debug!(
                    "Moving item {} from lane {} to lane {}",
                    item_id,
                    from_lane_id,
                    to_lane_id
                );
                let mut board = self.write_board();
                board.move_item(&from_lane_id, &to_lane_id, &item_id);
                board.save_to_file(BOARD_FILE);
            }
            Action::ReorderItem {
                lane_id,
                item_id,
                new_position,
            } => {
                tracing::debug!(
                    "Reordering item {} in lane {} to position {}",
                    item_id,
                    lane_id,
                    new_position
                );
                let mut board = self.write_board();
                board.reorder_item(&lane_id, &item_id, new_position);
                board.save_to_file(BOARD_FILE);
            }
            Action::EditItem { lane_id, id, body } => {
                tracing::debug!("Editing item {} in lane {}: {}", id, lane_id, body);
                let mut board = self.write_board();
                board.edit_item(&lane_id, &id, &body);
                board.save_to_file(BOARD_FILE);
            }
            Action::MergeItems {
                lane_id,
                source_id,
                target_id,
                merged_body,
            } => {
                tracing::debug!(
                    "Merging item {} into {} in lane {}",
                    source_id,
                    target_id,
                    lane_id
                );
                let mut board = self.write_board();
                board.merge_items(&lane_id, &source_id, &target_id, &merged_body);
                board.save_to_file(BOARD_FILE);
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
        let board = state.read_board();
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
                let board = state.read_board();
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_action_serialization() {
        // Test AddLane action
        let add_lane = Action::AddLane {
            title: "Test Lane".to_string(),
        };
        let json = serde_json::to_string(&add_lane).unwrap();
        let parsed: Action = serde_json::from_str(&json).unwrap();
        match parsed {
            Action::AddLane { title } => assert_eq!(title, "Test Lane"),
            _ => panic!("Wrong action type"),
        }

        // Test AddItem action
        let add_item = Action::AddItem {
            lane_id: "lane1".to_string(),
            body: "Test item".to_string(),
        };
        let json = serde_json::to_string(&add_item).unwrap();
        let parsed: Action = serde_json::from_str(&json).unwrap();
        match parsed {
            Action::AddItem { lane_id, body } => {
                assert_eq!(lane_id, "lane1");
                assert_eq!(body, "Test item");
            }
            _ => panic!("Wrong action type"),
        }

        // Test RemoveItem action
        let remove_item = Action::RemoveItem {
            lane_id: "lane1".to_string(),
            id: "item1".to_string(),
        };
        let json = serde_json::to_string(&remove_item).unwrap();
        let parsed: Action = serde_json::from_str(&json).unwrap();
        match parsed {
            Action::RemoveItem { lane_id, id } => {
                assert_eq!(lane_id, "lane1");
                assert_eq!(id, "item1");
            }
            _ => panic!("Wrong action type"),
        }

        // Test UpvoteItem action
        let upvote_item = Action::UpvoteItem {
            lane_id: "lane1".to_string(),
            id: "item1".to_string(),
        };
        let json = serde_json::to_string(&upvote_item).unwrap();
        let parsed: Action = serde_json::from_str(&json).unwrap();
        match parsed {
            Action::UpvoteItem { lane_id, id } => {
                assert_eq!(lane_id, "lane1");
                assert_eq!(id, "item1");
            }
            _ => panic!("Wrong action type"),
        }

        // Test MoveItem action
        let move_item = Action::MoveItem {
            from_lane_id: "lane1".to_string(),
            to_lane_id: "lane2".to_string(),
            item_id: "item1".to_string(),
        };
        let json = serde_json::to_string(&move_item).unwrap();
        let parsed: Action = serde_json::from_str(&json).unwrap();
        match parsed {
            Action::MoveItem {
                from_lane_id,
                to_lane_id,
                item_id,
            } => {
                assert_eq!(from_lane_id, "lane1");
                assert_eq!(to_lane_id, "lane2");
                assert_eq!(item_id, "item1");
            }
            _ => panic!("Wrong action type"),
        }

        // Test ReorderItem action
        let reorder_item = Action::ReorderItem {
            lane_id: "lane1".to_string(),
            item_id: "item1".to_string(),
            new_position: 5,
        };
        let json = serde_json::to_string(&reorder_item).unwrap();
        let parsed: Action = serde_json::from_str(&json).unwrap();
        match parsed {
            Action::ReorderItem {
                lane_id,
                item_id,
                new_position,
            } => {
                assert_eq!(lane_id, "lane1");
                assert_eq!(item_id, "item1");
                assert_eq!(new_position, 5);
            }
            _ => panic!("Wrong action type"),
        }

        // Test EditItem action
        let edit_item = Action::EditItem {
            lane_id: "lane1".to_string(),
            id: "item1".to_string(),
            body: "Updated body".to_string(),
        };
        let json = serde_json::to_string(&edit_item).unwrap();
        let parsed: Action = serde_json::from_str(&json).unwrap();
        match parsed {
            Action::EditItem { lane_id, id, body } => {
                assert_eq!(lane_id, "lane1");
                assert_eq!(id, "item1");
                assert_eq!(body, "Updated body");
            }
            _ => panic!("Wrong action type"),
        }

        // Test MergeItems action
        let merge_items = Action::MergeItems {
            lane_id: "lane1".to_string(),
            source_id: "item1".to_string(),
            target_id: "item2".to_string(),
            merged_body: "Merged text".to_string(),
        };
        let json = serde_json::to_string(&merge_items).unwrap();
        let parsed: Action = serde_json::from_str(&json).unwrap();
        match parsed {
            Action::MergeItems {
                lane_id,
                source_id,
                target_id,
                merged_body,
            } => {
                assert_eq!(lane_id, "lane1");
                assert_eq!(source_id, "item1");
                assert_eq!(target_id, "item2");
                assert_eq!(merged_body, "Merged text");
            }
            _ => panic!("Wrong action type"),
        }
    }

    #[test]
    fn test_invalid_action_json() {
        let invalid_json = r#"{"type": "InvalidAction"}"#;
        let result: Result<Action, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_app_state_process_action() {
        use std::sync::RwLock;
        use tokio::sync::broadcast;

        let board = RetroBoard::default();
        let (tx, _rx) = broadcast::channel(100);
        let app_state = AppState {
            board: RwLock::new(board),
            tx,
        };

        // Test AddLane action
        let action = Action::AddLane {
            title: "New Lane".to_string(),
        };
        app_state.process_action(action);

        let board = app_state.board.read().unwrap();
        assert!(board.lanes.contains_key("New Lane"));

        drop(board); // Release the read lock

        // Test AddItem action
        let action = Action::AddItem {
            lane_id: "New Lane".to_string(),
            body: "Test Item".to_string(),
        };
        app_state.process_action(action);

        let board = app_state.board.read().unwrap();
        let lane = board.lanes.get("New Lane").unwrap();
        assert_eq!(lane.items.len(), 1);
        let item = lane.items.values().next().unwrap();
        assert_eq!(item.body, "Test Item");

        drop(board); // Release the read lock

        // Test RemoveItem action
        let board = app_state.board.read().unwrap();
        let lane = board.lanes.get("New Lane").unwrap();
        let item_id = lane.items.keys().next().unwrap().clone();
        drop(board); // Release the read lock

        let action = Action::RemoveItem {
            lane_id: "New Lane".to_string(),
            id: item_id,
        };
        app_state.process_action(action);

        let board = app_state.board.read().unwrap();
        let lane = board.lanes.get("New Lane").unwrap();
        assert_eq!(lane.items.len(), 0);

        drop(board); // Release the read lock

        // Test UpvoteItem action with default board
        let action = Action::UpvoteItem {
            lane_id: "went-well".to_string(),
            id: "1".to_string(),
        };
        app_state.process_action(action);

        let board = app_state.board.read().unwrap();
        let lane = board.lanes.get("went-well").unwrap();
        let item = lane.items.get("1").unwrap();
        assert_eq!(item.vote_count, 1);

        drop(board); // Release the read lock

        // Test MoveItem action
        let action = Action::MoveItem {
            from_lane_id: "went-well".to_string(),
            to_lane_id: "to-improve".to_string(),
            item_id: "1".to_string(),
        };
        app_state.process_action(action);

        let board = app_state.board.read().unwrap();
        let went_well_lane = board.lanes.get("went-well").unwrap();
        let to_improve_lane = board.lanes.get("to-improve").unwrap();
        assert!(!went_well_lane.items.contains_key("1"));
        assert!(to_improve_lane.items.contains_key("1"));

        drop(board); // Release the read lock

        // Test ReorderItem action
        let action = Action::ReorderItem {
            lane_id: "to-improve".to_string(),
            item_id: "1".to_string(),
            new_position: 0,
        };
        app_state.process_action(action);

        let board = app_state.board.read().unwrap();
        let lane = board.lanes.get("to-improve").unwrap();
        let item = lane.items.get("1").unwrap();
        assert_eq!(item.sort_order, 0);

        drop(board);

        // Test EditItem action
        let action = Action::EditItem {
            lane_id: "to-improve".to_string(),
            id: "3".to_string(),
            body: "Edited body text".to_string(),
        };
        app_state.process_action(action);

        let board = app_state.board.read().unwrap();
        let lane = board.lanes.get("to-improve").unwrap();
        let item = lane.items.get("3").unwrap();
        assert_eq!(item.body, "Edited body text");

        drop(board);

        // Test MergeItems action
        let action = Action::MergeItems {
            lane_id: "to-improve".to_string(),
            source_id: "1".to_string(),
            target_id: "3".to_string(),
            merged_body: "Combined text".to_string(),
        };
        app_state.process_action(action);

        let board = app_state.board.read().unwrap();
        let lane = board.lanes.get("to-improve").unwrap();
        assert!(!lane.items.contains_key("1")); // source removed
        let target = lane.items.get("3").unwrap();
        assert_eq!(target.body, "Combined text");
        assert_eq!(target.vote_count, 1); // inherited from source (upvoted earlier)
    }
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
    let board = RetroBoard::load_from_file(BOARD_FILE);
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState {
        board: RwLock::new(board),
        tx,
    });

    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::debug!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
