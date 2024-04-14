use rand::Rng;

use crate::damage::Damage;
use crate::effects::{DamageType, DoTEffect, LimitedUseOnHitEffect, StackingOnHitEffect};

pub struct Champion {
    pub(crate) name: String,
    pub(crate) level: i32,
    pub(crate) base_health: f32,
    pub(crate) base_health_growth: f32,
    pub(crate) health: f32,
    pub(crate) base_hp5: f32,
    pub(crate) base_hp5_growth: f32,
    pub(crate) hp5: i32,
    pub(crate) base_resource: f32,
    pub(crate) base_resource_growth: f32,
    pub(crate) resource: i32,
    pub(crate) base_rp5: f32,
    pub(crate) base_rp5_growth: f32,
    pub(crate) rp5: i32,
    pub(crate) base_ad: f32,
    pub(crate) base_ad_growth: f32,
    pub(crate) ad: i32,
    pub(crate) base_as: f32,
    pub(crate) base_as_growth_percent: f32,
    pub(crate) attack_windup: f32,
    pub(crate) as_: f32,
    pub(crate) as_ratio: f32,
    pub(crate) base_armor: f32,
    pub(crate) base_armor_growth: f32,
    pub(crate) armor: f32,
    pub(crate) bonus_armor: f32,
    pub(crate) base_mr: f32,
    pub(crate) base_mr_growth: f32,
    pub(crate) mr: f32,
    pub(crate) base_range: i32,
    pub(crate) range: i32,
    pub(crate) base_ms: i32,
    pub(crate) ms: i32,
    pub(crate) base_crit: f32,
    pub(crate) crit: f32,
    pub(crate) bonus_crit_percent: f32,
    // TODO: Shields only last for a certain amount of time, and some decay! Use a struct
    pub(crate) shield_amount: f32,
    pub(crate) magic_shield_amount: f32,
    pub(crate) physical_shield_amount: f32,
    pub(crate) mr_pen: f32,
    pub(crate) flat_mr_reduction: f32,
    pub(crate) percent_mr_reduction: f32,
    pub(crate) percent_mr_pen: f32,
    pub(crate) lethality: f32,
    pub(crate) percent_bonus_armor_pen: f32,
    pub(crate) armor_reduction: f32,
    pub(crate) percent_armor_reduction: f32,
    pub(crate) life_steal: i32,
    pub(crate) spell_vamp: i32,
    pub(crate) tenacity: i32,
    pub(crate) limited_use_on_hit_effects: Vec<LimitedUseOnHitEffect>,
    pub(crate) duration_on_hit_effects: Vec<DoTEffect>,
    pub(crate) stacking_on_hit_effects: Vec<StackingOnHitEffect>,
}

// TODO: Implement damage over time tick event
impl Champion {
    /// Set the level of the champion. This will recalculate the base stats of the champion based on
    /// the level supplied.
    pub fn set_level(&mut self, level: i32) {
        self.level = level;
        self.health = calculate_base_stat(self.base_health, 0.0, self.base_health_growth, level).round();
        self.hp5 = calculate_base_stat(self.base_hp5, 0.0, self.base_hp5_growth, level).round() as i32;
        self.resource = calculate_base_stat(self.base_resource, 0.0, self.base_resource_growth, level).round() as i32;
        self.rp5 = calculate_base_stat(self.base_rp5, 0.0, self.base_rp5_growth, level).round() as i32;
        self.ad = calculate_base_stat(self.base_ad, 0.0, self.base_ad_growth, level).round() as i32;
        self.as_ = calculate_attack_speed(self.base_as, self.as_ratio, self.base_as_growth_percent, 0.25, level);
        self.armor = calculate_base_stat(self.base_armor, 0.0, self.base_armor_growth, level).round();
        self.mr = calculate_base_stat(self.base_mr, 0.0, self.base_mr_growth, level).round();
    }

