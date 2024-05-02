pub(crate) mod stats;

use std::collections::HashMap;
use std::time::Duration;
use rand::Rng;
use crate::champion::stats::{calculate_crit_damage_multiplier_from_target, ChampStats};
use crate::constants::TICK_SECOND;

use crate::damage::Damage;
use crate::effects::{DamageType, DoTEffect, LimitedUseOnHitEffect, StackingOnHitEffect};

#[derive(Clone)]
pub struct Champion {
    pub(crate) name: String,
    pub(crate) level: i32,
    pub(crate) champ_stats: ChampStats,
    pub(crate) friendly_limited_use_on_hit_effects: HashMap<String, LimitedUseOnHitEffect>,
    pub(crate) friendly_duration_on_hit_effects: HashMap<String, DoTEffect>,
    pub(crate) friendly_stacking_on_hit_effects: HashMap<String, StackingOnHitEffect>,
    pub(crate) enemy_dot_on_hit_effects: HashMap<String, DoTEffect>,
    pub(crate) enemy_stacking_on_hit_effects: HashMap<String, StackingOnHitEffect>,
}

impl Champion {
    /// Set the level of the champion. This will recalculate the base stats of the champion based on
    /// the level supplied.
    pub fn set_level(&mut self, level: i32) {
        self.level = level;
        self.champ_stats.calculate_stats_from_level(level);
    }

    pub fn take_auto_attack_damage(&mut self, _source: &mut Champion) -> Damage {
        let effective_armor = self.champ_stats.calculate_armor_reduction(&mut _source.champ_stats);
        let effective_mr = self.champ_stats.calculate_magic_resist_reduction(&mut _source.champ_stats);

        let mut aa_damage = self.calculate_physical_damage_taken_from_aa(effective_armor, &mut _source.champ_stats);

        let on_hit_damage_pre_mit = self.calculate_on_hit_damage(_source);
        let mut on_hit_damage = Damage::new(0.0, 0.0, 0.0);
        on_hit_damage.physical_component = self.calculate_physical_damage_taken(effective_armor, on_hit_damage_pre_mit.physical_component);
        on_hit_damage.magical_component = self.calculate_magical_damage_taken(effective_mr, on_hit_damage_pre_mit.magical_component);
        on_hit_damage.true_component = on_hit_damage_pre_mit.true_component;

        aa_damage += on_hit_damage;

        self.take_damage(aa_damage);

        _source.decrement_limited_use_on_hit_effects();

        println!("Remaining health: {}", self.champ_stats.health);

        aa_damage
    }

    pub fn apply_enemy_dot_on_hit_effect(&mut self, effect: DoTEffect) {
        let existing_effect = self.enemy_dot_on_hit_effects.get_mut(&effect.id);

        match existing_effect {
            Some(existing_effect) => {
                existing_effect.damage_time_left = effect.damage_time_left;
            }
            None => {
                self.enemy_dot_on_hit_effects.insert(effect.id.to_string(), effect);
            }
        }
    }

    pub fn apply_enemy_stacking_on_hit_effect(&mut self, mut effect: StackingOnHitEffect) {
        let existing_effect = self.enemy_stacking_on_hit_effects.get_mut(&effect.id);

        match existing_effect {
            Some(existing_effect) => {
                if existing_effect.current_stacks < effect.max_stacks {
                    existing_effect.current_stacks += 1;
                }
            }
            None => {
                effect.current_stacks = 1;
                self.enemy_stacking_on_hit_effects.insert(effect.id.to_string(), effect);
            }
        }
    }

    pub fn calculate_and_apply_dot_effects(&mut self, tick: i32) -> Damage {
        let mut total_dot_damage = Damage::new(0.0, 0.0, 0.0);
        let mut dots_to_take = Vec::new();

        for (id, effect) in self.enemy_dot_on_hit_effects.iter_mut() {
            if tick % &effect.tick_rate == 0 {
                dots_to_take.push(id.to_string());
            }

            effect.effect_time_left -= Duration::from_secs_f32(TICK_SECOND);
        }

        for id in dots_to_take {
            total_dot_damage += self.take_dot_damage(&id);
        }

        // TODO: Apply armor reduction and other stat burn effects

        total_dot_damage
    }

