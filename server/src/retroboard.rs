use serde::{Serialize, Deserialize};
use std::collections::HashMap;
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
            title: "New Retro Board".to_string(),
            lanes: HashMap::from([
                ("Foo".to_string(), RetroLane{
                    title: "Foo".to_string(),
                    items: HashMap::new(),
                }),
            ]),
        }
    }

    pub fn add_lane(&mut self, title: &String) {
        self.lanes.insert(title.clone(), RetroLane{
            title: title.clone(),
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