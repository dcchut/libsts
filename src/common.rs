use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    id: String,
    misc: u32,
    upgrades: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageTaken {
    damage: f64,
    enemies: String,
    floor: f64,
    turns: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardChoice {
    floor: f64,
    not_picked: Vec<String>,
    picked: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BossRelicChoice {
    picked: Option<String>,
    not_picked: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventChoice {
    cards_upgraded: Option<Vec<String>>,
    cards_obtained: Option<Vec<String>>,
    relics_obtained: Option<Vec<String>>,
    damage_healed: f64,
    damage_taken: f64,
    event_name: String,
    floor: f64,
    gold_gain: f64,
    gold_loss: f64,
    max_hp_gain: f64,
    max_hp_loss: f64,
    player_choice: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FloorKey {
    floor: f64,
    key: String,
    data: Option<String>,
}
