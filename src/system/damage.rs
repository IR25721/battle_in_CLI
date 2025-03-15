use rand;
pub fn calculate_damage(total_attack: &f64, totoal_defense: &f64) -> f64 {
    let based_damage = (total_attack - 0.5 * totoal_defense) * 0.5;
    rand::random::<f64>() * (based_damage / 4. + 1.) + based_damage * 0.875
}
