mod character;
use character::{enemy, fellow};
use system::battle_process::battle;
use system::order_of_action::sorted_by_speed;
use system::order_of_action::sorted_by_turn_order;
use system::toml::TomlData;
mod system;
use system::damage;
fn main() {
    let e0 = enemy::Data::get_data(0);
    let e1 = enemy::Data::get_data(5);
    let e2 = enemy::Data::get_data(10);
    let f0 = fellow::Data::get_data(1);
    battle(
        vec![
            e0.expect("e0 is None"),
            e1.expect("e1 is None"),
            e2.expect("e2 is None"),
        ],
        vec![f0.expect("f0 is None")],
    );
}