     fn take_dot_damage(&mut self, id: &String) -> Damage {
        // TODO: Take into consideration item changes in armor + bonus mr (?)
        let effective_armor = self.champ_stats.armor + self.champ_stats.bonus_armor;
        let effective_mr = self.champ_stats.mr + self.champ_stats.bonus_mr;

        let mut dot_damage = Damage::new(0.0, 0.0, 0.0);

        let effect = self.enemy_dot_on_hit_effects.get(id);

        match effect {
            Some(effect) => {
                match effect.damage_type {
                    DamageType::Physical => {
                        dot_damage.physical_component = self.calculate_physical_damage_taken(effective_armor, effect.damage_over_time);
                    }
                    DamageType::Magical => {
                        dot_damage.magical_component = self.calculate_magical_damage_taken(effective_mr, effect.damage_over_time);
                    }
                    DamageType::True => {
                        dot_damage.true_component += effect.damage_over_time;
                    }
                }
            }
            None => {
                return dot_damage;
            }
        }

        self.take_damage(dot_damage);

        dot_damage
    }

    pub fn add_friendly_limited_use_on_hit_effect(&mut self, effect: LimitedUseOnHitEffect) {
        if self.friendly_limited_use_on_hit_effects.get(&effect.id).is_some() {
            return;
        }

        self.friendly_limited_use_on_hit_effects.insert(effect.id.to_string(), effect);
    }

    pub fn add_friendly_duration_on_hit_effect(&mut self, effect: DoTEffect) {
        if self.friendly_duration_on_hit_effects.get(&effect.id).is_some() {
            return;
        }

        self.friendly_duration_on_hit_effects.insert(effect.id.to_string(), effect);
    }

    pub fn add_friendly_stacking_on_hit_effect(&mut self, effect: StackingOnHitEffect) {
        if self.friendly_stacking_on_hit_effects.get(&effect.id).is_some() {
            return;
        }

        self.friendly_stacking_on_hit_effects.insert(effect.id.to_string(), effect);
    }

    pub fn remove_on_hit_effect(&mut self, id: &str) {
        self.friendly_limited_use_on_hit_effects.remove(id);
        self.friendly_duration_on_hit_effects.remove(id);
        self.friendly_stacking_on_hit_effects.remove(id);
    }

    fn calculate_on_hit_damage(&mut self, _source: &Champion) -> Damage {
        let mut damage = Damage::new(0.0, 0.0, 0.0);

        for (_, effect) in &_source.friendly_limited_use_on_hit_effects {
            if effect.num_uses > 0 {
                match effect.damage_type {
                    DamageType::Physical => {
                        damage.physical_component += effect.damage;
                    }
                    DamageType::Magical => {
                        damage.magical_component += effect.damage;
                    }
                    DamageType::True => {
                        damage.true_component += effect.damage;
                    }
                }

            }
        }

        damage
    }

    fn decrement_limited_use_on_hit_effects(&mut self) {
        for (_, effect) in &mut self.friendly_limited_use_on_hit_effects {
            if effect.num_uses > 0 {
                effect.num_uses -= 1;
            }
        }
    }

    pub fn decrement_own_effect_time_left(&mut self) {
        let mut duration_effects_to_remove = Vec::new();
        let mut stacking_effects_to_remove = Vec::new();

        for (id, effect) in self.friendly_duration_on_hit_effects.iter_mut() {
            if effect.effect_time_left <= Duration::from_secs(0) {
                duration_effects_to_remove.push(id.to_string());
                continue;
            }

            if effect.finite_time_left {
                effect.reduce_effect_time_left(Duration::from_secs_f32(TICK_SECOND));
            }
        }

        for id in duration_effects_to_remove {
            self.friendly_duration_on_hit_effects.remove(&id);
        }

        for (id, effect) in self.friendly_stacking_on_hit_effects.iter_mut() {
            if effect.effect_time_left <= Duration::from_secs(0) {
                stacking_effects_to_remove.push(id.to_string());
                continue;
            }

            if effect.finite_time_left {
                effect.reduce_effect_time_left(Duration::from_secs_f32(TICK_SECOND));
            }
        }

        for id in stacking_effects_to_remove {
            self.friendly_stacking_on_hit_effects.remove(&id);
        }
    }

