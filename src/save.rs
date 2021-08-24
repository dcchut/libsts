use crate::common::*;
use base64::read::DecoderReader;
use base64::{decode, encode};
use failure_derive::Fail;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;

#[derive(Clone, Debug, Fail, PartialEq, Eq)]
pub enum SaveError {
    #[fail(display = "JSON error: {}", error_string)]
    JSONError { error_string: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Save {
    // Game parameters
    pub ascension_level: u32,
    pub name: String,
    pub custom_mods: Vec<String>,
    pub daily_mods: Vec<String>,
    pub is_endless_mode: bool,
    pub is_daily: bool,
    pub is_ascension_mode: bool,
    pub is_trial: bool,
    pub daily_date: u32,

    // Game state
    pub act_num: u32,
    pub gold: u32,
    pub current_health: u32,
    pub max_health: u32,
    pub post_combat: bool,
    pub smoked: bool,
    pub chose_neow_reward: bool,
    pub play_time: u32,
    pub obtained_cards: HashMap<String, u32>,
    pub mugged: bool,
    pub monsters_killed: u32,
    pub spirit_count: u32,
    pub relic_counters: Vec<i32>,
    pub one_time_event_list: Vec<String>,
    pub max_orbs: u32,
    pub level_name: String,
    pub save_date: u64,
    pub is_final_act_on: bool,
    pub has_sapphire_key: bool,
    pub has_ruby_key: bool,
    pub has_emerald_key: bool,
    pub hand_size: u32,
    pub gold_gained: u32,
    pub floor_num: f64,
    pub event_list: Vec<String>,
    pub event_chances: Vec<f64>,
    pub cards: Vec<Card>,

    // For some reason this is the only field that appears camelCased in the savefile
    #[serde(rename = "purgeCost")]
    pub purge_cost: u32,

    // Relics
    pub boss_relics: Vec<String>,
    pub common_relics: Vec<String>,
    pub rare_relics: Vec<String>,
    pub shop_relics: Vec<String>,
    pub uncommon_relics: Vec<String>,
    pub relics: Vec<String>,

    // Navigation
    pub current_room: String,
    pub room_x: i32,
    pub room_y: i32,
    pub path_x: Vec<i32>,
    pub path_y: Vec<i32>,

    // Monsters
    pub monster_list: Vec<String>,
    pub elite_monster_list: Vec<String>,
    pub boss_list: Vec<String>,
    pub boss: String,

    // Potions
    pub potions: Vec<String>,
    pub potion_slots: u32,
    pub potion_seed_count: u32,
    pub potion_chance: i32,

    // Relating to certain score bonuses
    pub perfect: u32,
    pub overkill: bool,
    pub elites1_killed: u32,
    pub elites2_killed: u32,
    pub elites3_killed: u32,
    pub combo: bool,

    // Metrics
    pub metric_relics_obtained: Vec<FloorKey>,
    pub metric_purchased_purges: u32,
    pub metric_potions_obtained: Vec<FloorKey>,
    pub metric_playtime: u32,
    pub metric_potions_floor_usage: Vec<u32>,
    pub metric_potions_floor_spawned: Vec<u32>,
    pub metric_path_taken: Vec<String>,
    pub metric_path_per_floor: Vec<Option<String>>,
    pub metric_max_hp_per_floor: Vec<u32>,
    pub metric_items_purged_floors: Vec<u32>,
    pub metric_items_purged: Vec<String>,
    pub metric_item_purchase_floors: Vec<u32>,
    pub metric_gold_per_floor: Vec<u32>,
    pub metric_floor_reached: u32,
    pub metric_event_choices: Vec<EventChoice>,
    pub metric_damage_taken: Vec<DamageTaken>,
    pub metric_current_hp_per_floor: Vec<u32>,
    pub metric_card_choices: Vec<CardChoice>,
    pub metric_campfire_upgraded: u32,
    pub metric_campfire_rituals: u32,
    pub metric_campfire_rested: u32,
    pub metric_campfire_meditates: u32,
    pub metric_campfire_choices: Vec<FloorKey>,
    pub metric_build_version: String,
    pub metric_boss_relics: Vec<BossRelicChoice>,
    pub metric_seed_played: String,

    // Seeds (+related)
    pub seed: i64,
    pub special_seed: u32,
    pub treasure_seed_count: u32,
    pub shuffle_seed_count: u32,
    pub relic_seed_count: u32,
    pub monster_seed_count: u32,
    pub merchant_seed_count: u32,
    pub event_seed_count: u32,
    pub card_seed_count: u32,
    pub card_random_seed_randomizer: i32,
    pub card_random_seed_count: u32,
    pub ai_seed_count: u32,

    // Unknown
    pub neow_cost: String,
    pub neow_bonus: String,
    pub mystery_machine: u32,
    pub champions: u32,
    pub blights: Vec<String>,
    pub blight_counters: Vec<i32>,
    pub endless_increments: Vec<i32>,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

const KEY: &[u8; 3] = b"key";

fn xor_key(bytes: &[u8]) -> Vec<u8> {
    bytes
        .iter()
        .enumerate()
        .map(|(i, v)| *v ^ KEY[i % KEY.len()])
        .collect()
}

impl Save {
    /// Attempts to create an instance of Save using the contents of a savefile.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsts::{Save, SaveError};
    /// use std::fs;
    ///
    ///
    /// let contents = fs::read_to_string("IRONCLAD.autosave").unwrap();
    /// let save = Save::new(&contents).unwrap();
    /// ```
    pub fn new(contents: &str) -> Result<Save, SaveError> {
        // Attempt to perform a base64 decode of the input string, xor-ing the input against our key
        let decoded = decode(contents).map(|bytes| xor_key(&bytes));

        let bytes = match decoded {
            Ok(decoded) => decoded,
            Err(_) => {
                // If that fails, attempt to decode directly
                Vec::from(contents)
            }
        };

        // Deserialize the resulting JSON to our custom struct
        serde_json::from_slice(bytes.as_slice()).map_err(|e| SaveError::JSONError {
            error_string: e.to_string(),
        })
    }

    /// Reads savefile from given reader.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsts::Save;
    /// use std::fs::File;
    ///
    ///
    /// let file = File::open("IRONCLAD.autosave").unwrap();
    /// let save = Save::from_reader(file).unwrap();
    /// ```
    pub fn from_reader<R>(mut reader: R) -> Result<Save, SaveError>
    where
        R: Read,
    {
        match serde_json::from_reader(KeyXORReader::new(DecoderReader::new(
            &mut reader,
            base64::STANDARD,
        ))) {
            Ok(s) => s,
            Err(_) => serde_json::from_reader(reader).map_err(|e| SaveError::JSONError {
                error_string: e.to_string(),
            }),
        }
    }

    /// Attempts to represent this save file as a byte vector
    pub fn to_bytes(&self) -> Result<Vec<u8>, SaveError> {
        serde_json::to_vec(self).map_err(|e| SaveError::JSONError {
            error_string: e.to_string(),
        })
    }

    /// Attempts to represent this save file as a string
    pub fn to_string(&self) -> Result<String, SaveError> {
        serde_json::to_string(self).map_err(|e| SaveError::JSONError {
            error_string: e.to_string(),
        })
    }

    /// Attempts to represent this save file as a base64 string
    pub fn to_b64_string(&self) -> Result<String, SaveError> {
        let bytes = self.to_bytes().map(|b| xor_key(&b))?;

        Ok(encode(std::str::from_utf8(&bytes).unwrap()))
    }
}

#[derive(Debug)]
struct KeyXORReader<R> {
    inner: R,
    pos: usize,
}

impl<R> KeyXORReader<R> {
    fn new(inner: R) -> KeyXORReader<R>
    where
        R: Read,
    {
        KeyXORReader { inner, pos: 0 }
    }
}

impl<R> Read for KeyXORReader<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf).map(|n| {
            for i in 0..n {
                buf[i] ^= KEY[(self.pos + i) % KEY.len()]
            }
            self.pos += n;
            n
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_xor_reader() {
        let mut result: Vec<u8> = Vec::new();
        let mut input: Vec<u8> = Vec::new();
        KeyXORReader::new(&input[..])
            .read_to_end(&mut result)
            .unwrap();
        assert!(result.is_empty());

        input = vec![0];
        KeyXORReader::new(&input[..])
            .read_to_end(&mut result)
            .unwrap();
        assert_eq!(result, &[b'k']);

        result.clear();
        input = vec![b'k', b'e', b'y', b'k'];
        KeyXORReader::new(&input[..])
            .read_to_end(&mut result)
            .unwrap();
        assert_eq!(result, &[0, 0, 0, 0])
    }
}
