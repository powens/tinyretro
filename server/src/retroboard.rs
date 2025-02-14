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
        self.vote_count = self.vote_count + 1;
    }
}


#[derive(Serialize, Deserialize)]
pub struct RetroLane {
    title: String,
    theme: String,
    items: HashMap<String, RetroItem>,
}

impl RetroLane {
    fn add_item(&mut self, body: &String) {
        let id = Uuid::new_v4().to_string();
        self.items.insert(id, RetroItem{
            body: body.clone(),
            vote_count: 0,
            sort_order: self.items.len() as u64,
        });
    }

    fn remove_item(&mut self, id: &String) {
        self.items.remove(id);
    }

    fn increment_vote(&mut self, id: &String) {
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
        if exists == true {
            let file = match File::open(path) {
                Ok(file) => file,
                Err(_) => {
                    println!("Error opening file, returning default RetroBoard");
                    return Self::default();
                },
            };
            match serde_json::from_reader(file) {
                Ok(board) => return board,
                Err(_) => {
                    println!("Error reading file, returning default RetroBoard");
                    return Self::default();
                },
            };
        }
        else {
            return Self::default();
        }
    }

    pub fn add_lane(&mut self, title: &String) {
        self.lanes.insert(title.clone(), RetroLane{
            title: title.clone(),
            theme: self.title.clone(),
            items: HashMap::new(),
        });
    }

    pub fn add_item(&mut self, lane_id: &String, body: &String) {
        self.lanes.get_mut(lane_id).unwrap().add_item(body);
    }
    
    pub fn remove_item(&mut self, lane_id: &String, id: &String) {
        self.lanes.get_mut(lane_id).unwrap().remove_item(id);
    }
    
    pub fn upvote_item(&mut self, lane_id: &String, id: &String) {
        self.lanes.get_mut(lane_id).unwrap().increment_vote(id);
    }
}