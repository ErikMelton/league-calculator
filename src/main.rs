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
    let test_on_hit_effect_phys = effects::LimitedUseOnHitEffect::new("test", 10.0, effects::DamageType::Physical, 1);
    let test_on_hit_effect_magic = effects::LimitedUseOnHitEffect::new("test2", 20.0, effects::DamageType::Magical, 1);
    let test_on_hit_effect_true = effects::LimitedUseOnHitEffect::new("test3", 30.0, effects::DamageType::True, 1);

    champion.add_limited_use_on_hit_effect(test_on_hit_effect_phys);
    champion.add_limited_use_on_hit_effect(test_on_hit_effect_magic);
    champion.add_limited_use_on_hit_effect(test_on_hit_effect_true);
    champion.set_level(1);

    let mut dummy = utils::create_champion_by_name("dummy");

    println!("Champion: {}, {}", champion.name, champion.health);

    dummy.take_auto_attack_damage(&mut champion);
    dummy.take_auto_attack_damage(&mut champion);
}

// TODO: Runes!
