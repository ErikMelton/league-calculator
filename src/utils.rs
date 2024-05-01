use std::collections::HashMap;
use crate::champion::Champion;
use crate::champion::stats::ChampStats;

pub fn create_champion_by_name(name: &str) -> Champion {
    let lower_name = name.to_lowercase();

    // TODO: Load these base values from a file or API
    match lower_name.as_str() {
        "aatrox" => Champion {
            name: String::from("Aatrox"),
            level: 1,
            friendly_limited_use_on_hit_effects: HashMap::new(),
            friendly_duration_on_hit_effects: HashMap::new(),
            friendly_stacking_on_hit_effects: HashMap::new(),
            enemy_dot_on_hit_effects: HashMap::new(),
            enemy_stacking_on_hit_effects: HashMap::new(),
            champ_stats: ChampStats {
                base_health: 685.0,
                base_health_growth: 114.0,
                health: 685.0,
                base_hp5: 3.0,
                base_hp5_growth: 1.0,
                hp5: 3,
                base_resource: 0.0,
                base_resource_growth: 0.0,
                resource: 0,
                base_rp5: 0.0,
                base_rp5_growth: 0.0,
                rp5: 0,
                base_ad: 60.0,
                base_ad_growth: 5.0,
                ad: 60,
                base_as: 0.651,
                base_as_growth_percent: 0.025,
                attack_windup: 0.23384,
                as_: 0.651,
                as_ratio: 0.651,
                base_armor: 38.0,
                base_armor_growth: 4.45,
                armor: 38.0,
                bonus_armor: 0.0,
                base_mr: 32.0,
                base_mr_growth: 2.05,
                mr: 32.0,
                bonus_mr: 0.0,
                base_range: 175,
                range: 175,
                base_ms: 345,
                ms: 345,
                base_crit: 0.0,
                crit: 0.50,
                bonus_crit_percent: 0.0,
                shield_amount: 0.0,
                magic_shield_amount: 0.0,
                physical_shield_amount: 0.0,
                mr_pen: 0.0,
                flat_mr_reduction: 0.0,
                percent_mr_reduction: 0.0,
                percent_mr_pen: 0.0,
                lethality: 0.0,
                percent_bonus_armor_pen: 0.0,
                armor_reduction: 0.0,
                percent_armor_reduction: 0.0,
                life_steal: 0,
                spell_vamp: 0,
                tenacity: 0,
            },
        },
        "test-bruiser" => Champion {
            name: String::from("Test Bruiser"),
            level: 1,
            friendly_limited_use_on_hit_effects: HashMap::new(),
            friendly_duration_on_hit_effects: HashMap::new(),
            friendly_stacking_on_hit_effects: HashMap::new(),
            enemy_dot_on_hit_effects: HashMap::new(),
            enemy_stacking_on_hit_effects: HashMap::new(),
            champ_stats: ChampStats {
                base_health: 685.0,
                base_health_growth: 114.0,
                health: 685.0,
                base_hp5: 3.0,
                base_hp5_growth: 1.0,
                hp5: 3,
                base_resource: 0.0,
                base_resource_growth: 0.0,
                resource: 0,
                base_rp5: 0.0,
                base_rp5_growth: 0.0,
                rp5: 0,
                base_ad: 60.0,
                base_ad_growth: 5.0,
                ad: 60,
                base_as: 0.651,
                base_as_growth_percent: 0.025,
                attack_windup: 0.23384,
                as_: 0.651,
                as_ratio: 0.651,
                base_armor: 38.0,
                base_armor_growth: 4.45,
                armor: 38.0,
                bonus_armor: 0.0,
                base_mr: 32.0,
                base_mr_growth: 2.05,
                mr: 32.0,
                bonus_mr: 0.0,
                base_range: 175,
                range: 175,
                base_ms: 345,
                ms: 345,
                base_crit: 0.0,
                crit: 0.00,
                bonus_crit_percent: 0.0,
                shield_amount: 0.0,
                magic_shield_amount: 0.0,
                physical_shield_amount: 0.0,
                mr_pen: 0.0,
                flat_mr_reduction: 0.0,
                percent_mr_reduction: 0.0,
                percent_mr_pen: 0.0,
                lethality: 0.0,
                percent_bonus_armor_pen: 0.0,
                armor_reduction: 0.0,
                percent_armor_reduction: 0.0,
                life_steal: 0,
                spell_vamp: 0,
                tenacity: 0,
            },
        },
        "dummy" => Champion {
            name: String::from("Dummy"),
            level: 1,
            friendly_limited_use_on_hit_effects: HashMap::new(),
            friendly_duration_on_hit_effects: HashMap::new(),
            friendly_stacking_on_hit_effects: HashMap::new(),
            enemy_dot_on_hit_effects: HashMap::new(),
            enemy_stacking_on_hit_effects: HashMap::new(),
            champ_stats: ChampStats {
                base_health: 10000.0,
                base_health_growth: 0.0,
                health: 10000.0,
                base_hp5: 0.0,
                base_hp5_growth: 0.0,
                hp5: 0,
                base_resource: 0.0,
                base_resource_growth: 0.0,
                resource: 0,
                base_rp5: 0.0,
                base_rp5_growth: 0.0,
                rp5: 0,
                base_ad: 0.0,
                base_ad_growth: 0.0,
                ad: 0,
                base_as: 0.0,
                base_as_growth_percent: 0.0,
                attack_windup: 0.0,
                as_: 0.0,
                as_ratio: 0.0,
                base_armor: 0.0,
                base_armor_growth: 0.0,
                armor: 0.0,
                bonus_armor: 0.0,
                base_mr: 0.0,
                base_mr_growth: 0.0,
                mr: 0.0,
                bonus_mr: 0.0,
                base_range: 0,
                range: 0,
                base_ms: 0,
                ms: 0,
                base_crit: 0.0,
                crit: 0.0,
                bonus_crit_percent: 0.0,
                shield_amount: 0.0,
                magic_shield_amount: 0.0,
                physical_shield_amount: 0.0,
                mr_pen: 0.0,
                flat_mr_reduction: 0.0,
                percent_mr_reduction: 0.0,
                percent_mr_pen: 0.0,
                lethality: 0.0,
                percent_bonus_armor_pen: 0.0,
                armor_reduction: 0.0,
                percent_armor_reduction: 0.0,
                life_steal: 0,
                spell_vamp: 0,
                tenacity: 0,
            },
        },
        _ => {
            panic!("Champion not found: {}", name);
        }
    }
}