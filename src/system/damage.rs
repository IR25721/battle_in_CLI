use rand;
pub fn calculate_damage(attack: &f64, defense: &f64) -> f64 {
    let based_damage = (attack - 0.5 * defense) * 0.5;
    rand::random::<f64>() * (based_damage / 4. + 1.) + based_damage * 0.875
}
