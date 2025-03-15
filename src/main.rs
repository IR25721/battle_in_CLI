mod character;
use character::{enemy, fellow};
use system::toml::TomlData;
mod system;
use system::damage;
use system::order_of_action;
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
}
