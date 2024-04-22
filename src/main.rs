use std::time::Duration;
use crate::build::Build;
use crate::scenario::Scenario;

mod item;
mod build;
mod scenario;
mod effects;
mod simulation;
mod champion;
mod damage;
mod utils;

fn main() {
    let mut champion = utils::create_champion_by_name("aatrox");
    let test_on_hit_effect_phys = effects::LimitedUseOnHitEffect::new("test", 10.0, effects::DamageType::Physical, 1, Duration::from_secs(10), true);
    let test_on_hit_effect_magic = effects::LimitedUseOnHitEffect::new("test2", 20.0, effects::DamageType::Magical, 1, Duration::from_secs(10), true);
    let test_on_hit_effect_true = effects::LimitedUseOnHitEffect::new("test3", 30.0, effects::DamageType::True, 1, Duration::from_secs(10), true);

    champion.add_friendly_limited_use_on_hit_effect(test_on_hit_effect_phys);
    champion.add_friendly_limited_use_on_hit_effect(test_on_hit_effect_magic);
    champion.add_friendly_limited_use_on_hit_effect(test_on_hit_effect_true);
    champion.set_level(1);

    let mut dummy = utils::create_champion_by_name("dummy");

    println!("Champion: {}, {}", champion.name, champion.health);

    dummy.take_auto_attack_damage(&mut champion);
    dummy.take_auto_attack_damage(&mut champion);

    let mut champion1 = utils::create_champion_by_name("aatrox");
    let champ1_build = Build::new(&champion1, vec![]);
    champion1.set_level(6);

    let champion2 = utils::create_champion_by_name("aatrox");
    let champ2_build = Build::new(&champion2, vec![]);

    let mut scenario = Scenario::new(0, Duration::from_secs_f32(0.5), champ1_build, champ2_build);
    scenario.calculate_scenario();
}

// TODO: Runes!
