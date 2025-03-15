use crossterm::event::{self, Event, KeyCode, KeyEvent};
use proconio::input;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum Entity {
    Enemy(enemy::Data),
    Fellow(fellow::Data),
}

use crate::{
    character::{enemy, fellow},
    system::{
        order_of_action::{sorted_by_speed, sorted_by_turn_order},
        toml::TomlData,
    },
};

pub fn battle(mut enemies: Vec<enemy::Data>, mut fellows: Vec<fellow::Data>) {
    let entities: Vec<Entity> = enemies
        .clone()
        .into_iter()
        .map(Entity::Enemy)
        .chain(fellows.clone().into_iter().map(Entity::Fellow))
        .collect();
    let enemies_name: Vec<String> = enemies.clone().iter().map(|a| a.name()).collect();
    println!("{}が現れた！", enemies_name.join(", "));

    while enemies.iter().all(|e| e.hp() > 0.) && fellows.iter().all(|f| f.hp() > 0.) {
        println!("残りHP");
        fellows.iter().map(|f| println!("{},{}", f.name(), f.hp()));
        println!("行動？\n0:攻撃\n1:防御\n2:魔術\n3:道具\n4:逃げる");
        input! {
            action:u8
        }
        let order_of_action = sorted_by_turn_order(&mut sorted_by_speed(&enemies, &fellows));

        for id in order_of_action
            .iter()
            .map(|(id, _)| *id)
            .collect::<Vec<u32>>()
        {
            todo!()
        }
    }
}
