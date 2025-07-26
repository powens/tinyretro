use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RetroItem {
    body: String,
    vote_count: u64,
    sort_order: u64,
}

impl RetroItem {
    fn increment_vote(&mut self) {
        self.vote_count += 1;
    }
}

#[derive(Serialize, Deserialize)]
pub struct RetroLane {
    title: String,
    theme: String,
    items: HashMap<String, RetroItem>,
}

impl RetroLane {
    fn add_item(&mut self, body: &str) {
        let id = Uuid::new_v4().to_string();
        self.items.insert(
            id,
            RetroItem {
                body: body.to_owned(),
                vote_count: 0,
                sort_order: self.items.len() as u64,
            },
        );
    }

    fn remove_item(&mut self, id: &str) {
        self.items.remove(id);
    }

    fn increment_vote(&mut self, id: &str) {
        self.items.get_mut(id).unwrap().increment_vote();
    }
}

#[derive(Serialize, Deserialize)]
pub struct RetroBoard {
    pub title: String,
    pub lanes: HashMap<String, RetroLane>,
}

impl RetroBoard {
    pub fn default() -> Self {
        Self {
            title: "My Retro Board".to_string(),
            lanes: HashMap::from([
                (
                    "went-well".to_string(),
                    RetroLane {
                        title: "Went Well".to_string(),
                        theme: "went-well".to_string(),
                        items: HashMap::from([
                            (
                                "1".to_string(),
                                RetroItem {
                                    body: "We shipped the feature on time".to_string(),
                                    vote_count: 0,
                                    sort_order: 0,
                                },
                            ),
                            (
                                "2".to_string(),
                                RetroItem {
                                    body: "The team worked well together".to_string(),
                                    vote_count: 0,
                                    sort_order: 1,
                                },
                            ),
                        ]),
                    },
                ),
                (
                    "to-improve".to_string(),
                    RetroLane {
                        title: "To Improve".to_string(),
                        theme: "to-improve".to_string(),
                        items: HashMap::from([
                            (
                                "3".to_string(),
                                RetroItem {
                                    body: "We need to improve our testing".to_string(),
                                    vote_count: 0,
                                    sort_order: 0,
                                },
                            ),
                            (
                                "4".to_string(),
                                RetroItem {
                                    body: "We need to improve our communication".to_string(),
                                    vote_count: 0,
                                    sort_order: 1,
                                },
                            ),
                        ]),
                    },
                ),
                (
                    "action-items".to_string(),
                    RetroLane {
                        title: "Action Items".to_string(),
                        theme: "action-items".to_string(),
                        items: HashMap::from([
                            (
                                "5".to_string(),
                                RetroItem {
                                    body: "Write more tests".to_string(),
                                    vote_count: 0,
                                    sort_order: 0,
                                },
                            ),
                            (
                                "6".to_string(),
                                RetroItem {
                                    body: "Schedule a team-building event".to_string(),
                                    vote_count: 0,
                                    sort_order: 1,
                                },
                            ),
                        ]),
                    },
                ),
            ]),
        }
    }

    pub fn save_to_file(&self, path: &str) {
        let file = File::create(path).unwrap();
        serde_json::to_writer(file, self).unwrap();
    }

    pub fn load_from_file(path: &str) -> Self {
        let exists = std::path::Path::new(path).exists();
        if exists {
            let file = match File::open(path) {
                Ok(file) => file,
                Err(e) => {
                    tracing::error!("Error opening file, returning default RetroBoard: {:?}", e);
                    return Self::default();
                }
            };
            match serde_json::from_reader(file) {
                Ok(board) => board,
                Err(e) => {
                    tracing::error!("Error reading file, returning default RetroBoard: {:?}", e);
                    Self::default()
                }
            }
        } else {
            Self::default()
        }
    }

    pub fn add_lane(&mut self, title: &str) {
        let title_string = title.to_string();
        self.lanes.insert(
            title_string.clone(),
            RetroLane {
                title: title_string,
                theme: self.title.clone(),
                items: HashMap::new(),
            },
        );
    }

    pub fn add_item(&mut self, lane_id: &str, body: &str) {
        self.lanes.get_mut(lane_id).unwrap().add_item(body);
    }

    pub fn remove_item(&mut self, lane_id: &str, id: &str) {
        self.lanes.get_mut(lane_id).unwrap().remove_item(id);
    }

    pub fn upvote_item(&mut self, lane_id: &str, id: &str) {
        self.lanes.get_mut(lane_id).unwrap().increment_vote(id);
    }

    pub fn move_item(&mut self, from_lane_id: &str, to_lane_id: &str, item_id: &str) {
        if from_lane_id == to_lane_id {
            return; // No need to move if it's the same lane
        }

        // Remove the item from the source lane
        let item = {
            let from_lane = self.lanes.get_mut(from_lane_id).unwrap();
            from_lane.items.remove(item_id)
        };

        // Add the item to the destination lane if it was found
        if let Some(item) = item {
            let to_lane = self.lanes.get_mut(to_lane_id).unwrap();
            to_lane.items.insert(item_id.to_string(), item);
        }
    }

