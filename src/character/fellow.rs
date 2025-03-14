use crate::system::toml::TomlData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Data {
    id: u8,
    name: String,
    lv: u8,
    total_exp: i32,
    hp: f64,
    mp: i32,
    attack: f64,
    defense: f64,
    speed: f64,
    sword_id: usize,
    armor_id: usize,
    accessible_spells: Vec<u8>,
}

impl TomlData for Data {
    fn get_data(id: u8) -> Option<Self> {
        let path = "assets/data/fellow_data.toml";
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
    fn update_and_save<K, V>(id: u8, field: K, new_value: V) -> bool
    where
        Self: Sized,
        K: AsRef<str>,
        V: ToString,
    {
        let path = "assets/data/fellow_data.toml";

        let toml_str = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return false,
        };

        let mut items_map: HashMap<String, Data> = match toml::from_str(&toml_str) {
            Ok(map) => map,
            Err(_) => return false,
        };

        let mut found = false;
        for item in items_map.values_mut() {
            if item.id == id {
                match field.as_ref() {
                    "name" => {
                        item.name = new_value.to_string();
                        found = true;
                    }
                    "lv" => {
                        if let Ok(new_lv) = new_value.to_string().parse::<u8>() {
                            item.lv = new_lv;
                            found = true;
                        }
                    }
                    "total_exp" => {
                        if let Ok(new_exp) = new_value.to_string().parse::<i32>() {
                            item.total_exp = new_exp;
                            found = true;
                        }
                    }
                    "hp" => {
                        if let Ok(new_hp) = new_value.to_string().parse::<f64>() {
                            item.hp = new_hp;
                            found = true;
                        }
                    }
                    "mp" => {
                        if let Ok(new_mp) = new_value.to_string().parse::<i32>() {
                            item.mp = new_mp;
                            found = true;
                        }
                    }
                    "attack" => {
                        if let Ok(new_atk) = new_value.to_string().parse::<f64>() {
                            item.attack = new_atk;
                            found = true;
                        }
                    }
                    "defense" => {
                        if let Ok(new_def) = new_value.to_string().parse::<f64>() {
                            item.defense = new_def;
                            found = true;
                        }
                    }
                    "speed" => {
                        if let Ok(new_spd) = new_value.to_string().parse::<f64>() {
                            item.speed = new_spd;
                            found = true;
                        }
                    }
                    "sword_id" => {
                        if let Ok(new_sword_id) = new_value.to_string().parse::<usize>() {
                            item.sword_id = new_sword_id;
                            found = true;
                        }
                    }
                    "armor_id" => {
                        if let Ok(new_armor_id) = new_value.to_string().parse::<usize>() {
                            item.armor_id = new_armor_id;
                            found = true;
                        }
                    }
                    "accessible_spells" => {
                        let new_spells: Vec<u8> = new_value
                            .to_string()
                            .split(',')
                            .filter_map(|s| s.parse::<u8>().ok())
                            .collect();

                        item.accessible_spells = new_spells;
                        found = true;
                    }
                    _ => {}
                }
                break;
            }
        }

        if !found {
            return false;
        }

        let new_toml = match toml::to_string(&items_map) {
            Ok(t) => t,
            Err(_) => return false,
        };

        fs::write(path, new_toml).is_ok()
    }
}

impl Data {
    pub fn lv(&self) -> u8 {
        self.lv
    }
}
