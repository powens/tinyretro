use serde::{Serialize, Deserialize};
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
        self.items.insert(id, RetroItem{
            body: body.to_owned(),
            vote_count: 0,
            sort_order: self.items.len() as u64,
        });
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
                ("went-well".to_string(), RetroLane{
                    title: "Went Well".to_string(),
                    theme: "went-well".to_string(),
                    items: HashMap::from([
                        ("1".to_string(), RetroItem{
                            body: "We shipped the feature on time".to_string(),
                            vote_count: 0,
                            sort_order: 0,
                        }),
                        ("2".to_string(), RetroItem{
                            body: "The team worked well together".to_string(),
                            vote_count: 0,
                            sort_order: 1,
                        }),
                    ]),
                }),
                ("to-improve".to_string(), RetroLane{
                    title: "To Improve".to_string(),
                    theme: "to-improve".to_string(),
                    items: HashMap::from([
                        ("3".to_string(), RetroItem{
                            body: "We need to improve our testing".to_string(),
                            vote_count: 0,
                            sort_order: 0,
                        }),
                        ("4".to_string(), RetroItem{
                            body: "We need to improve our communication".to_string(),
                            vote_count: 0,
                            sort_order: 1,
                        }),
                    ]),
                }),
                ("action-items".to_string(), RetroLane{
                    title: "Action Items".to_string(),
                    theme: "action-items".to_string(),
                    items: HashMap::from([
                        ("5".to_string(), RetroItem{
                            body: "Write more tests".to_string(),
                            vote_count: 0,
                            sort_order: 0,
                        }),
                        ("6".to_string(), RetroItem{
                            body: "Schedule a team-building event".to_string(),
                            vote_count: 0,
                            sort_order: 1,
                        }),
                    ]),
                }),
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
                },
            };
            match serde_json::from_reader(file) {
                Ok(board) => board,
                Err(e) => {
                    tracing::error!("Error reading file, returning default RetroBoard: {:?}", e);
                    Self::default()
                },
            }
        }
        else {
            Self::default()
        }
    }

    pub fn add_lane(&mut self, title: &str) {
        let title_string = title.to_string();
        self.lanes.insert(title_string.clone(), RetroLane{
            title: title_string,
            theme: self.title.clone(),
            items: HashMap::new(),
        });
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
}