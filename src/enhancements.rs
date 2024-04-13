use std::time::Duration;

pub enum DamageType {
    Physical,
    Magical,
    True,
}

struct OnHitEffect {
    damage: f32,
    damage_over_time: f32,
    damage_type: DamageType,
    num_uses: i32,
    time_left: Duration,
}