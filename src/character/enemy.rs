use crate::system::toml::TomlData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use super::{fellow, spell};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Data {
    id: u32,
    name: String,
    hp: f64,
    lv: u8,
    mp: i32,
    attack: f64,
    defense: f64,
    speed: f64,
    action_count: i32,
    exp: i32,
    accessible_spells: Vec<u32>,
}

impl TomlData for Data {
    fn get_data(id: u32) -> Option<Self> {
        let path = "assets/data/enemy_data.toml";
        let toml_str = fs::read_to_string(path).ok()?;
        let enemy_map: HashMap<String, Self> = match toml::from_str(&toml_str) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to deserialize TOML: {}", e);
                return None;
            }
        };
        let id_map: HashMap<u32, Self> = enemy_map
            .into_iter()
            .map(|(_, data)| (data.id, data))
            .collect();

        id_map.get(&id).cloned()
    }
    fn update_and_save<K, V>(_id: u32, _field: K, _new_value: V) -> bool
    where
        Self: Sized,
        K: AsRef<str>,
        V: ToString,
    {
        todo!()
    }
    fn id(&self) -> u32 {
        self.id
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Data {
    pub fn lv(&self) -> u8 {
        self.lv
    }
    pub fn hp(&self) -> f64 {
        self.hp
    }
    pub fn set_hp(&mut self, hp: f64) {
        self.hp = hp;
    }
    pub fn mp(&self) -> i32 {
        self.mp
    }
    pub fn attack(&self) -> f64 {
        self.attack
    }
    pub fn defense(&self) -> f64 {
        self.defense
    }
    pub fn speed(&self) -> f64 {
        self.speed
    }
    pub fn accessible_spells(&self) -> Vec<u32> {
        self.accessible_spells.clone()
    }
}
impl Data {
    pub fn use_spell(&mut self, id: u32, fellow: &mut fellow::Data) {
        let spell = spell::Data::get_data(id);
        if self.mp > spell.expect("error").mp_cost() {
            match id {
                3 => self.use3(fellow),
                8 => self.use8(),
                _ => println!("error"),
            }
        } else {
            println!("mpが足りません！")
        }
    }
    pub fn use8(&mut self) {
        let base_recover = 10.;
        self.hp += base_recover;
        self.mp -= 4;
        println!("{}はs1を発動した", self.name);
        println!("{}は{}回復した", self.name(), base_recover);
    }
    pub fn use3(&mut self, fellow: &mut fellow::Data) {
        self.mp -= 3;
        let base_damage = 6.;
        fellow.set_hp(fellow.hp() - base_damage);
        println!("{}はs0を発動した", self.name);
        println!("{}は{}ダメージを受けた", fellow.name(), base_damage);
    }
}