    pub fn take_auto_attack_damage(&mut self, _source: &mut Champion) {
        let effective_armor = self.calculate_armor_reduction(_source);
        let effective_mr = self.calculate_magic_resist_reduction(_source);

        let mut aa_damage = self.calculate_physical_damage_taken_from_aa(effective_armor, _source);

        let on_hit_damage_pre_mit = self.calculate_on_hit_damage(_source);
        let mut on_hit_damage = Damage::new(0.0, 0.0, 0.0);
        on_hit_damage.physical_component = self.calculate_physical_damage_taken(effective_armor, on_hit_damage_pre_mit.physical_component);
        on_hit_damage.magical_component = self.calculate_magical_damage_taken(effective_mr, on_hit_damage_pre_mit.magical_component);
        on_hit_damage.true_component = on_hit_damage_pre_mit.true_component;

        aa_damage += on_hit_damage;

        self.take_damage(aa_damage);

        _source.decrement_limited_use_on_hit_effects();

        println!("Remaining health: {}", self.health);
    }

    pub fn add_limited_use_on_hit_effect(&mut self, effect: LimitedUseOnHitEffect) {
        if self.limited_use_on_hit_effects.iter().any(|e| e.id == effect.id) {
            return;
        }

        self.limited_use_on_hit_effects.push(effect);
    }

    pub fn add_duration_on_hit_effect(&mut self, effect: DoTEffect) {
        if self.duration_on_hit_effects.iter().any(|e| e.id == effect.id) {
            return;
        }

        self.duration_on_hit_effects.push(effect);
    }

    pub fn add_stacking_on_hit_effect(&mut self, effect: StackingOnHitEffect) {
        if self.stacking_on_hit_effects.iter().any(|e| e.id == effect.id) {
            return;
        }

        self.stacking_on_hit_effects.push(effect);
    }

    pub fn remove_on_hit_effect(&mut self, id: &str) {
        self.limited_use_on_hit_effects.retain(|effect| effect.id != id);
        self.duration_on_hit_effects.retain(|effect| effect.id != id);
        self.stacking_on_hit_effects.retain(|effect| effect.id != id);
    }

