use crate::system::toml::TomlData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Data {
    id: u32,
    name: String,
    modifier: f64,
    quantity: u8,
    bonus_effect: String,
}

impl TomlData for Data {
    fn get_data(id: u32) -> Option<Self> {
        let path = "assets/data/enemy_data.toml";
        let toml_str = fs::read_to_string(path).ok()?;
        let equipment_map: HashMap<String, Self> = match toml::from_str(&toml_str) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to deserialize TOML: {}", e);
                return None;
            }
        };
        let id_map: HashMap<u32, Self> = equipment_map
            .into_iter()
            .map(|(_, data)| (data.id, data))
            .collect();

        id_map.get(&id).cloned()
    }
    fn update_and_save<K, V>(id: u32, field: K, new_value: V) -> bool
    where
        Self: Sized,
        K: AsRef<str>,
        V: ToString,
    {
        let path = "assets/data/equipment_data.toml";

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
                    "modifier" => {
                        if let Ok(new_modifier) = new_value.to_string().parse::<f64>() {
                            item.modifier = new_modifier;
                            found = true;
                        }
                    }
                    "quantity" => {
                        if let Ok(new_quantity) = new_value.to_string().parse::<u8>() {
                            item.quantity = new_quantity;
                            found = true;
                        }
                    }
                    "bonus_effect" => {
                        if let Ok(new_bonus_effect) = new_value.to_string().parse::<String>() {
                            item.bonus_effect = new_bonus_effect;
                            found = true;
                        }
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

    fn id(&self) -> u32 {
        self.id
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Data {
    pub fn modifier(&self) -> f64 {
        self.modifier
    }

    pub fn bonus_effect(&self) -> String {
        self.bonus_effect.clone()
    }

    pub fn quantity(&self) -> u8 {
        self.quantity
    }
}
