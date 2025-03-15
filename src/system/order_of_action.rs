use crate::character::enemy;
use crate::character::fellow;
use rand;

use super::toml::TomlData;
pub fn sorted_by_speed(enemies: &Vec<enemy::Data>, fellows: &Vec<fellow::Data>) -> Vec<(u32, f64)> {
    let enemies_speed: Vec<(u32, f64)> = enemies
        .iter()
        .map(|enemy| (enemy.id(), enemy.speed()))
        .collect();
    let fellows_speed: Vec<(u32, f64)> = fellows
        .iter()
        .map(|fellow| (fellow.id(), fellow.speed()))
        .collect();
    let mut speeds = [enemies_speed, fellows_speed].concat();
    speeds.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    speeds
}