    pub fn reorder_item(&mut self, lane_id: &str, item_id: &str, new_position: u64) {
        tracing::debug!(
            "DEBUG: reorder_item called - lane: {}, item: {}, position: {}",
            lane_id,
            item_id,
            new_position
        );

        let lane = match self.lanes.get_mut(lane_id) {
            Some(lane) => lane,
            None => {
                tracing::error!("Lane with ID '{}' not found", lane_id);
                return Err(format!("Lane with ID '{}' not found", lane_id));
            }
        };

        // Get all items and sort by current sort_order
        let mut items: Vec<(String, RetroItem)> = lane.items.drain().collect();
        tracing::debug!("DEBUG: Found {} items in lane", items.len());

        items.sort_by_key(|(_, item)| item.sort_order);

        // Find the item to move
        let item_index = items.iter().position(|(id, _)| id == item_id);
        tracing::debug!("DEBUG: Item index: {:?}", item_index);

        if let Some(old_index) = item_index {
            let (item_id, item) = items.remove(old_index);
            tracing::debug!("DEBUG: Removed item from position {}", old_index);

            // Insert at new position (clamped to valid range)
            let new_index = (new_position as usize).min(items.len());
            tracing::debug!("DEBUG: Inserting at position {}", new_index);
            items.insert(new_index, (item_id, item));

            // Reassign sort_order values
            for (i, (_, item)) in items.iter_mut().enumerate() {
                item.sort_order = i as u64;
                tracing::debug!("DEBUG: Set item {} sort_order to {}", i, item.sort_order);
            }

            // Put items back into the lane
            lane.items = items.into_iter().collect();
            tracing::debug!("DEBUG: Reordering complete");
        } else {
            tracing::debug!("DEBUG: Item not found in lane");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_add_item() {
        let mut lane = RetroLane {
            title: "Test Lane".to_string(),
            theme: "Test Theme".to_string(),
            items: HashMap::new(),
        };

        lane.add_item("Test Item");
        assert_eq!(lane.items.len(), 1);
        let item = lane.items.values().next().unwrap();
        assert_eq!(item.body, "Test Item");
        assert_eq!(item.vote_count, 0);
        assert_eq!(item.sort_order, 0);
    }

    #[test]
    fn test_remove_item() {
        let mut lane = RetroLane {
            title: "Test Lane".to_string(),
            theme: "Test Theme".to_string(),
            items: HashMap::new(),
        };

        lane.add_item("Test Item");
        let item_id = lane.items.keys().next().unwrap().clone();
        lane.remove_item(&item_id);
        assert_eq!(lane.items.len(), 0);
    }

    #[test]
    fn test_increment_vote() {
        let mut lane = RetroLane {
            title: "Test Lane".to_string(),
            theme: "Test Theme".to_string(),
            items: HashMap::new(),
        };

        lane.add_item("Test Item");
        let item_id = lane.items.keys().next().unwrap().clone();
        lane.increment_vote(&item_id);
        let item = lane.items.get(&item_id).unwrap();
        assert_eq!(item.vote_count, 1);
    }

    #[test]
    fn test_add_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Test Lane");
        assert_eq!(board.lanes.len(), 1);
        let lane = board.lanes.values().next().unwrap();
        assert_eq!(lane.title, "Test Lane");
        assert_eq!(lane.theme, "Test Board");
    }

    #[test]
    fn test_add_item_to_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Test Lane");
        board.add_item("Test Lane", "Test Item");
        let lane = board.lanes.get("Test Lane").unwrap();
        assert_eq!(lane.items.len(), 1);
        let item = lane.items.values().next().unwrap();
        assert_eq!(item.body, "Test Item");
    }

    #[test]
    fn test_remove_item_from_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Test Lane");
        board.add_item("Test Lane", "Test Item");
        let lane = board.lanes.get("Test Lane").unwrap();
        let item_id = lane.items.keys().next().unwrap().clone();
        board.remove_item("Test Lane", &item_id);
        let lane = board.lanes.get("Test Lane").unwrap();
        assert_eq!(lane.items.len(), 0);
    }

    #[test]
    fn test_upvote_item_in_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Test Lane");
        board.add_item("Test Lane", "Test Item");
        let lane = board.lanes.get("Test Lane").unwrap();
        let item_id = lane.items.keys().next().unwrap().clone();
        board.upvote_item("Test Lane", &item_id);
        let lane = board.lanes.get("Test Lane").unwrap();
        let item = lane.items.get(&item_id).unwrap();
        assert_eq!(item.vote_count, 1);
    }

    #[test]
    fn test_move_item_between_lanes() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Lane 1");
        board.add_lane("Lane 2");
        board.add_item("Lane 1", "Test Item");

        let lane1 = board.lanes.get("Lane 1").unwrap();
        let item_id = lane1.items.keys().next().unwrap().clone();

        board.move_item("Lane 1", "Lane 2", &item_id);

        let lane1 = board.lanes.get("Lane 1").unwrap();
        let lane2 = board.lanes.get("Lane 2").unwrap();

        assert_eq!(lane1.items.len(), 0);
        assert_eq!(lane2.items.len(), 1);
        assert!(lane2.items.contains_key(&item_id));
    }

    #[test]
    fn test_reorder_item_within_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Lane 1");

        // Add three items
        board.add_item("Lane 1", "First item");
        board.add_item("Lane 1", "Second item");
        board.add_item("Lane 1", "Third item");

        let lane = board.lanes.get("Lane 1").unwrap();

        // Get items by their sort order
        let mut sorted_items: Vec<(String, u64)> = lane
            .items
            .iter()
            .map(|(id, item)| (id.clone(), item.sort_order))
            .collect();
        sorted_items.sort_by_key(|(_, order)| *order);

        let (item1_id, _) = &sorted_items[0];
        let (item2_id, _) = &sorted_items[1];
        let (item3_id, _) = &sorted_items[2];

        // Move the third item to the first position
        board.reorder_item("Lane 1", item3_id, 0);

        // Verify new order
        let lane = board.lanes.get("Lane 1").unwrap();
        assert_eq!(lane.items.get(item3_id).unwrap().sort_order, 0);
        assert_eq!(lane.items.get(item1_id).unwrap().sort_order, 1);
        assert_eq!(lane.items.get(item2_id).unwrap().sort_order, 2);
    }
}
