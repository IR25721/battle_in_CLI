use crate::system::toml::TomlData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Data {
    id: u8,
    name: String,
    hp: f64,
    lv: u8,
    mp: f64,
    attack: f64,
    defense: f64,
    speed: f64,
    action_count: i32,
    exp: i32,
    accessible_spells: Vec<u8>,
}

impl TomlData for Data {
    fn get_data(id: u8) -> Option<Self> {
        let path = "assets/data/enemy_data.toml";
        let toml_str = fs::read_to_string(path).ok()?;
        let enemy_map: HashMap<String, Self> = match toml::from_str(&toml_str) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to deserialize TOML: {}", e);
                return None;
            }
        };
        let id_map: HashMap<u8, Self> = enemy_map
            .into_iter()
            .map(|(_, data)| (data.id, data))
            .collect();

        id_map.get(&id).cloned()
    }
    fn update_and_save<K, V>(_id: u8, _field: K, _new_value: V) -> bool
    where
        Self: Sized,
        K: AsRef<str>,
        V: ToString,
    {
        todo!()
    }
}
