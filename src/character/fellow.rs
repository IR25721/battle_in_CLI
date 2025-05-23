use crate::system::toml::TomlData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use super::{enemy, item, spell};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Data {
    id: u32,
    name: String,
    lv: u8,
    total_exp: i32,
    hp: f64,
    mp: i32,
    attack: f64,
    defense: f64,
    speed: f64,
    sword_id: u32,
    armor_id: u32,
    accessible_spells: Vec<u32>,
    accessible_items: Vec<u32>,
}

impl TomlData for Data {
    fn get_data(id: u32) -> Option<Self> {
        let path = "assets/data/fellow_data.toml";
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
    fn update_and_save<K, V>(id: u32, field: K, new_value: V) -> bool
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
                        if let Ok(new_sword_id) = new_value.to_string().parse::<u32>() {
                            item.sword_id = new_sword_id;
                            found = true;
                        }
                    }
                    "armor_id" => {
                        if let Ok(new_armor_id) = new_value.to_string().parse::<u32>() {
                            item.armor_id = new_armor_id;
                            found = true;
                        }
                    }
                    "accessible_spells" => {
                        let new_spells: Vec<u32> = new_value
                            .to_string()
                            .split(',')
                            .filter_map(|s| s.parse::<u32>().ok())
                            .collect();

                        item.accessible_spells = new_spells;
                        found = true;
                    }
                    "accessible_items" => {
                        let new_items: Vec<u32> = new_value
                            .to_string()
                            .split(',')
                            .filter_map(|s| s.parse::<u32>().ok())
                            .collect();

                        item.accessible_items = new_items;
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
    pub fn sword_id(&self) -> u32 {
        self.sword_id
    }
    pub fn armor_id(&self) -> u32 {
        self.armor_id
    }
    pub fn accessible_spells(&self) -> Vec<u32> {
        self.accessible_spells.clone()
    }
    pub fn accessible_items(&self) -> Vec<u32> {
        self.accessible_items.clone()
    }
}

impl Data {
    pub fn use_attack_spell(&mut self, spell_id: u32, enemy: &mut enemy::Data) {
        let spell = spell::Data::get_data(spell_id);
        if self.mp > spell.expect("error").mp_cost() {
            match spell_id {
                3 => self.use3(enemy),
                _ => println!("error"),
            }
        } else {
            println!("mpが足りません！")
        }
    }
    pub fn use_assist_spell(&mut self, spell_id: u32) {
        let spell = spell::Data::get_data(spell_id);
        if self.mp > spell.expect("error").mp_cost() {
            match spell_id {
                8 => self.use8(),
                13 => self.use13(),
                18 => self.use18(),
                _ => println!("error"),
            }
        } else {
            println!("mpが足りません！")
        }
    }
    pub fn use8(&mut self) {
        let base_recover = 10.;
        let fellow = Data::get_data(self.id);
        self.hp += 10.;
        if self.hp > fellow.clone().expect("not found").hp() {
            self.set_hp(fellow.expect("not found").hp());
        } else {
            self.set_hp(self.hp());
        }
        self.mp -= 4;
        println!("{}はs1を発動した", self.name);
        println!("{}は{}回復した", self.name(), base_recover);
    }
    pub fn use3(&mut self, enemy: &mut enemy::Data) {
        let base_damage = 6.;
        self.mp -= 3;
        enemy.set_hp(enemy.hp() - 6.);
        println!("{}はs0を発動した", self.name);
        println!("{}は{}のダメージを与えた！", enemy.name(), base_damage);
    }
    pub fn use13(&mut self) {
        println!("before:{}", self.attack);
        let base_rate = 1.2;
        self.mp -= 3;
        self.attack *= base_rate;
        println!("after:{}", self.attack);
        println!("{}はs2を発動した", self.name);
        println!("{}の攻撃力が上がった！", self.name())
    }
    pub fn use18(&mut self) {
        let base_rate = 1.2;
        self.mp -= 3;
        self.defense *= base_rate;
        println!("{}はs3を発動した", self.name);
        println!("{}の守備力が上がった！", self.name())
    }
}
impl Data {
    pub fn use_assist_item(&mut self, item_id: u32) {
        let item = item::Data::get_data(item_id);
        match item_id {
            0 => println!("何も使わなかった"),
            2 => self.use2(&mut item.expect("error")),
            _ => println!("error"),
        }
    }

    pub fn use2(&mut self, item: &mut item::Data) {
        let base_recover = 8.;
        item.set_quantity(item.quantity() - 1);
        self.set_hp(self.hp() + base_recover);
        item::Data::update_and_save(2, "quantity", item.quantity());
        println!("{}は{}回復した", self.name(), base_recover)
    }
}