    pub fn decrement_enemy_effect_time_left(&mut self) {
        let mut duration_effects_to_remove = Vec::new();
        let mut stacking_effects_to_remove = Vec::new();

        for (id, effect) in self.enemy_dot_on_hit_effects.iter_mut() {
            if effect.effect_time_left <= Duration::from_secs(0) {
                duration_effects_to_remove.push(id.to_string());
                continue;
            }

            if effect.finite_time_left {
                effect.reduce_effect_time_left(Duration::from_secs_f32(TICK_SECOND));
            }
        }

        for id in duration_effects_to_remove {
            self.enemy_dot_on_hit_effects.remove(&id);
        }

        for (id, effect) in self.enemy_stacking_on_hit_effects.iter_mut() {
            if effect.effect_time_left <= Duration::from_secs(0) {
                stacking_effects_to_remove.push(id.to_string());
                continue;
            }

            if effect.finite_time_left {
                effect.reduce_effect_time_left(Duration::from_secs_f32(TICK_SECOND));
            }
        }

        for id in stacking_effects_to_remove {
            self.enemy_stacking_on_hit_effects.remove(&id);
        }
    }

    fn calculate_physical_damage_taken_from_aa(&self, effective_armor: f32, _source: &ChampStats) -> Damage {
        let mut damage = Damage::new(_source.ad as f32, 0.0, 0.0);

        // Simplified crit damage calculation; we do not apply smoothing to compensate for "streaks"
        let crit_damage_multiplier = calculate_crit_damage_multiplier_from_target(_source);

        if _source.crit > 0.0 {
            let mut rng = rand::thread_rng();
            let probability = rng.gen::<f32>();

            if probability <= _source.crit {
                damage.physical_component = damage.physical_component * crit_damage_multiplier;
            }
        }

        damage.physical_component = self.calculate_physical_damage_taken(effective_armor, damage.physical_component);

        damage
    }

    fn calculate_physical_damage_taken(&self, effective_armor: f32, damage: f32) -> f32 {
        return if effective_armor >= 0.0 {
            (100.0 / (100.0 + effective_armor)) * damage
        } else {
            (2.0 - 100.0 / (100.0 - effective_armor)) * damage
        }
    }

    fn calculate_magical_damage_taken(&self, effective_mr: f32, damage: f32) -> f32 {
        return if effective_mr >= 0.0 {
            (100.0 / (100.0 + effective_mr)) * damage
        } else {
            (2.0 - 100.0 / (100.0 - effective_mr)) * damage
        }
    }

    fn take_damage(&mut self, mut damage: Damage) {
        self.take_physical_damage(&mut damage);
        self.take_magical_damage(&mut damage);
        self.take_true_damage(&mut damage);
    }

    fn take_physical_damage(&mut self, damage: &mut Damage) {
        let stats = &mut self.champ_stats;
        // Take away from the physical shield first
        if stats.physical_shield_amount > 0.0 {
            let old_physical_shield_amount = stats.physical_shield_amount;

            stats.physical_shield_amount = stats.physical_shield_amount - damage.physical_component;

            damage.reduce_physical_damage(old_physical_shield_amount);

            if stats.physical_shield_amount < 0.0 {
                stats.physical_shield_amount = 0.0;
            }
        }

        // Take away from the shield second
        if stats.shield_amount > 0.0 && damage.physical_component > 0.0 {
            let old_shield_amount = stats.shield_amount;

            stats.shield_amount = stats.shield_amount - damage.physical_component;

            damage.reduce_physical_damage(old_shield_amount);

            if stats.shield_amount < 0.0 {
                stats.shield_amount = 0.0;
            }
        }

        // Take away from the health last
        if damage.physical_component > 0.0 {
            stats.health = (stats.health - damage.physical_component).round();

            if stats.health < 0.0 {
                stats.health = 0.0;
            }

            damage.reduce_physical_damage(damage.physical_component);
        }
    }

    fn take_magical_damage(&mut self, damage: &mut Damage) {
        let stats = &mut self.champ_stats;
        // Take away from the magic shield first
        if stats.magic_shield_amount > 0.0 {
            let old_magic_shield_amount = stats.magic_shield_amount;
            stats.magic_shield_amount = stats.magic_shield_amount - damage.magical_component;
            damage.reduce_magical_damage(old_magic_shield_amount);

            if stats.magic_shield_amount < 0.0 {
                stats.magic_shield_amount = 0.0;
            }
        }

        // Take away from the shield second
        if stats.shield_amount > 0.0 && damage.magical_component > 0.0 {
            let old_shield_amount = stats.shield_amount;
            stats.shield_amount = stats.shield_amount - damage.magical_component;
            damage.reduce_magical_damage(old_shield_amount);

            if stats.shield_amount < 0.0 {
                stats.shield_amount = 0.0;
            }
        }

        // Take away from the health last
        if damage.magical_component > 0.0 {
            stats.health = (stats.health - damage.magical_component).round();
            damage.reduce_magical_damage(damage.magical_component);

            if stats.health < 0.0 {
                stats.health = 0.0;
            }
        }
    }

