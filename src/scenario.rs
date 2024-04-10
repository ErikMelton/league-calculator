use crate::build::Build;

pub struct Scenario {
    pub you_build: Build,
    // TODO: Your ability rotation, hit chance
    pub enemy_build: Build,
    // TODO: Enemy ability rotation, hit chance
}

pub fn calculate_scenario(scenario: Scenario) {
    println!("Calculating scenario...");
}

fn calculate_aa_damage(scenario: &Scenario) -> i32{
    println!("Calculating damage...");

    0
}
