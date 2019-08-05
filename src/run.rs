use crate::common::*;
use failure_derive::Fail;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Fail)]
pub enum RunError {
    #[fail(display = "JSON error: {}", error_string)]
    JSONError { error_string: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Run {
    pub ascension_level: u32,
    pub boss_relics: Vec<BossRelicChoice>,
    pub build_version: String,
    pub campfire_choices: Vec<FloorKey>,
    pub campfire_rested: u32,
    pub campfire_upgraded: u32,
    pub card_choices: Vec<CardChoice>,
    pub character_chosen: String,
    pub chose_seed: bool,
    pub circlet_count: u32,
    pub current_hp_per_floor: Vec<u32>,
    pub damage_taken: Vec<DamageTaken>,
    pub event_choices: Vec<EventChoice>,
    pub floor_reached: u32,
    pub gold: u32,
    pub gold_per_floor: Vec<u32>,
    pub is_ascension_mode: bool,
    pub is_beta: bool,
    pub is_daily: bool,
    pub is_endless: bool,
    pub is_prod: bool,
    pub is_trial: bool,
    pub item_purchase_floors: Vec<u32>,
    pub items_purchased: Vec<String>,
    pub items_purged: Vec<String>,
    pub items_purged_floors: Vec<u32>,
    pub killed_by: String, // maybe should be Option<String>
    pub local_time: String,
    pub master_deck: Vec<String>,
    pub max_hp_per_floor: Vec<u32>,
    pub neow_bonus: String,
    pub neow_cost: String,
    pub path_per_floor: Vec<Option<String>>,
    pub path_taken: Vec<String>,
    pub play_id: String,
    pub player_experience: u32,
    pub playtime: u32,
    pub potions_floor_spawned: Vec<u32>,
    pub potions_floor_usage: Vec<u32>,
    pub potions_obtained: Vec<FloorKey>,
    pub purchased_purges: u32,
    pub relics: Vec<String>,
    pub relics_obtained: Vec<FloorKey>,
    pub score: u32,
    pub seed_played: String,
    pub seed_source_timestamp: u64,
    pub timestamp: u64,
    pub victory: bool,
    pub win_rate: u32,
}

impl Run {
    pub fn new(contents: &str) -> Result<Run, RunError> {
        serde_json::from_str(contents).map_err(|e| RunError::JSONError {
            error_string: e.to_string(),
        })
    }
}
