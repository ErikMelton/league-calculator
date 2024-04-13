mod champion;
mod item;
mod build;
mod scenario;
mod enhancements;
mod simulation;

fn main() {
    let mut champion = champion::create_champion_by_name("aatrox");
    champion.set_level(1);

    let mut dummy = champion::create_champion_by_name("dummy");

    println!("Champion: {}, {}", champion.name, champion.health);

    dummy.take_auto_attack_damage( &champion);
}

// TODO: Runes!
