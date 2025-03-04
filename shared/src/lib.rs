use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GamePhase {
    Waiting,
    ClassSelection,
    CardSelection,
    AbilityUsage,
    Resolution,
    Tiebreaker,
    Playing,
    Finished,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CardType {
    Rock,
    Paper,
    Scissors,
    Dragon,
    Rabbit,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CardStatus {
    InHand,
    Played,
    Removed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    pub card_type: CardType,
    pub status: CardStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CharacterClass {
    Royalguard,
    Swordmaster,
    Gunslinger,
    Trickster,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    SelectClass(CharacterClass),
    PlayCard(Card),
    UseAbility(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RoomStatus {
    Waiting,
    InProgress,
    Completed,
    Paused,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    CreateRoom {
        player_id: Uuid,
    },
    JoinRoom {
        room_id: Uuid,
        player_id: Uuid,
    },
    SelectClass {
        class_name: CharacterClass,
        room_id: Uuid,
        player_id: Uuid,
    },
    PlayCard {
        card_type: CardType,
        room_id: Uuid,
        player_id: Uuid,
        round_number: i32,
    },
    UseAbility {
        ability_name: String,
        room_id: Uuid,
        player_id: Uuid,
        target_player_id: Uuid,
    },
    QuickDrawAction {
        room_id: Uuid,
        player_id: Uuid,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerState {
    pub player_id: Uuid,
    pub username: String,
    pub character_class: Option<CharacterClass>,
    pub status: PlayerStatus,
    pub points: i32,
    pub hand: Vec<Card>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PlayerStatus {
    Active,
    Disconnected,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub room_id: Uuid,
    pub players: Vec<Player>,
    pub phase: GamePhase,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            room_id: Uuid::new_v4(),
            players: Vec::new(),
            phase: GamePhase::ClassSelection,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    RoomStatus {
        room_id: Uuid,
        status: RoomStatus,
        players: Vec<PlayerState>,
    },
    GameState(GameState),
    Error {
        code: String,
        message: String,
        details: Option<serde_json::Value>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundResult {
    pub winner_id: Option<Uuid>,
    pub player_actions: Vec<PlayerAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAction {
    pub player_id: Uuid,
    pub action: Action,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerRoundResult {
    pub player_id: Uuid,
    pub card_played: CardType,
    pub ability_used: Option<String>,
    pub points_gained: i32,
}