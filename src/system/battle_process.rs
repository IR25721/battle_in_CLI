use proconio::input;
use rand;

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
    character::{enemy, fellow, item, spell},
    system::{
        damage::calculate_damage,
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

    let mut escape = false;

    while enemies.iter().any(|e| e.hp() > 0.) && fellows.iter().any(|f| f.hp() > 0.) {
        println!("残りHP:{}:{}", fellows[0].name(), fellows[0].hp().floor());
        let enemies_hp: Vec<String> = enemies
            .iter()
            .map(|enemy| format!("残りHP:{}:{}", enemy.name(), enemy.hp().floor()))
            .collect();
        println!("{}", enemies_hp.join(", "));

        let mut valid_input = false;
        let mut action: u8 = 0;

        while !valid_input {
            println!("行動？\n0:攻撃\n1:魔術\n2:道具\n3:逃げる");
            input! {
                action_input: u8
            }

            if action_input == 0 || action_input == 1 || action_input == 2 || action_input == 3 {
                action = action_input;
                valid_input = true;
            } else {
                println!("無効な選択肢です。もう一度入力してください。");
            }
        }

        let order_of_action = sorted_by_turn_order(&mut sorted_by_speed(&enemies, &fellows));

        for id in order_of_action.iter().map(|(id, _)| *id) {
            if let Some(entity) = entities.iter().find(|entity| entity.id() == id) {
                match entity {
                    Entity::Enemy(enemy) => {
                        if enemy.hp() <= 0. {
                            continue;
                        }
                        let action_rand = rand::random::<f64>();
                        let fellow = &mut fellows[0];

                        if let Some(enemy) = enemies.iter_mut().find(|enemy| enemy.id() == id) {
                            if action_rand < 0.5 {
                                let damage =
                                    calculate_damage(&enemy.attack(), &fellow.defense()).floor();
                                fellow.set_hp((fellow.hp() - damage).max(0.));
                                println!(
                                    "{}は{}に{}ダメージ与えた",
                                    enemy.name(),
                                    fellow.name(),
                                    damage
                                );
                            } else if enemy.accessible_spells().len() > 0 && action_rand >= 0.5 {
                                other_action(enemy, fellow, &action_rand);
                            }
                        }
                    }

                    Entity::Fellow(fellow) => {
                        if fellow.hp() <= 0. {
                            continue;
                        }
                        if action == 0 {
                            let selected_enemy = select_enemy(&mut enemies);
                            let damage =
                                calculate_damage(&fellow.attack(), &selected_enemy.defense())
                                    .floor();
                            selected_enemy.set_hp((selected_enemy.hp() - damage).max(0.));
                            println!("{}に{}ダメージ与えた", selected_enemy.name(), damage);
                        }
                        if action == 1 {
                            let fellow = &mut fellows[0];
                            if let Some(selected_spell) = select_spell(fellow) {
                                if selected_spell == 3 {
                                    let selected_enemy = select_enemy(&mut enemies);
                                    fellow.use_attack_spell(selected_spell, selected_enemy);
                                } else {
                                    fellow.use_assist_spell(selected_spell);
                                }
                            } else {
                                println!("魔法の選択がキャンセルされました。");
                            }
                        }
                        if action == 2 {
                            let fellow = &mut fellows[0];
                            if let Some(selected_item) = select_item(fellow) {
                                fellow.use_assist_item(selected_item);
                            } else {
                                println!("道具の選択がキャンセルされました。");
                            }
                        }
                        if action == 3 {
                            let rand = rand::random::<f64>();
                            if rand < 0.5 {
                                println!("逃げれた！");
                                escape = true; // 逃げるフラグを立てる
                                break; // forループから抜ける
                            } else {
                                println!("逃げれなかった...");
                            }
                        }
                    }
                }
            }
        }

        if escape {
            break; // whileループから抜ける
        }
    }

    fn other_action(enemy: &mut enemy::Data, fellow: &mut fellow::Data, action: &f64) {
        for i in 0..enemy.accessible_spells().len() {
            let start = 0.5 * (1.0 + i as f64 / enemy.accessible_spells().len() as f64);
            let end = 0.5 * (1.0 + (i as f64 + 1.0) / enemy.accessible_spells().len() as f64);

            if (start..end).contains(action) {
                match enemy.accessible_spells()[i] {
                    3 => enemy.use3(fellow),
                    8 => enemy.use8(),
                    _ => println!("error1"),
                }
            }
        }
    }
    fn select_item(fellow: &fellow::Data) -> Option<u32> {
        loop {
            let items: Vec<item::Data> = fellow
                .accessible_items()
                .iter()
                .filter_map(|&id| item::Data::get_data(id))
                .collect();

            if items.is_empty() {
                println!("使用可能なアイテムがありません。");
                return None;
            }

            let items_display: Vec<String> = items
                .iter()
                .map(|item| format!("{}:{} (残り: {})", item.id(), item.name(), item.quantity()))
                .collect();

            println!("どの道具をを使用しますか？使用しない場合は0を選択");
            println!("{}", items_display.join(", "));

            input! {
                select: u32
            }

            if select == 0 {
                return Some(0);
            } else if let Some(item) = items.iter().find(|item| item.id() == select) {
                if item.quantity() == 0 {
                    println!("{} を使えません", item.name());
                } else {
                    return Some(item.id());
                }
            } else {
                println!("無効なIDです。もう一度入力してください。");
            }
        }
    }

    fn select_spell(fellow: &fellow::Data) -> Option<u32> {
        loop {
            let spells: Vec<spell::Data> = fellow
                .accessible_spells()
                .iter()
                .filter_map(|&id| spell::Data::get_data(id))
                .collect();

            if spells.is_empty() {
                println!("使用可能な魔法がありません。");
                return None;
            }

            let spells_display: Vec<String> = spells
                .iter()
                .map(|spell| {
                    format!(
                        "{}:{} (MP消費: {})\n説明：{}\n",
                        spell.id(),
                        spell.name(),
                        spell.mp_cost(),
                        spell.description()
                    )
                })
                .collect();

            println!("どの魔法を使用しますか？");
            println!("{}", spells_display.join(""));

            input! {
                select: u32
            }

            if let Some(spell) = spells.iter().find(|spell| spell.id() == select) {
                if fellow.mp() < spell.mp_cost() {
                    println!("{} を発動できません（MP不足）", spell.name());
                } else {
                    return Some(spell.id());
                }
            } else {
                println!("無効なIDです。もう一度入力してください。");
            }
        }
    }

    fn select_enemy(enemies: &mut Vec<enemy::Data>) -> &mut enemy::Data {
        loop {
            let enemies_name: Vec<String> = enemies
                .iter()
                .filter(|enemy| enemy.hp() > 0.)
                .map(|enemy| format!("{}:{}", enemy.id(), enemy.name()))
                .collect();

            if enemies_name.is_empty() {
                panic!("有効な敵がいません。");
            }

            println!("どの敵にしますか？");
            println!("{}", enemies_name.join(", "));

            input! {
                select: u32
            }

            if let Some(index) = enemies
                .iter_mut()
                .position(|enemy| enemy.id() == select && enemy.hp() > 0.)
            {
                return &mut enemies[index];
            } else {
                println!(
                    "無効なIDです。またはその敵はすでに倒れています。もう一度入力してください。"
                );
            }
        }
    }
    //作ったので残しておく
    fn select_fellow(fellows: &mut Vec<fellow::Data>) -> &mut fellow::Data {
        loop {
            // 使用可能な仲間の情報を表示
            let fellows_name: Vec<String> = fellows
                .iter()
                .filter(|fellow| fellow.hp() > 0.0) // HP が 0 より大きい仲間を対象
                .map(|fellow| format!("{}: {} HP: {}", fellow.id(), fellow.name(), fellow.hp()))
                .collect();

            if fellows_name.is_empty() {
                panic!("有効な仲間がいません。");
            }

            println!("どの仲間を選びますか？");
            println!("{}", fellows_name.join(", "));

            input! {
                select: u32
            }

            // 選択した仲間の ID を使って、該当する仲間を返す
            if let Some(index) = fellows
                .iter_mut()
                .position(|fellow| fellow.id() == select && fellow.hp() > 0.0)
            {
                return &mut fellows[index];
            } else {
                println!(
                    "無効なIDです。またはその仲間はすでに倒れています。もう一度入力してください。"
                );
            }
        }
    }

    println!("戦闘結果");
    println!("残りHP:{}:{}", fellows[0].name(), fellows[0].hp().floor());
    let enemies_hp: Vec<String> = enemies
        .iter()
        .map(|enemy| format!("残りHP:{}:{}", enemy.name(), enemy.hp().floor()))
        .collect();
    println!("{}", enemies_hp.join(", "));
}