    fn take_true_damage(&mut self, damage: &mut Damage) {
        let stats = &mut self.champ_stats;

        // Take away from the shield first
        if stats.shield_amount > 0.0 {
            let old_shield_amount = stats.shield_amount;
            stats.shield_amount = stats.shield_amount - damage.true_component;
            damage.reduce_true_damage(old_shield_amount);

            if stats.shield_amount < 0.0 {
                stats.shield_amount = 0.0;
            }
        }

        // Take away from the health last
        if damage.true_component > 0.0 {
            stats.health = (stats.health - damage.true_component).round();
            damage.reduce_true_damage(damage.true_component);

            if stats.health < 0.0 {
                stats.health = 0.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::effects::{DamageType, DoTEffect, LimitedUseOnHitEffect, StackingOnHitEffect, EffectTickRate};
    use crate::utils::create_champion_by_name;

    #[test]
    fn test_set_level() {
        let mut champion = create_champion_by_name("test-bruiser");

        assert_eq!(champion.champ_stats.health, 685.0);
        assert_eq!(champion.champ_stats.hp5, 3);
        assert_eq!(champion.champ_stats.resource, 0);
        assert_eq!(champion.champ_stats.rp5, 0);
        assert_eq!(champion.champ_stats.ad, 60);
        assert_eq!(champion.champ_stats.as_, 0.651);
        assert_eq!(champion.champ_stats.armor, 38.0);
        assert_eq!(champion.champ_stats.mr, 32.0);

        champion.set_level(2);

        assert_eq!(champion.champ_stats.health, 767.0);
        assert_eq!(champion.champ_stats.hp5, 4);
        assert_eq!(champion.champ_stats.resource, 0);
        assert_eq!(champion.champ_stats.rp5, 0);
        assert_eq!(champion.champ_stats.ad, 64);
        assert_eq!(champion.champ_stats.as_, 0.825468063);
        assert_eq!(champion.champ_stats.armor, 41.0);
        assert_eq!(champion.champ_stats.mr, 33.0);
    }

    #[test]
    fn test_take_true_damage() {
        let mut champion = create_champion_by_name("test-bruiser");

        champion.champ_stats.shield_amount = 100.0;
        champion.champ_stats.magic_shield_amount = 100.0;
        champion.champ_stats.physical_shield_amount = 100.0;

        let mut damage = crate::damage::Damage::new(0.0, 0.0, 100.0);

        champion.take_true_damage(&mut damage);

        assert_eq!(champion.champ_stats.shield_amount, 0.0);
        assert_eq!(champion.champ_stats.health, 685.0);
        assert_eq!(champion.champ_stats.magic_shield_amount, 100.0);
        assert_eq!(champion.champ_stats.physical_shield_amount, 100.0);

        damage.true_component = 100.0;

        champion.take_true_damage(&mut damage);

        assert_eq!(champion.champ_stats.shield_amount, 0.0);
        assert_eq!(champion.champ_stats.health, 585.0);

        champion.champ_stats.shield_amount = 99.0;
        damage.true_component = 100.0;

        champion.take_true_damage(&mut damage);

        assert_eq!(champion.champ_stats.shield_amount, 0.0);
        assert_eq!(champion.champ_stats.health, 584.0);

        damage.true_component = 600.0;

        champion.take_true_damage(&mut damage);

        assert_eq!(champion.champ_stats.shield_amount, 0.0);
        assert_eq!(champion.champ_stats.health, 0.0);
    }

    #[test]
    fn take_magic_damage() {
        let mut champion = create_champion_by_name("test-bruiser");

        champion.champ_stats.shield_amount = 100.0;
        champion.champ_stats.magic_shield_amount = 100.0;
        champion.champ_stats.physical_shield_amount = 100.0;

        let mut damage = crate::damage::Damage::new(0.0, 100.0, 0.0);

        champion.take_magical_damage(&mut damage);

        assert_eq!(champion.champ_stats.shield_amount, 100.0);
        assert_eq!(champion.champ_stats.health, 685.0);
        assert_eq!(champion.champ_stats.magic_shield_amount, 0.0);
        assert_eq!(champion.champ_stats.physical_shield_amount, 100.0);

        damage.magical_component = 100.0;

        champion.take_magical_damage(&mut damage);

        assert_eq!(champion.champ_stats.shield_amount, 0.0);
        assert_eq!(champion.champ_stats.health, 685.0);

        champion.champ_stats.shield_amount = 99.0;
        damage.magical_component = 100.0;

        champion.take_magical_damage(&mut damage);

        assert_eq!(champion.champ_stats.shield_amount, 0.0);
        assert_eq!(champion.champ_stats.health, 684.0);

        damage.magical_component = 700.0;

        champion.take_magical_damage(&mut damage);

        assert_eq!(champion.champ_stats.shield_amount, 0.0);
        assert_eq!(champion.champ_stats.health, 0.0);
    }

    #[test]
    fn take_physical_damage() {
        let mut champion = create_champion_by_name("test-bruiser");

        champion.champ_stats.shield_amount = 100.0;
        champion.champ_stats.magic_shield_amount = 100.0;
        champion.champ_stats.physical_shield_amount = 100.0;

        let mut damage = crate::damage::Damage::new(100.0, 0.0, 0.0);

        champion.take_physical_damage(&mut damage);

        assert_eq!(champion.champ_stats.shield_amount, 100.0);
        assert_eq!(champion.champ_stats.health, 685.0);
        assert_eq!(champion.champ_stats.magic_shield_amount, 100.0);
        assert_eq!(champion.champ_stats.physical_shield_amount, 0.0);

        damage.physical_component = 100.0;

        champion.take_physical_damage(&mut damage);

        assert_eq!(champion.champ_stats.shield_amount, 0.0);
        assert_eq!(champion.champ_stats.health, 685.0);

        champion.champ_stats.shield_amount = 99.0;
        damage.physical_component = 100.0;

        champion.take_physical_damage(&mut damage);

        assert_eq!(champion.champ_stats.shield_amount, 0.0);
        assert_eq!(champion.champ_stats.health, 684.0);

        damage.physical_component = 700.0;

        champion.take_physical_damage(&mut damage);

        assert_eq!(champion.champ_stats.shield_amount, 0.0);
        assert_eq!(champion.champ_stats.health, 0.0);
    }

    #[test]
    fn test_calculate_physical_damage_taken() {
        let champion = create_champion_by_name("test-bruiser");
        let effective_armor = 25.0;
        let damage = 1000.0;

        assert_eq!(champion.calculate_physical_damage_taken(effective_armor, damage), 800.0);

        let effective_armor = 100.0;
        assert_eq!(champion.calculate_physical_damage_taken(effective_armor, damage), 500.0);

        let effective_armor = 200.0;
        assert_eq!(champion.calculate_physical_damage_taken(effective_armor, damage), 333.33334);

        let effective_armor = -100.0;
        assert_eq!(champion.calculate_physical_damage_taken(effective_armor, damage), 1500.0);

        let effective_armor = 0.0;
        assert_eq!(champion.calculate_physical_damage_taken(effective_armor, damage), 1000.0);
    }

    #[test]
    fn test_calculate_magic_damage_taken() {
        let champion = create_champion_by_name("test-bruiser");
        let effective_mr = 25.0;
        let damage = 1000.0;

        assert_eq!(champion.calculate_magical_damage_taken(effective_mr, damage), 800.0);

        let effective_mr = 100.0;
        assert_eq!(champion.calculate_magical_damage_taken(effective_mr, damage), 500.0);

        let effective_mr = 200.0;
        assert_eq!(champion.calculate_magical_damage_taken(effective_mr, damage), 333.33334);

        let effective_mr = -100.0;
        assert_eq!(champion.calculate_magical_damage_taken(effective_mr, damage), 1500.0);

        let effective_mr = 0.0;
        assert_eq!(champion.calculate_magical_damage_taken(effective_mr, damage), 1000.0);
    }

    #[test]
    fn test_take_damage() {
        let mut champion = create_champion_by_name("test-bruiser");

        let damage1 = crate::damage::Damage::new(100.0, 100.0, 100.0);
        let damage2 = crate::damage::Damage::new(100.0, 100.0, 100.0);

        champion.take_damage(damage1);

        assert_eq!(champion.champ_stats.health, 385.0);
        assert_eq!(champion.champ_stats.shield_amount, 0.0);
        assert_eq!(champion.champ_stats.magic_shield_amount, 0.0);
        assert_eq!(champion.champ_stats.physical_shield_amount, 0.0);

        champion.champ_stats.shield_amount = 100.0;
        champion.champ_stats.magic_shield_amount = 100.0;
        champion.champ_stats.physical_shield_amount = 100.0;

        champion.take_damage(damage2);

        assert_eq!(champion.champ_stats.health, 385.0);
        assert_eq!(champion.champ_stats.shield_amount, 0.0);
        assert_eq!(champion.champ_stats.magic_shield_amount, 0.0);
        assert_eq!(champion.champ_stats.physical_shield_amount, 0.0);
    }

    #[test]
    fn test_add_limited_use_on_hit_effect() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = LimitedUseOnHitEffect::new("test", 100.0, DamageType::Physical, 1, Duration::from_secs(10), true);

        champion.add_friendly_limited_use_on_hit_effect(effect);

        assert_eq!(champion.friendly_limited_use_on_hit_effects.len(), 1);
    }

    #[test]
    fn test_add_duplicate_limited_use_on_hit_effect() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = LimitedUseOnHitEffect::new("test", 100.0, DamageType::Physical, 1, Duration::from_secs(10), true);
        let effect2 = LimitedUseOnHitEffect::new("test", 100.0, DamageType::Physical, 1, Duration::from_secs(10), true);

        champion.add_friendly_limited_use_on_hit_effect(effect);
        champion.add_friendly_limited_use_on_hit_effect(effect2);

        assert_eq!(champion.friendly_limited_use_on_hit_effects.len(), 1);
    }

    #[test]
    fn test_add_duration_on_hit_effect() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = DoTEffect::new("test", 100.0, DamageType::Physical, Duration::new(2, 0), EffectTickRate::PerSecond, Duration::new(10, 0), true);

        champion.add_friendly_duration_on_hit_effect(effect);

        assert_eq!(champion.friendly_duration_on_hit_effects.len(), 1);
    }

