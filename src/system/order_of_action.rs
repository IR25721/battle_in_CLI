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

pub fn sorted_by_turn_order(sorted_by_speed: &mut Vec<(u32, f64)>) -> Vec<(u32, f64)> {
    let mut result = Vec::<(u32, f64)>::new();
    for a in sorted_by_speed.iter() {
        if result.iter().any(|e| e == a) {
            continue;
        }
        for b in sorted_by_speed.iter() {
            if result.iter().any(|e| e == b) || a == b {
                continue;
            }

            let ratio = match a.1 / b.1 {
                std::f64::INFINITY => 2.,
                v => v,
            };

            if ratio < 1. {
                result.push(ordering_by_probability(b, a));
            } else if (1. ..=2.).contains(&ratio) {
                result.push(ordering_by_probability(a, b))
            } else {
                result.push(*a)
            }
            break;
        }
    }
    for a in sorted_by_speed.iter() {
        if !result.iter().any(|e| e == a) {
            result.push(*a);
        }
    }
    result
}
fn ordering_by_probability(bigger: &(u32, f64), smaller: &(u32, f64)) -> (u32, f64) {
    let ratio = match bigger.1 / smaller.1 {
        std::f64::INFINITY => 2.,
        v => v,
    };

    let bound = 0.5 * (1. - (ratio - 2.).powf(2.)).powf(0.5) + 0.5;

    if rand::random::<f64>() < bound {
        *bigger
    } else {
        *smaller
    }
}
