use shared::{GamePhase, PlayerState};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct GameState {
    pub current_room: Option<Uuid>,
    pub players: HashMap<Uuid, PlayerState>,
    pub phase: GamePhase,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_room: None,
            players: HashMap::new(),
            phase: GamePhase::Waiting,
        }
    }
}