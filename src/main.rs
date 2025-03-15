mod character;
use character::{enemy, fellow};
use system::order_of_action::sorted_by_speed;
use system::order_of_action::sorted_by_turn_order;
use system::toml::TomlData;
mod system;
use system::damage;
fn main() {
    let f0 = fellow::Data::get_data(0);
    let a = match f0 {
        Some(ref f) => f.lv(),
        None => 0,
    };
    fellow::Data::update_and_save(0, "lv", 6);
    let f0 = fellow::Data::get_data(0);
    let b = match f0 {
        Some(ref f) => f.lv(),
        None => 0,
    };
    println!("Level UP!:{}=>{}", a, b);
    let e0 = enemy::Data::get_data(0);
    let e1 = enemy::Data::get_data(1);
    let e2 = enemy::Data::get_data(2);
    let es = vec![e0, e1, e2];
    let fs = vec![f0];
    let es: Vec<enemy::Data> = es.into_iter().filter_map(|e| e).collect();
    let fs: Vec<fellow::Data> = fs.into_iter().filter_map(|f| f).collect();

    let order_of_action = sorted_by_turn_order(&mut sorted_by_speed(&es, &fs));

    println!("{:?}", order_of_action);
}