    #[test]
    fn test_add_duplicate_duration_on_hit_effect() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = DoTEffect::new("test", 100.0, DamageType::Physical, Duration::new(2, 0), EffectTickRate::PerSecond, Duration::new(10, 0), true);
        let effect2 = DoTEffect::new("test", 100.0, DamageType::Physical, Duration::new(2, 0), EffectTickRate::PerSecond, Duration::new(10, 0), true);

        champion.add_friendly_duration_on_hit_effect(effect);
        champion.add_friendly_duration_on_hit_effect(effect2);

        assert_eq!(champion.friendly_duration_on_hit_effects.len(), 1);
    }

    #[test]
    fn test_add_stacking_on_hit_effect() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = StackingOnHitEffect::new("test", 5.0, DamageType::Physical, 100, Duration::new(2, 0), Duration::from_secs(10), true);

        champion.add_friendly_stacking_on_hit_effect(effect);

        assert_eq!(champion.friendly_stacking_on_hit_effects.len(), 1);
    }

    #[test]
    fn test_add_duplicate_stacking_on_hit_effect() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = StackingOnHitEffect::new("test", 5.0, DamageType::Physical, 100, Duration::new(2, 0), Duration::from_secs(10), true);
        let effect2 = StackingOnHitEffect::new("test", 5.0, DamageType::Physical, 100, Duration::new(2, 0), Duration::from_secs(10), true);

        champion.add_friendly_stacking_on_hit_effect(effect);
        champion.add_friendly_stacking_on_hit_effect(effect2);

        assert_eq!(champion.friendly_stacking_on_hit_effects.len(), 1);
    }

    #[test]
    fn test_remove_limited_use_on_hit_effects() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = LimitedUseOnHitEffect::new("test", 100.0, DamageType::Physical, 1, Duration::from_secs(10), true);
        let effect2 = LimitedUseOnHitEffect::new("test2", 100.0, DamageType::Physical, 1, Duration::from_secs(10), true);
        let effect3 = LimitedUseOnHitEffect::new("test3", 100.0, DamageType::Physical, 1, Duration::from_secs(10), true);

        champion.add_friendly_limited_use_on_hit_effect(effect);
        champion.add_friendly_limited_use_on_hit_effect(effect2);
        champion.add_friendly_limited_use_on_hit_effect(effect3);

        champion.remove_on_hit_effect("test2");

        assert_eq!(champion.friendly_limited_use_on_hit_effects.len(), 2);
        assert_eq!(champion.friendly_limited_use_on_hit_effects.get("test2"), None);
        assert!(champion.friendly_limited_use_on_hit_effects.get("test").is_some());
        assert!(champion.friendly_limited_use_on_hit_effects.get("test3").is_some());
    }

    #[test]
    fn test_remove_on_hit_effects_duration() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = DoTEffect::new("test", 100.0, DamageType::Physical, Duration::new(2, 0), EffectTickRate::PerSecond, Duration::new(10, 0), true);
        let effect2 = DoTEffect::new("test2", 100.0, DamageType::Physical, Duration::new(2, 0), EffectTickRate::PerSecond, Duration::new(10, 0), true);
        let effect3 = DoTEffect::new("test3", 100.0, DamageType::Physical, Duration::new(2, 0), EffectTickRate::PerSecond, Duration::new(10, 0), true);

        champion.add_friendly_duration_on_hit_effect(effect);
        champion.add_friendly_duration_on_hit_effect(effect2);
        champion.add_friendly_duration_on_hit_effect(effect3);

        champion.remove_on_hit_effect("test2");

        assert_eq!(champion.friendly_duration_on_hit_effects.len(), 2);
        assert_eq!(champion.friendly_duration_on_hit_effects.get("test2"), None);
        assert!(champion.friendly_duration_on_hit_effects.get("test").is_some());
        assert!(champion.friendly_duration_on_hit_effects.get("test3").is_some());
    }

    #[test]
    fn test_remove_on_hit_effects_stacking() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = StackingOnHitEffect::new("test", 5.0, DamageType::Physical, 100, Duration::new(2, 0), Duration::from_secs(10), true);
        let effect2 = StackingOnHitEffect::new("test2", 5.0, DamageType::Physical, 100, Duration::new(2, 0), Duration::from_secs(10), true);
        let effect3 = StackingOnHitEffect::new("test3", 5.0, DamageType::Physical, 100, Duration::new(2, 0), Duration::from_secs(10), true);

        champion.add_friendly_stacking_on_hit_effect(effect);
        champion.add_friendly_stacking_on_hit_effect(effect2);
        champion.add_friendly_stacking_on_hit_effect(effect3);

        champion.remove_on_hit_effect("test2");

        assert_eq!(champion.friendly_stacking_on_hit_effects.len(), 2);
        assert_eq!(champion.friendly_stacking_on_hit_effects.get("test2"), None);
        assert!(champion.friendly_stacking_on_hit_effects.get("test").is_some());
        assert!(champion.friendly_stacking_on_hit_effects.get("test3").is_some());
    }

    #[test]
    fn test_decrement_limited_use_on_hit_effects() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = LimitedUseOnHitEffect::new("test", 100.0, DamageType::Physical, 1, Duration::from_secs(10), true);
        let effect2 = LimitedUseOnHitEffect::new("test2", 100.0, DamageType::Physical, 2, Duration::from_secs(10), true);
        let effect3 = LimitedUseOnHitEffect::new("test3", 100.0, DamageType::Physical, 0, Duration::from_secs(10), true);

        champion.add_friendly_limited_use_on_hit_effect(effect);
        champion.add_friendly_limited_use_on_hit_effect(effect2);
        champion.add_friendly_limited_use_on_hit_effect(effect3);

        champion.decrement_limited_use_on_hit_effects();

        assert_eq!(champion.friendly_limited_use_on_hit_effects.len(), 3);
        assert_eq!(champion.friendly_limited_use_on_hit_effects.get("test").unwrap().num_uses, 0);
        assert_eq!(champion.friendly_limited_use_on_hit_effects.get("test2").unwrap().num_uses, 1);
        assert_eq!(champion.friendly_limited_use_on_hit_effects.get("test3").unwrap().num_uses, 0);
    }

    #[test]
    fn test_calculate_on_hit_damage() {
        let mut champion = create_champion_by_name("test-bruiser");
        let mut source = create_champion_by_name("test-bruiser");

        let effect = LimitedUseOnHitEffect::new("test", 100.0, DamageType::Physical, 1, Duration::from_secs(10), true);
        let effect2 = LimitedUseOnHitEffect::new("test2", 100.0, DamageType::Physical, 2, Duration::from_secs(10), true);
        let effect3 = LimitedUseOnHitEffect::new("test3", 100.0, DamageType::Physical, 0, Duration::from_secs(10), true);
        let effect4 = LimitedUseOnHitEffect::new("test4", 100.0, DamageType::Magical, 1, Duration::from_secs(10), true);
        let effect5 = LimitedUseOnHitEffect::new("test5", 100.0, DamageType::True, 1, Duration::from_secs(10), true);

        source.add_friendly_limited_use_on_hit_effect(effect);
        source.add_friendly_limited_use_on_hit_effect(effect2);
        source.add_friendly_limited_use_on_hit_effect(effect3);
        source.add_friendly_limited_use_on_hit_effect(effect4);
        source.add_friendly_limited_use_on_hit_effect(effect5);

        let damage = champion.calculate_on_hit_damage(&source);

        assert_eq!(damage.physical_component, 200.0);
        assert_eq!(damage.magical_component, 100.0);
        assert_eq!(damage.true_component, 100.0);
    }

    #[test]
    fn test_calculate_physical_damage_taken_from_aa_no_armor() {
        let champion = create_champion_by_name("test-bruiser");
        let source = create_champion_by_name("test-bruiser");

        let damage = champion.calculate_physical_damage_taken_from_aa(0.0, &source.champ_stats);

        assert_eq!(damage.physical_component, 60.0);
    }

    #[test]
    fn test_calculate_physical_damage_taken_from_aa_with_armor() {
        let champion = create_champion_by_name("test-bruiser");
        let source = create_champion_by_name("test-bruiser");

        let damage = champion.calculate_physical_damage_taken_from_aa(100.0, &source.champ_stats);

        assert_eq!(damage.physical_component, 30.0);
    }

    #[test]
    fn test_calculate_physical_damage_taken_from_aa_crit() {
        let champion = create_champion_by_name("test-bruiser");
        let mut source = create_champion_by_name("test-bruiser");

        source.champ_stats.crit = 1.0;

        let damage = champion.calculate_physical_damage_taken_from_aa(0.0, &source.champ_stats);

        assert_eq!(damage.physical_component, 105.0);
    }

    #[test]
    fn test_take_auto_attack_damage() {
        let mut champion = create_champion_by_name("test-bruiser");
        let mut source = create_champion_by_name("test-bruiser");

        source.champ_stats.ad = 100;
        source.champ_stats.crit = 1.0;

        champion.take_auto_attack_damage(&mut source);

        assert_eq!(champion.champ_stats.health, 558.0);
    }
}

