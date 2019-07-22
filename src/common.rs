use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub misc: u32,
    pub upgrades: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageTaken {
    pub damage: f64,
    pub enemies: String,
    pub floor: f64,
    pub turns: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardChoice {
    pub floor: f64,
    pub not_picked: Vec<String>,
    pub picked: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BossRelicChoice {
    pub picked: Option<String>,
    pub not_picked: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventChoice {
    pub cards_upgraded: Option<Vec<String>>,
    pub cards_obtained: Option<Vec<String>>,
    pub relics_obtained: Option<Vec<String>>,
    pub damage_healed: f64,
    pub damage_taken: f64,
    pub event_name: String,
    pub floor: f64,
    pub gold_gain: f64,
    pub gold_loss: f64,
    pub max_hp_gain: f64,
    pub max_hp_loss: f64,
    pub player_choice: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FloorKey {
    pub floor: f64,
    pub key: String,
    pub data: Option<String>,
}
