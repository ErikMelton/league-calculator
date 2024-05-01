#[derive(Clone)]
pub struct ChampStats {
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
    pub(crate) as_: f32, // TODO: Consider the attack speed cap / exceeding the cap
    pub(crate) as_ratio: f32,
    pub(crate) base_armor: f32,
    pub(crate) base_armor_growth: f32,
    pub(crate) armor: f32,
    pub(crate) bonus_armor: f32,
    pub(crate) base_mr: f32,
    pub(crate) base_mr_growth: f32,
    pub(crate) mr: f32,
    pub(crate) bonus_mr: f32,
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
}

impl ChampStats {
    pub fn calculate_stats_from_level(&mut self, level: i32) {
        self.health = calculate_base_stat(self.base_health, 0.0, self.base_health_growth, level).round();
        self.hp5 = calculate_base_stat(self.base_hp5, 0.0, self.base_hp5_growth, level).round() as i32;
        self.resource = calculate_base_stat(self.base_resource, 0.0, self.base_resource_growth, level).round() as i32;
        self.rp5 = calculate_base_stat(self.base_rp5, 0.0, self.base_rp5_growth, level).round() as i32;
        self.ad = calculate_base_stat(self.base_ad, 0.0, self.base_ad_growth, level).round() as i32;
        self.as_ = calculate_attack_speed(self.base_as, self.as_ratio, self.base_as_growth_percent, 0.25, level);
        self.armor = calculate_base_stat(self.base_armor, self.bonus_armor, self.base_armor_growth, level).round();
        self.mr = calculate_base_stat(self.base_mr, self.bonus_mr, self.base_mr_growth, level).round();

        // TODO: Bonus values from items, runes, etc.
    }

    pub fn calculate_armor_reduction(&self, _source: &ChampStats) -> f32 {
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

    pub fn calculate_magic_resist_reduction(&self, _source: &ChampStats) -> f32 {
        let mut mr = self.mr + self.bonus_mr;

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
}

fn calculate_attack_speed(base_as: f32, as_ratio: f32, growth: f32, bonus_as: f32, n: i32) -> f32 {
    base_as + ((bonus_as + growth * (n - 1) as f32 * (0.7025 + 0.0175 * (n - 1) as f32)) * as_ratio)
}

fn calculate_base_stat(base_stat: f32, bonus: f32, growth: f32, n: i32) -> f32 {
    base_stat + bonus + (growth * (n - 1) as f32) * (0.7025 + 0.0175 * (n - 1) as f32)
}

pub fn calculate_crit_damage_multiplier_from_target(_source: &ChampStats) -> f32 {
    1.0 + (_source.crit * (0.75 + _source.bonus_crit_percent))
}

#[cfg(test)]
mod tests {
    use crate::champion::stats::{calculate_crit_damage_multiplier_from_target, ChampStats};
    use crate::utils::create_champion_by_name;

    #[test]
    fn test_clone_champ_stats() {
        let stats = ChampStats {
            base_health: 1000.0,
            base_health_growth: 100.0,
            health: 1000.0,
            base_hp5: 10.0,
            base_hp5_growth: 1.0,
            hp5: 10,
            base_resource: 1000.0,
            base_resource_growth: 100.0,
            resource: 1000,
            base_rp5: 10.0,
            base_rp5_growth: 1.0,
            rp5: 10,
            base_ad: 100.0,
            base_ad_growth: 10.0,
            ad: 100,
            base_as: 1.0,
            base_as_growth_percent: 0.1,
            attack_windup: 0.25,
            as_: 1.0,
            as_ratio: 0.1,
            base_armor: 50.0,
            base_armor_growth: 5.0,
            armor: 50.0,
            bonus_armor: 0.0,
            base_mr: 50.0,
            base_mr_growth: 5.0,
            mr: 50.0,
            bonus_mr: 0.0,
            base_range: 125,
            range: 125,
            base_ms: 325,
            ms: 325,
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
        };

        let stats_clone = stats.clone();

        assert_eq!(stats.base_health, stats_clone.base_health);
        assert_eq!(stats.base_health_growth, stats_clone.base_health_growth);
        assert_eq!(stats.health, stats_clone.health);
        assert_eq!(stats.base_hp5, stats_clone.base_hp5);
        assert_eq!(stats.base_hp5_growth, stats_clone.base_hp5_growth);
        assert_eq!(stats.hp5, stats_clone.hp5);
        assert_eq!(stats.base_resource, stats_clone.base_resource);
        assert_eq!(stats.base_resource_growth, stats_clone.base_resource_growth);
        assert_eq!(stats.resource, stats_clone.resource);
        assert_eq!(stats.base_rp5, stats_clone.base_rp5);
    }

    #[test]
    fn test_calculate_crit_damage_multiplier_from_target() {
        let champion = create_champion_by_name("test-bruiser");
        let mut source = create_champion_by_name("test-bruiser");

        assert_eq!(calculate_crit_damage_multiplier_from_target(&source.champ_stats), 1.0);

        source.champ_stats.crit = 1.0;

        assert_eq!(calculate_crit_damage_multiplier_from_target(&source.champ_stats), 1.75);

        source.champ_stats.bonus_crit_percent = 0.5;

        assert_eq!(calculate_crit_damage_multiplier_from_target(&source.champ_stats), 2.25);
    }

    #[test]
    fn test_calculate_armor_reduction() {
        let mut champion = create_champion_by_name("test-bruiser");

        champion.champ_stats.armor = 100.0;
        champion.champ_stats.bonus_armor = 200.0;

        let mut source = create_champion_by_name("test-bruiser");
        let mut source_stats = &mut source.champ_stats;

        source_stats.lethality = 10.0;
        source_stats.percent_bonus_armor_pen = 0.45;
        source_stats.armor_reduction = 30.0;
        source_stats.percent_armor_reduction = 0.30;

        assert_eq!(champion.champ_stats.calculate_armor_reduction(&source.champ_stats), 122.3);

        champion.champ_stats.armor = 18.0;
        champion.champ_stats.bonus_armor = 0.0;

        assert_eq!(champion.champ_stats.calculate_armor_reduction(&source.champ_stats), -12.0);
    }

    #[test]
    fn test_calculate_magic_resist_reduction() {
        let mut champion = create_champion_by_name("test-bruiser");
        let mut source = create_champion_by_name("test-bruiser");

        champion.champ_stats.mr = 80.0;

        source.champ_stats.mr_pen = 10.0;
        source.champ_stats.percent_mr_pen = 0.35;
        source.champ_stats.flat_mr_reduction = 20.0;
        source.champ_stats.percent_mr_reduction = 0.30;

        assert_eq!(champion.champ_stats.calculate_magic_resist_reduction(&source.champ_stats), 17.3);

        champion.champ_stats.mr = 18.0;

        assert_eq!(champion.champ_stats.calculate_magic_resist_reduction(&source.champ_stats), -2.0);
    }
}