    fn calculate_on_hit_damage(&mut self, _source: &Champion) -> Damage {
        let mut damage = Damage::new(0.0, 0.0, 0.0);

        for effect in &_source.limited_use_on_hit_effects {
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
        for effect in &mut self.limited_use_on_hit_effects {
            if effect.num_uses > 0 {
                effect.num_uses -= 1;
            }
        }
    }

    fn calculate_armor_reduction(&self, _source: &Champion) -> f32 {
        let mut armor = self.armor;
        let mut bonus_armor = self.bonus_armor;
        let total_armor = armor + bonus_armor;

        if total_armor == 0.0 {
            return 0.0;
        }

        let starting_prop = armor / total_armor;
        let bonus_prop = bonus_armor / total_armor;

        let armor_reduced = _source.armor_reduction * starting_prop;
        let armor_reduced_bonus = _source.armor_reduction * bonus_prop;

        armor = armor - armor_reduced;
        bonus_armor = bonus_armor - armor_reduced_bonus;

        if armor + bonus_armor > 0.0 {
            armor = armor * (1.0 - _source.percent_armor_reduction);
            bonus_armor = bonus_armor * (1.0 - _source.percent_armor_reduction);

            bonus_armor = bonus_armor * (1.0 - _source.percent_bonus_armor_pen);

            let total_armor = armor + bonus_armor;

            // Lethality
            if total_armor - _source.lethality < 0.0 {
                return 0.0;
            }

            return total_armor - _source.lethality;
        }

        armor + bonus_armor
    }

    fn calculate_magic_resist_reduction(&self, _source: &Champion) -> f32 {
        let mut mr = self.mr;

        if mr == 0.0 {
            return 0.0;
        }

        mr = mr - _source.flat_mr_reduction;

        if mr > 0.0 {
            mr = mr * (1.0 - _source.percent_mr_reduction);
            mr = mr * (1.0 - _source.percent_mr_pen);
            mr = mr - _source.mr_pen;
        }

        mr
    }

    fn calculate_physical_damage_taken_from_aa(&self, effective_armor: f32, _source: &Champion) -> Damage {
        let mut damage = Damage::new(_source.ad as f32, 0.0, 0.0);

        // Simplified crit damage calculation; we do not apply smoothing to compensate for "streaks"
        let crit_damage_multiplier = self.calculate_crit_damage_multiplier_from_target(_source);

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
        // Take away from the physical shield first
        if self.physical_shield_amount > 0.0 {
            let old_physical_shield_amount = self.physical_shield_amount;

            self.physical_shield_amount = self.physical_shield_amount - damage.physical_component;

            damage.reduce_physical_damage(old_physical_shield_amount);

            if self.physical_shield_amount < 0.0 {
                self.physical_shield_amount = 0.0;
            }
        }

        // Take away from the shield second
        if self.shield_amount > 0.0 && damage.physical_component > 0.0 {
            let old_shield_amount = self.shield_amount;

            self.shield_amount = self.shield_amount - damage.physical_component;

            damage.reduce_physical_damage(old_shield_amount);

            if self.shield_amount < 0.0 {
                self.shield_amount = 0.0;
            }
        }

        // Take away from the health last
        if damage.physical_component > 0.0 {
            self.health = (self.health - damage.physical_component).round();

            if self.health < 0.0 {
                self.health = 0.0;
            }

            damage.reduce_physical_damage(damage.physical_component);
        }
    }

    fn take_magical_damage(&mut self, damage: &mut Damage) {
        // Take away from the magic shield first
        if self.magic_shield_amount > 0.0 {
            let old_magic_shield_amount = self.magic_shield_amount;
            self.magic_shield_amount = self.magic_shield_amount - damage.magical_component;
            damage.reduce_magical_damage(old_magic_shield_amount);

            if self.magic_shield_amount < 0.0 {
                self.magic_shield_amount = 0.0;
            }
        }

        // Take away from the shield second
        if self.shield_amount > 0.0 && damage.magical_component > 0.0 {
            let old_shield_amount = self.shield_amount;
            self.shield_amount = self.shield_amount - damage.magical_component;
            damage.reduce_magical_damage(old_shield_amount);

            if self.shield_amount < 0.0 {
                self.shield_amount = 0.0;
            }
        }

        // Take away from the health last
        if damage.magical_component > 0.0 {
            self.health = (self.health - damage.magical_component).round();
            damage.reduce_magical_damage(damage.magical_component);

            if self.health < 0.0 {
                self.health = 0.0;
            }
        }
    }

    fn take_true_damage(&mut self, damage: &mut Damage) {
        // Take away from the shield first
        if self.shield_amount > 0.0 {
            let old_shield_amount = self.shield_amount;
            self.shield_amount = self.shield_amount - damage.true_component;
            damage.reduce_true_damage(old_shield_amount);

            if self.shield_amount < 0.0 {
                self.shield_amount = 0.0;
            }
        }

        // Take away from the health last
        if damage.true_component > 0.0 {
            self.health = (self.health - damage.true_component).round();
            damage.reduce_true_damage(damage.true_component);

            if self.health < 0.0 {
                self.health = 0.0;
            }
        }
    }

    fn calculate_crit_damage_multiplier_from_target(&self, _source: &Champion) -> f32 {
        1.0 + (_source.crit * (0.75 + _source.bonus_crit_percent))
    }
}

/// Calculate the attack speed of a champion
///
/// base_as: Base attack speed
///
/// as_ratio: Attack speed ratio
///
/// g: Percent bonus attack speed growth gained from leveling up
///
/// bonus_as: Sum of any percent bonus attack speed gained from any source _other than leveling up_
///
/// n: Level of the champion
fn calculate_attack_speed(base_as: f32, as_ratio: f32, growth: f32, bonus_as: f32, n: i32) -> f32 {
    base_as + ((bonus_as + growth * (n - 1) as f32 * (0.7025 + 0.0175 * (n - 1) as f32)) * as_ratio)
}

fn calculate_base_stat(base_stat: f32, bonus: f32, growth: f32, n: i32) -> f32 {
    base_stat + bonus + (growth * (n - 1) as f32) * (0.7025 + 0.0175 * (n - 1) as f32)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::effects::{DamageType, DoTEffect, LimitedUseOnHitEffect, StackingOnHitEffect, EffectTickRate};
    use crate::utils::create_champion_by_name;

    #[test]
    fn test_set_level() {
        let mut champion = create_champion_by_name("test-bruiser");

        assert_eq!(champion.health, 685.0);
        assert_eq!(champion.hp5, 3);
        assert_eq!(champion.resource, 0);
        assert_eq!(champion.rp5, 0);
        assert_eq!(champion.ad, 60);
        assert_eq!(champion.as_, 0.651);
        assert_eq!(champion.armor, 38.0);
        assert_eq!(champion.mr, 32.0);

        champion.set_level(2);

        assert_eq!(champion.health, 767.0);
        assert_eq!(champion.hp5, 4);
        assert_eq!(champion.resource, 0);
        assert_eq!(champion.rp5, 0);
        assert_eq!(champion.ad, 64);
        assert_eq!(champion.as_, 0.825468063);
        assert_eq!(champion.armor, 41.0);
        assert_eq!(champion.mr, 33.0);
    }

    #[test]
    fn test_calculate_crit_damage_multiplier_from_target() {
        let mut champion = create_champion_by_name("test-bruiser");
        let mut source = create_champion_by_name("test-bruiser");

        assert_eq!(champion.calculate_crit_damage_multiplier_from_target(&source), 1.0);

        source.crit = 1.0;

        assert_eq!(champion.calculate_crit_damage_multiplier_from_target(&source), 1.75);

        source.bonus_crit_percent = 0.5;

        assert_eq!(champion.calculate_crit_damage_multiplier_from_target(&source), 2.25);
    }

    #[test]
    fn test_take_true_damage() {
        let mut champion = create_champion_by_name("test-bruiser");

        champion.shield_amount = 100.0;
        champion.magic_shield_amount = 100.0;
        champion.physical_shield_amount = 100.0;

        let mut damage = crate::damage::Damage::new(0.0, 0.0, 100.0);

        champion.take_true_damage(&mut damage);

        assert_eq!(champion.shield_amount, 0.0);
        assert_eq!(champion.health, 685.0);
        assert_eq!(champion.magic_shield_amount, 100.0);
        assert_eq!(champion.physical_shield_amount, 100.0);

        damage.true_component = 100.0;

        champion.take_true_damage(&mut damage);

        assert_eq!(champion.shield_amount, 0.0);
        assert_eq!(champion.health, 585.0);

        champion.shield_amount = 99.0;
        damage.true_component = 100.0;

        champion.take_true_damage(&mut damage);

        assert_eq!(champion.shield_amount, 0.0);
        assert_eq!(champion.health, 584.0);

        damage.true_component = 600.0;

        champion.take_true_damage(&mut damage);

        assert_eq!(champion.shield_amount, 0.0);
        assert_eq!(champion.health, 0.0);
    }

    #[test]
    fn take_magic_damage() {
        let mut champion = create_champion_by_name("test-bruiser");

        champion.shield_amount = 100.0;
        champion.magic_shield_amount = 100.0;
        champion.physical_shield_amount = 100.0;

        let mut damage = crate::damage::Damage::new(0.0, 100.0, 0.0);

        champion.take_magical_damage(&mut damage);

        assert_eq!(champion.shield_amount, 100.0);
        assert_eq!(champion.health, 685.0);
        assert_eq!(champion.magic_shield_amount, 0.0);
        assert_eq!(champion.physical_shield_amount, 100.0);

        damage.magical_component = 100.0;

        champion.take_magical_damage(&mut damage);

        assert_eq!(champion.shield_amount, 0.0);
        assert_eq!(champion.health, 685.0);

        champion.shield_amount = 99.0;
        damage.magical_component = 100.0;

        champion.take_magical_damage(&mut damage);

        assert_eq!(champion.shield_amount, 0.0);
        assert_eq!(champion.health, 684.0);

        damage.magical_component = 700.0;

        champion.take_magical_damage(&mut damage);

        assert_eq!(champion.shield_amount, 0.0);
        assert_eq!(champion.health, 0.0);
    }

    #[test]
    fn take_physical_damage() {
        let mut champion = create_champion_by_name("test-bruiser");

        champion.shield_amount = 100.0;
        champion.magic_shield_amount = 100.0;
        champion.physical_shield_amount = 100.0;

        let mut damage = crate::damage::Damage::new(100.0, 0.0, 0.0);

        champion.take_physical_damage(&mut damage);

        assert_eq!(champion.shield_amount, 100.0);
        assert_eq!(champion.health, 685.0);
        assert_eq!(champion.magic_shield_amount, 100.0);
        assert_eq!(champion.physical_shield_amount, 0.0);

        damage.physical_component = 100.0;

        champion.take_physical_damage(&mut damage);

        assert_eq!(champion.shield_amount, 0.0);
        assert_eq!(champion.health, 685.0);

        champion.shield_amount = 99.0;
        damage.physical_component = 100.0;

        champion.take_physical_damage(&mut damage);

        assert_eq!(champion.shield_amount, 0.0);
        assert_eq!(champion.health, 684.0);

        damage.physical_component = 700.0;

        champion.take_physical_damage(&mut damage);

        assert_eq!(champion.shield_amount, 0.0);
        assert_eq!(champion.health, 0.0);
    }

    #[test]
    fn test_calculate_physical_damage_taken() {
        let mut champion = create_champion_by_name("test-bruiser");
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
        let mut champion = create_champion_by_name("test-bruiser");
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
    fn test_calculate_magic_resist_reduction() {
        let mut champion = create_champion_by_name("test-bruiser");
        champion.mr = 80.0;

        let mut source = create_champion_by_name("test-bruiser");

        source.mr_pen = 10.0;
        source.percent_mr_pen = 0.35;
        source.flat_mr_reduction = 20.0;
        source.percent_mr_reduction = 0.30;

        assert_eq!(champion.calculate_magic_resist_reduction(&source), 17.3);

        champion.mr = 18.0;

        assert_eq!(champion.calculate_magic_resist_reduction(&source), -2.0);
    }

    #[test]
    fn test_calculate_armor_reduction() {
        let mut champion = create_champion_by_name("test-bruiser");
        champion.armor = 100.0;
        champion.bonus_armor = 200.0;

        let mut source = create_champion_by_name("test-bruiser");

        source.lethality = 10.0;
        source.percent_bonus_armor_pen = 0.45;
        source.armor_reduction = 30.0;
        source.percent_armor_reduction = 0.30;

        assert_eq!(champion.calculate_armor_reduction(&source), 122.3);

        champion.armor = 18.0;
        champion.bonus_armor = 0.0;

        assert_eq!(champion.calculate_armor_reduction(&source), -12.0);
    }

    #[test]
    fn test_take_damage() {
        let mut champion = create_champion_by_name("test-bruiser");

        let damage1 = crate::damage::Damage::new(100.0, 100.0, 100.0);
        let damage2 = crate::damage::Damage::new(100.0, 100.0, 100.0);

        champion.take_damage(damage1);

        assert_eq!(champion.health, 385.0);
        assert_eq!(champion.shield_amount, 0.0);
        assert_eq!(champion.magic_shield_amount, 0.0);
        assert_eq!(champion.physical_shield_amount, 0.0);

        champion.shield_amount = 100.0;
        champion.magic_shield_amount = 100.0;
        champion.physical_shield_amount = 100.0;

        champion.take_damage(damage2);

        assert_eq!(champion.health, 385.0);
        assert_eq!(champion.shield_amount, 0.0);
        assert_eq!(champion.magic_shield_amount, 0.0);
        assert_eq!(champion.physical_shield_amount, 0.0);
    }

    #[test]
    fn test_add_limited_use_on_hit_effect() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = LimitedUseOnHitEffect::new("test", 100.0, DamageType::Physical, 1);

        champion.add_limited_use_on_hit_effect(effect);

        assert_eq!(champion.limited_use_on_hit_effects.len(), 1);
    }

    #[test]
    fn test_add_duplicate_limited_use_on_hit_effect() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = LimitedUseOnHitEffect::new("test", 100.0, DamageType::Physical, 1);
        let effect2 = LimitedUseOnHitEffect::new("test", 100.0, DamageType::Physical, 1);

        champion.add_limited_use_on_hit_effect(effect);
        champion.add_limited_use_on_hit_effect(effect2);

        assert_eq!(champion.limited_use_on_hit_effects.len(), 1);
    }

