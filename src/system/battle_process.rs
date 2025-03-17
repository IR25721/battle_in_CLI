use crossterm::event::{self, Event, KeyCode, KeyEvent};
use proconio::input;
use rand;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum Entity {
    Enemy(enemy::Data),
    Fellow(fellow::Data),
}

impl Entity {
    fn id(&self) -> u32 {
        match self {
            Entity::Enemy(enemy) => enemy.id(),
            Entity::Fellow(fellow) => fellow.id(),
        }
    }
}

use crate::{
    character::{enemy, fellow},
    system::{
        damage::{self, calculate_damage},
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
            if entities.iter().any(|entity| entity.id() == id) {
                if id % 5 == 0
                /*enemy*/
                {
                    let action = rand::random::<f64>();
                    let fellow = &mut fellows[0];
                    let enemy = enemies.iter_mut().find(|enemy| enemy.id() == id);
                    if let Some(enemy) = enemy {
                        if enemy.accessible_spells().len() > 0 {
                            let damage = calculate_damage(&enemy.attack(), &fellow.defense());
                            if (0. ..0.5).contains(&action) {
                                fellow.set_hp(fellow.hp() - damage);
                                println!("{}ダメージ受けた！", damage);
                            } else {
                                other_action(enemy, fellow, &action);
                            }
                        } else {
                            let damage = calculate_damage(&enemy.attack(), &fellow.defense());
                            fellow.set_hp(fellow.hp() - damage);
                            println!("{}ダメージ受けた！", damage);
                        }
                    }
                } else if id % 5 == 1
                /*fellow*/
                {
                    todo!()
                } else {
                    println!("error")
                }
            }
        }
    }
    fn other_action(enemy: &mut enemy::Data, fellow: &mut fellow::Data, action: &f64) {
        for i in 0..enemy.accessible_spells().len() {
            let start = 0.5 * (1.0 + i as f64 / enemy.accessible_spells().len() as f64);
            let end = 0.5 * (1.0 + (i as f64 + 1.0) / enemy.accessible_spells().len() as f64);

            if (start..end).contains(action) {
                match i {
                    3 => enemy.use3(fellow),
                    8 => enemy.use8(),
                    _ => println!("error"),
                }
            } else {
                continue;
            }
        }
    }
}
