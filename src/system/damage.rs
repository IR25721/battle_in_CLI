use rand;
pub fn calculate_damage(total_attack: &f64, totoal_defense: &f64) -> f64 {
    let based_damage = (total_attack - 0.5 * totoal_defense) * 0.5;
    println!("based_damage:{}", based_damage);
    if based_damage > 0. {
        let damage = rand::random::<f64>() * (based_damage / 5. + 1.) + based_damage * 0.875;
        println!("damage:{}", damage);
        damage
    } else {
        0.
    }
}