    #[test]
    fn test_add_duration_on_hit_effect() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = DoTEffect::new("test", 0.0, 100.0, DamageType::Physical, Duration::new(2, 0), EffectTickRate::PerSecond);

        champion.add_duration_on_hit_effect(effect);

        assert_eq!(champion.duration_on_hit_effects.len(), 1);
    }

    #[test]
    fn test_add_duplicate_duration_on_hit_effect() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = DoTEffect::new("test", 0.0, 100.0, DamageType::Physical, Duration::new(2, 0), EffectTickRate::PerSecond);
        let effect2 = DoTEffect::new("test", 0.0, 100.0, DamageType::Physical, Duration::new(2, 0), EffectTickRate::PerSecond);

        champion.add_duration_on_hit_effect(effect);
        champion.add_duration_on_hit_effect(effect2);

        assert_eq!(champion.duration_on_hit_effects.len(), 1);
    }

    #[test]
    fn test_add_stacking_on_hit_effect() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = StackingOnHitEffect::new("test", 10.0, 5.0, DamageType::Physical, 100, Duration::new(2, 0));

        champion.add_stacking_on_hit_effect(effect);

        assert_eq!(champion.stacking_on_hit_effects.len(), 1);
    }

    #[test]
    fn test_add_duplicate_stacking_on_hit_effect() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = StackingOnHitEffect::new("test", 10.0, 5.0, DamageType::Physical, 100, Duration::new(2, 0));
        let effect2 = StackingOnHitEffect::new("test", 10.0, 5.0, DamageType::Physical, 100, Duration::new(2, 0));

        champion.add_stacking_on_hit_effect(effect);
        champion.add_stacking_on_hit_effect(effect2);

        assert_eq!(champion.stacking_on_hit_effects.len(), 1);
    }

    #[test]
    fn test_remove_limited_use_on_hit_effects() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = LimitedUseOnHitEffect::new("test", 100.0, DamageType::Physical, 1);
        let effect2 = LimitedUseOnHitEffect::new("test2", 100.0, DamageType::Physical, 1);
        let effect3 = LimitedUseOnHitEffect::new("test3", 100.0, DamageType::Physical, 1);

        champion.add_limited_use_on_hit_effect(effect);
        champion.add_limited_use_on_hit_effect(effect2);
        champion.add_limited_use_on_hit_effect(effect3);

        champion.remove_on_hit_effect("test2");

        assert_eq!(champion.limited_use_on_hit_effects.len(), 2);
        assert_eq!(champion.limited_use_on_hit_effects[0].id, "test");
        assert_eq!(champion.limited_use_on_hit_effects[1].id, "test3");
    }

    #[test]
    fn test_remove_on_hit_effects_duration() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = DoTEffect::new("test", 0.0, 100.0, DamageType::Physical, Duration::new(2, 0), EffectTickRate::PerSecond);
        let effect2 = DoTEffect::new("test2", 0.0, 100.0, DamageType::Physical, Duration::new(2, 0), EffectTickRate::PerSecond);
        let effect3 = DoTEffect::new("test3", 0.0, 100.0, DamageType::Physical, Duration::new(2, 0), EffectTickRate::PerSecond);

        champion.add_duration_on_hit_effect(effect);
        champion.add_duration_on_hit_effect(effect2);
        champion.add_duration_on_hit_effect(effect3);

        champion.remove_on_hit_effect("test2");

        assert_eq!(champion.duration_on_hit_effects.len(), 2);
        assert_eq!(champion.duration_on_hit_effects[0].id, "test");
        assert_eq!(champion.duration_on_hit_effects[1].id, "test3");
    }

    #[test]
    fn test_remove_on_hit_effects_stacking() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = StackingOnHitEffect::new("test", 10.0, 5.0, DamageType::Physical, 100, Duration::new(2, 0));
        let effect2 = StackingOnHitEffect::new("test2", 10.0, 5.0, DamageType::Physical, 100, Duration::new(2, 0));
        let effect3 = StackingOnHitEffect::new("test3", 10.0, 5.0, DamageType::Physical, 100, Duration::new(2, 0));

        champion.add_stacking_on_hit_effect(effect);
        champion.add_stacking_on_hit_effect(effect2);
        champion.add_stacking_on_hit_effect(effect3);

        champion.remove_on_hit_effect("test2");

        assert_eq!(champion.stacking_on_hit_effects.len(), 2);
        assert_eq!(champion.stacking_on_hit_effects[0].id, "test");
        assert_eq!(champion.stacking_on_hit_effects[1].id, "test3");
    }

    #[test]
    fn test_decrement_limited_use_on_hit_effects() {
        let mut champion = create_champion_by_name("test-bruiser");

        let effect = LimitedUseOnHitEffect::new("test", 100.0, DamageType::Physical, 1);
        let effect2 = LimitedUseOnHitEffect::new("test2", 100.0, DamageType::Physical, 2);
        let effect3 = LimitedUseOnHitEffect::new("test3", 100.0, DamageType::Physical, 0);

        champion.add_limited_use_on_hit_effect(effect);
        champion.add_limited_use_on_hit_effect(effect2);
        champion.add_limited_use_on_hit_effect(effect3);

        champion.decrement_limited_use_on_hit_effects();

        assert_eq!(champion.limited_use_on_hit_effects[0].num_uses, 0);
        assert_eq!(champion.limited_use_on_hit_effects[1].num_uses, 1);
        assert_eq!(champion.limited_use_on_hit_effects[2].num_uses, 0);
    }

    #[test]
    fn test_calculate_on_hit_damage() {
        let mut champion = create_champion_by_name("test-bruiser");
        let mut source = create_champion_by_name("test-bruiser");

        let effect = LimitedUseOnHitEffect::new("test", 100.0, DamageType::Physical, 1);
        let effect2 = LimitedUseOnHitEffect::new("test2", 100.0, DamageType::Physical, 2);
        let effect3 = LimitedUseOnHitEffect::new("test3", 100.0, DamageType::Physical, 0);
        let effect4 = LimitedUseOnHitEffect::new("test4", 100.0, DamageType::Magical, 1);
        let effect5 = LimitedUseOnHitEffect::new("test5", 100.0, DamageType::True, 1);

        source.add_limited_use_on_hit_effect(effect);
        source.add_limited_use_on_hit_effect(effect2);
        source.add_limited_use_on_hit_effect(effect3);
        source.add_limited_use_on_hit_effect(effect4);
        source.add_limited_use_on_hit_effect(effect5);

        let damage = champion.calculate_on_hit_damage(&source);

        assert_eq!(damage.physical_component, 200.0);
        assert_eq!(damage.magical_component, 100.0);
        assert_eq!(damage.true_component, 100.0);
    }

    #[test]
    fn test_calculate_physical_damage_taken_from_aa_no_armor() {
        let mut champion = create_champion_by_name("test-bruiser");
        let source = create_champion_by_name("test-bruiser");

        let damage = champion.calculate_physical_damage_taken_from_aa(0.0, &source);

        assert_eq!(damage.physical_component, 60.0);
    }

    #[test]
    fn test_calculate_physical_damage_taken_from_aa_with_armor() {
        let mut champion = create_champion_by_name("test-bruiser");
        let source = create_champion_by_name("test-bruiser");

        let damage = champion.calculate_physical_damage_taken_from_aa(100.0, &source);

        assert_eq!(damage.physical_component, 30.0);
    }

    #[test]
    fn test_calculate_physical_damage_taken_from_aa_crit() {
        let mut champion = create_champion_by_name("test-bruiser");
        let mut source = create_champion_by_name("test-bruiser");

        source.crit = 1.0;

        let damage = champion.calculate_physical_damage_taken_from_aa(0.0, &source);

        assert_eq!(damage.physical_component, 105.0);
    }

    #[test]
    fn test_take_auto_attack_damage() {
        let mut champion = create_champion_by_name("test-bruiser");
        let mut source = create_champion_by_name("test-bruiser");

        source.ad = 100;
        source.crit = 1.0;

        champion.take_auto_attack_damage(&mut source);

        assert_eq!(champion.health, 558.0);
    }
}