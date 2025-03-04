use uuid::Uuid;
use shared::GameState as SharedGameState;

pub struct Room {
    pub id: Uuid,
    pub state: SharedGameState,
}

impl Room {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            state: SharedGameState::default(),
        }
    }
}