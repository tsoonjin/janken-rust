use shared::GameState as SharedGameState;
use std::collections::HashMap;
use uuid::Uuid;

pub struct GameState {
    pub rooms: HashMap<Uuid, SharedGameState>,
    pub players: HashMap<Uuid, String>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            players: HashMap::new(),
        }
    }
}