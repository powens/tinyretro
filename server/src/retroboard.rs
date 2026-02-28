use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RetroItem {
    pub body: String,
    pub vote_count: u64,
    pub sort_order: u64,
}

impl RetroItem {
    fn increment_vote(&mut self) {
        self.vote_count += 1;
    }
}

#[derive(Serialize, Deserialize)]
pub struct RetroLane {
    pub title: String,
    pub theme: String,
    pub items: HashMap<String, RetroItem>,
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
            if let Some(from_lane) = self.lanes.get_mut(from_lane_id) {
                from_lane.items.remove(item_id)
            } else {
                tracing::error!("Lane with ID '{}' does not exist", from_lane_id);
                return;
            }
        };

        // Add the item to the destination lane if it was found
        if let Some(mut item) = item {
            if let Some(to_lane) = self.lanes.get_mut(to_lane_id) {
                // Assign sort_order to the end of the destination lane
                item.sort_order = to_lane.items.len() as u64;
                to_lane.items.insert(item_id.to_string(), item);
            } else {
                tracing::error!("Lane with ID '{}' does not exist", to_lane_id);
            }
        } else {
            tracing::error!("Item with ID '{}' not found in lane '{}'", item_id, from_lane_id);
        }
    }

    pub fn edit_item(&mut self, lane_id: &str, id: &str, body: &str) {
        if let Some(lane) = self.lanes.get_mut(lane_id) {
            if let Some(item) = lane.items.get_mut(id) {
                item.body = body.to_owned();
            } else {
                tracing::error!("Item with ID '{}' not found in lane '{}'", id, lane_id);
            }
        } else {
            tracing::error!("Lane with ID '{}' not found", lane_id);
        }
    }

    pub fn merge_items(
        &mut self,
        lane_id: &str,
        source_id: &str,
        target_id: &str,
        merged_body: &str,
    ) {
        if let Some(lane) = self.lanes.get_mut(lane_id) {
            // Verify both items exist before mutating anything
            if !lane.items.contains_key(source_id) {
                tracing::error!("Source item '{}' not found in lane '{}'", source_id, lane_id);
                return;
            }
            if !lane.items.contains_key(target_id) {
                tracing::error!(
                    "Target item '{}' not found in lane '{}'",
                    target_id,
                    lane_id
                );
                return;
            }

            let source_votes = lane.items.get(source_id).unwrap().vote_count;

            // Remove the source item (safe — we verified it exists above)
            lane.items.remove(source_id);

            // Update the target item (safe — we verified it exists above)
            let target = lane.items.get_mut(target_id).unwrap();
            target.body = merged_body.to_owned();
            target.vote_count += source_votes;
        } else {
            tracing::error!("Lane with ID '{}' not found", lane_id);
        }
    }

    pub fn reorder_item(&mut self, lane_id: &str, item_id: &str, new_position: u64) {
        tracing::debug!(
            "reorder_item called - lane: {}, item: {}, position: {}",
            lane_id,
            item_id,
            new_position
        );

        let lane = match self.lanes.get_mut(lane_id) {
            Some(lane) => lane,
            None => {
                tracing::error!("Lane with ID '{}' not found", lane_id);
                return;
            }
        };

        // Get all items and sort by current sort_order
        let mut items: Vec<(String, RetroItem)> = lane.items.drain().collect();
        tracing::debug!("Found {} items in lane", items.len());

        items.sort_by_key(|(_, item)| item.sort_order);

        // Find the item to move
        let item_index = items.iter().position(|(id, _)| id == item_id);
        tracing::debug!("Item index: {:?}", item_index);

        if let Some(old_index) = item_index {
            let (item_id, item) = items.remove(old_index);
            tracing::debug!("Removed item from position {}", old_index);

            // Insert at new position (clamped to valid range)
            let new_index = (new_position as usize).min(items.len());
            tracing::debug!("Inserting at position {}", new_index);
            items.insert(new_index, (item_id, item));

            // Reassign sort_order values
            for (i, (_, item)) in items.iter_mut().enumerate() {
                item.sort_order = i as u64;
                tracing::debug!("Set item {} sort_order to {}", i, item.sort_order);
            }

            tracing::debug!("Reordering complete");
        } else {
            tracing::debug!("Item not found in lane");
        }

        // Always put items back into the lane, whether found or not
        lane.items = items.into_iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::tempdir;

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
    fn test_add_multiple_items_sort_order() {
        let mut lane = RetroLane {
            title: "Test Lane".to_string(),
            theme: "Test Theme".to_string(),
            items: HashMap::new(),
        };

        lane.add_item("First Item");
        lane.add_item("Second Item");
        lane.add_item("Third Item");
        
        assert_eq!(lane.items.len(), 3);
        
        // Verify sort orders are correct
        let mut items: Vec<_> = lane.items.values().collect();
        items.sort_by_key(|item| item.sort_order);
        
        assert_eq!(items[0].sort_order, 0);
        assert_eq!(items[1].sort_order, 1);
        assert_eq!(items[2].sort_order, 2);
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
    fn test_remove_nonexistent_item() {
        let mut lane = RetroLane {
            title: "Test Lane".to_string(),
            theme: "Test Theme".to_string(),
            items: HashMap::new(),
        };

        // Try to remove an item that doesn't exist
        lane.remove_item("nonexistent");
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
    #[should_panic]
    fn test_increment_vote_nonexistent_item() {
        let mut lane = RetroLane {
            title: "Test Lane".to_string(),
            theme: "Test Theme".to_string(),
            items: HashMap::new(),
        };

        // This should panic because the item doesn't exist
        lane.increment_vote("nonexistent");
    }

    #[test]
    fn test_retro_item_increment_vote() {
        let mut item = RetroItem {
            body: "Test".to_string(),
            vote_count: 0,
            sort_order: 0,
        };

        item.increment_vote();
        assert_eq!(item.vote_count, 1);

        item.increment_vote();
        assert_eq!(item.vote_count, 2);
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
        assert_eq!(lane.items.len(), 0);
    }

    #[test]
    fn test_add_duplicate_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Test Lane");
        board.add_lane("Test Lane"); // Add same lane again
        assert_eq!(board.lanes.len(), 1); // Should still be 1
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
    #[should_panic]
    fn test_add_item_to_nonexistent_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        // This should panic because the lane doesn't exist
        board.add_item("Nonexistent Lane", "Test Item");
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
    #[should_panic]
    fn test_remove_item_from_nonexistent_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        // This should panic because the lane doesn't exist
        board.remove_item("Nonexistent Lane", "item_id");
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
    #[should_panic]
    fn test_upvote_item_in_nonexistent_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        // This should panic because the lane doesn't exist
        board.upvote_item("Nonexistent Lane", "item_id");
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
    fn test_move_item_same_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Lane 1");
        board.add_item("Lane 1", "Test Item");

        let lane1 = board.lanes.get("Lane 1").unwrap();
        let item_id = lane1.items.keys().next().unwrap().clone();

        // Moving to the same lane should do nothing
        board.move_item("Lane 1", "Lane 1", &item_id);

        let lane1 = board.lanes.get("Lane 1").unwrap();
        assert_eq!(lane1.items.len(), 1);
        assert!(lane1.items.contains_key(&item_id));
    }

    #[test]
    fn test_move_item_from_nonexistent_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Lane 2");

        // Try to move from nonexistent lane - should do nothing
        board.move_item("Nonexistent Lane", "Lane 2", "item_id");

        let lane2 = board.lanes.get("Lane 2").unwrap();
        assert_eq!(lane2.items.len(), 0);
    }

    #[test]
    fn test_move_item_to_nonexistent_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Lane 1");
        board.add_item("Lane 1", "Test Item");

        let lane1 = board.lanes.get("Lane 1").unwrap();
        let item_id = lane1.items.keys().next().unwrap().clone();

        // Try to move to nonexistent lane - item should be removed from source
        board.move_item("Lane 1", "Nonexistent Lane", &item_id);

        let lane1 = board.lanes.get("Lane 1").unwrap();
        assert_eq!(lane1.items.len(), 0);
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

    #[test]
    fn test_reorder_item_in_nonexistent_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        // Try to reorder item in nonexistent lane - should do nothing
        board.reorder_item("Nonexistent Lane", "item_id", 0);
        // No panic expected, just graceful handling
    }

    #[test]
    fn test_reorder_nonexistent_item() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Lane 1");
        board.add_item("Lane 1", "Test Item");

        // Try to reorder nonexistent item - should do nothing
        board.reorder_item("Lane 1", "nonexistent_item", 0);

        let lane = board.lanes.get("Lane 1").unwrap();
        assert_eq!(lane.items.len(), 1);
    }

    #[test]
    fn test_reorder_item_beyond_bounds() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Lane 1");
        board.add_item("Lane 1", "First item");
        board.add_item("Lane 1", "Second item");

        let lane = board.lanes.get("Lane 1").unwrap();
        let item_id = lane.items.keys().next().unwrap().clone();

        // Try to move to position beyond the number of items
        board.reorder_item("Lane 1", &item_id, 10);

        let lane = board.lanes.get("Lane 1").unwrap();
        let item = lane.items.get(&item_id).unwrap();
        // Should be clamped to the end of the list
        assert_eq!(item.sort_order, 1);
    }

    #[test]
    fn test_retroboard_default() {
        let board = RetroBoard::default();
        
        assert_eq!(board.title, "My Retro Board");
        assert_eq!(board.lanes.len(), 3);
        
        assert!(board.lanes.contains_key("went-well"));
        assert!(board.lanes.contains_key("to-improve"));
        assert!(board.lanes.contains_key("action-items"));
        
        let went_well = board.lanes.get("went-well").unwrap();
        assert_eq!(went_well.title, "Went Well");
        assert_eq!(went_well.items.len(), 2);
        
        let to_improve = board.lanes.get("to-improve").unwrap();
        assert_eq!(to_improve.title, "To Improve");
        assert_eq!(to_improve.items.len(), 2);
        
        let action_items = board.lanes.get("action-items").unwrap();
        assert_eq!(action_items.title, "Action Items");
        assert_eq!(action_items.items.len(), 2);
    }

    #[test]
    fn test_save_and_load_from_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_board.json");
        let file_path_str = file_path.to_str().unwrap();

        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };
        board.add_lane("Test Lane");
        board.add_item("Test Lane", "Test Item");

        // Test save
        board.save_to_file(file_path_str);

        // Test load
        let loaded_board = RetroBoard::load_from_file(file_path_str);
        assert_eq!(loaded_board.title, "Test Board");
        assert_eq!(loaded_board.lanes.len(), 1);
        assert!(loaded_board.lanes.contains_key("Test Lane"));
        
        let lane = loaded_board.lanes.get("Test Lane").unwrap();
        assert_eq!(lane.items.len(), 1);
        
        let item = lane.items.values().next().unwrap();
        assert_eq!(item.body, "Test Item");
    }

    #[test]
    fn test_load_from_nonexistent_file() {
        let board = RetroBoard::load_from_file("/nonexistent/path.json");
        
        // Should return default board when file doesn't exist
        assert_eq!(board.title, "My Retro Board");
        assert_eq!(board.lanes.len(), 3);
    }

    #[test]
    fn test_load_from_invalid_json() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("invalid.json");
        let file_path_str = file_path.to_str().unwrap();

        // Create a file with invalid JSON
        std::fs::write(file_path_str, "invalid json content").unwrap();

        let board = RetroBoard::load_from_file(file_path_str);
        
        // Should return default board when JSON is invalid
        assert_eq!(board.title, "My Retro Board");
        assert_eq!(board.lanes.len(), 3);
    }

    #[test]
    fn test_edit_item() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Test Lane");
        board.add_item("Test Lane", "Original body");

        let lane = board.lanes.get("Test Lane").unwrap();
        let item_id = lane.items.keys().next().unwrap().clone();

        board.edit_item("Test Lane", &item_id, "Updated body");

        let lane = board.lanes.get("Test Lane").unwrap();
        let item = lane.items.get(&item_id).unwrap();
        assert_eq!(item.body, "Updated body");
    }

    #[test]
    fn test_edit_item_nonexistent_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        // Should not panic, just log error
        board.edit_item("Nonexistent", "id", "body");
    }

    #[test]
    fn test_edit_item_nonexistent_item() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Test Lane");

        // Should not panic, just log error
        board.edit_item("Test Lane", "nonexistent", "body");
    }

    #[test]
    fn test_merge_items() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Test Lane");
        board.add_item("Test Lane", "First item");
        board.add_item("Test Lane", "Second item");

        let lane = board.lanes.get("Test Lane").unwrap();
        let mut ids: Vec<String> = lane.items.keys().cloned().collect();
        ids.sort();
        let source_id = ids[0].clone();
        let target_id = ids[1].clone();

        // Upvote the source to verify vote transfer
        board.upvote_item("Test Lane", &source_id);
        board.upvote_item("Test Lane", &source_id);
        // Upvote the target
        board.upvote_item("Test Lane", &target_id);

        board.merge_items("Test Lane", &source_id, &target_id, "Merged body");

        let lane = board.lanes.get("Test Lane").unwrap();
        assert_eq!(lane.items.len(), 1);
        assert!(!lane.items.contains_key(&source_id));

        let target = lane.items.get(&target_id).unwrap();
        assert_eq!(target.body, "Merged body");
        assert_eq!(target.vote_count, 3); // 2 from source + 1 from target
    }

    #[test]
    fn test_merge_items_nonexistent_lane() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        // Should not panic
        board.merge_items("Nonexistent", "s", "t", "body");
    }

    #[test]
    fn test_merge_items_nonexistent_target() {
        let mut board = RetroBoard {
            title: "Test Board".to_string(),
            lanes: HashMap::new(),
        };

        board.add_lane("Test Lane");
        board.add_item("Test Lane", "Item");

        let lane = board.lanes.get("Test Lane").unwrap();
        let source_id = lane.items.keys().next().unwrap().clone();

        // Merge with nonexistent target - source should NOT be removed
        board.merge_items("Test Lane", &source_id, "nonexistent", "body");

        let lane = board.lanes.get("Test Lane").unwrap();
        assert!(lane.items.contains_key(&source_id));
    }
}