#[cfg(test)]
mod struct_tests {
    #[test]
    fn test_clone_champion() {
        let champion = crate::utils::create_champion_by_name("test-bruiser");

        let clone = champion.clone();

        assert_eq!(champion.name, clone.name);
        assert_eq!(champion.level, clone.level);
        assert_eq!(champion.friendly_limited_use_on_hit_effects.len(), clone.friendly_limited_use_on_hit_effects.len());
        assert_eq!(champion.friendly_duration_on_hit_effects.len(), clone.friendly_duration_on_hit_effects.len());
    }
}

#[cfg(test)]
mod effect_tests {
    use std::time::Duration;
    use crate::effects::{DamageType, StackingOnHitEffect};
    use crate::utils::create_champion_by_name;

    #[test]
    fn test_apply_enemy_stacking_on_hit_effect() {
        let mut champion = create_champion_by_name("test-bruiser");
        let enemy = create_champion_by_name("test-bruiser");

        let effect = StackingOnHitEffect::new("test", 10.0, DamageType::Physical, 3, Duration::from_secs(5), Duration::from_secs(10), true);

        champion.apply_enemy_stacking_on_hit_effect(effect.clone());

        assert_eq!(champion.enemy_stacking_on_hit_effects.len(), 1);

        champion.apply_enemy_stacking_on_hit_effect(effect.clone());

        assert_eq!(champion.enemy_stacking_on_hit_effects.len(), 1);
        assert_eq!(champion.enemy_stacking_on_hit_effects.get("test").unwrap().current_stacks, 2);

        let effect2 = StackingOnHitEffect::new("test2", 10.0, DamageType::Physical, 1, Duration::from_secs(5), Duration::from_secs(10), true);

        champion.apply_enemy_stacking_on_hit_effect(effect2.clone());
        champion.apply_enemy_stacking_on_hit_effect(effect2.clone());

        assert_eq!(champion.enemy_stacking_on_hit_effects.len(), 2);
        assert_eq!(champion.enemy_stacking_on_hit_effects.get("test2").unwrap().current_stacks, 1);

        let effect3 = StackingOnHitEffect::new("test3", 10.0, DamageType::Physical, 1, Duration::from_secs(5), Duration::from_secs(10), true);

        champion.apply_enemy_stacking_on_hit_effect(effect3.clone());

        assert_eq!(champion.enemy_stacking_on_hit_effects.len(), 3);
        assert_eq!(champion.enemy_stacking_on_hit_effects.get("test3").unwrap().current_stacks, 1);
    }
}
