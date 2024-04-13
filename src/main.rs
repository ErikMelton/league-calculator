mod item;
mod build;
mod scenario;
mod enhancements;
mod simulation;
mod champion;
mod damage;

fn main() {
    let mut champion = champion::create_champion_by_name("aatrox");
    let test_on_hit_effect = enhancements::LimitedUseOnHitEffect::new("test", 10.0, 0.0, enhancements::DamageType::Physical, 1);

    champion.add_limited_use_on_hit_effect(test_on_hit_effect);
    champion.set_level(1);

    let mut dummy = champion::create_champion_by_name("dummy");

    println!("Champion: {}, {}", champion.name, champion.health);

    dummy.take_auto_attack_damage(&mut champion);
}

// TODO: Runes!
