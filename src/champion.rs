pub struct Champion {
    pub(crate) name: String,
    pub(crate) level: i32,
    base_health: f32,
    base_health_growth: f32,
    pub(crate) health: f32,
    base_hp5: f32,
    base_hp5_growth: f32,
    pub(crate) hp5: i32,
    base_resource: f32,
    base_resource_growth: f32,
    pub(crate) resource: i32,
    base_rp5: f32,
    base_rp5_growth: f32,
    pub(crate) rp5: i32,
    base_ad: f32,
    base_ad_growth: f32,
    pub(crate) ad: i32,
    base_as: f32,
    base_as_growth_percent: f32,
    attack_windup: f32,
    pub(crate) as_: f32,
    as_ratio: f32,
    base_armor: f32,
    base_armor_growth: f32,
    pub(crate) armor: f32,
    pub(crate) bonus_armor: f32,
    base_mr: f32,
    base_mr_growth: f32,
    pub(crate) mr: i32,
    base_range: i32,
    pub(crate) range: i32,
    base_ms: i32,
    pub(crate) ms: i32,
    base_crit: f32,
    pub(crate) crit: f32,
    base_crit_percent_damage: f32,
    // TODO: Shields only last for a certain amount of time, and decay! Use a struct
    pub(crate) shield_amount: f32,
    pub(crate) magic_shield_amount: f32,
    pub(crate) physical_shield_amount: f32,
    pub(crate) mr_pen: i32,
    pub(crate) lethality: f32, // https://leagueoflegends.fandom.com/wiki/Armor_penetration
    pub(crate) percent_bonus_armor_pen: f32,
    pub(crate) armor_reduction: f32,
    pub(crate) percent_armor_reduction: f32,
    pub(crate) life_steal: i32,
    pub(crate) spell_vamp: i32,
    pub(crate) tenacity: i32,
}

pub enum DamageType {
    Physical,
    Magical,
    True,
}

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
        self.mr = calculate_base_stat(self.base_mr, 0.0, self.base_mr_growth, level).round() as i32;
    }

    pub fn take_auto_attack_damage(&mut self, _source: &Champion) {
        let effective_armor = self.calculate_armor_reduction(_source);
        let effective_health = self.health + self.shield_amount + self.physical_shield_amount;
        let mut damage = self.calculate_physical_damage_taken(effective_armor, _source);

        if damage >= effective_health {
            println!("Champion died!");
            return;
        }

        // Take away from the physical shield first
        if self.physical_shield_amount > 0.0 {
            let old_physical_shield_amount = self.physical_shield_amount;

            self.physical_shield_amount = self.physical_shield_amount - damage;

            damage = damage - old_physical_shield_amount;

            if self.physical_shield_amount < 0.0 {
                self.physical_shield_amount = 0.0;
            }
        }

        // Take away from the shield second
        if self.shield_amount > 0.0 && damage > 0.0{
            let old_shield_amount = self.shield_amount;

            self.shield_amount = self.shield_amount - damage;

            damage = damage - old_shield_amount;

            if self.shield_amount < 0.0 {
                self.shield_amount = 0.0;
            }
        }

        // Take away from the health last
        if damage > 0.0 {
            self.health = self.health - damage;
        }


        println!("Remaining health: {}", self.health);

        // TODO: Calculate crit damage
        // TODO: Consider on-hit effects
        // TODO: Consider auto cancels
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

    fn calculate_physical_damage_taken(&self, effective_armor: f32,  _source: &Champion) -> f32 {
        return if effective_armor >= 0.0 {
            (100.0 / (100.0 + effective_armor)) * _source.ad as f32
        } else {
            (2.0 - 100.0 / (100.0 - effective_armor)) * _source.ad as f32
        }
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

pub fn create_champion_by_name(name: &str) -> Champion {
    let lower_name = name.to_lowercase();

    // TODO: Load these base values from a file or API
    match lower_name.as_str() {
        "aatrox" => Champion {
            name: String::from("Aatrox"),
            level: 1,
            base_health: 685.0,
            base_health_growth: 114.0,
            health: 650.0,
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
            mr: 32,
            base_range: 175,
            range: 175,
            base_ms: 345,
            ms: 345,
            base_crit: 0.0,
            crit: 0.0,
            base_crit_percent_damage: 1.75,
            shield_amount: 0.0,
            magic_shield_amount: 0.0,
            physical_shield_amount: 0.0,
            mr_pen: 0,
            lethality: 0.0,
            percent_bonus_armor_pen: 0.0,
            armor_reduction: 0.0,
            percent_armor_reduction: 0.0,
            life_steal: 0,
            spell_vamp: 0,
            tenacity: 0,
        },
        _ => {
            panic!("Champion not found: {}", name);
        }
    }
}

pub fn create_dummy() -> Champion {
    Champion {
        name: String::from("Dummy"),
        level: 1,
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
        mr: 0,
        base_range: 0,
        range: 0,
        base_ms: 0,
        ms: 0,
        base_crit: 0.0,
        crit: 0.0,
        base_crit_percent_damage: 0.0,
        shield_amount: 0.0,
        magic_shield_amount: 0.0,
        physical_shield_amount: 0.0,
        mr_pen: 0,
        lethality: 0.0,
        percent_bonus_armor_pen: 0.0,
        armor_reduction: 0.0,
        percent_armor_reduction: 0.0,
        life_steal: 0,
        spell_vamp: 0,
        tenacity: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_level() {
        let mut champion = create_champion_by_name("aatrox");
        champion.set_level(2);

        assert_eq!(champion.health, 764.0);
        assert_eq!(champion.hp5, 4);
        assert_eq!(champion.resource, 0);
        assert_eq!(champion.rp5, 0);
        assert_eq!(champion.ad, 65);
        assert_eq!(champion.as_, 0.676);
        assert_eq!(champion.armor, 42.45);
        assert_eq!(champion.mr, 34);
    }

    #[test]
    fn test_take_auto_attack_damage() {
        let mut champion = create_champion_by_name("aatrox");
        let mut dummy = create_dummy();

        champion.take_auto_attack_damage(&dummy);

        assert_eq!(champion.health, 9940.0);
    }

    #[test]
    fn test_take_auto_attack_damage_with_shield() {
        let mut champion = create_champion_by_name("aatrox");
        let mut dummy = create_dummy();

        champion.physical_shield_amount = 100.0;

        champion.take_auto_attack_damage(&dummy);

        assert_eq!(champion.physical_shield_amount, 40.0);
        assert_eq!(champion.health, 10000.0);
    }

    #[test]
    fn test_take_auto_attack_damage_with_shield_and_shield() {
        let mut champion = create_champion_by_name("aatrox");
        let mut dummy = create_dummy();

        champion.physical_shield_amount = 100.0;
        champion.shield_amount = 100.0;

        champion.take_auto_attack_damage(&dummy);

        assert_eq!(champion.physical_shield_amount, 40.0);
        assert_eq!(champion.shield_amount, 0.0);
        assert_eq!(champion.health, 10000.0);
    }

    #[test]
    fn test_take_auto_attack_damage_with_shield_and_shield_and_health() {
        let mut champion = create_champion_by_name("aatrox");
        let mut dummy = create_dummy();

        champion.physical_shield_amount = 100.0;
        champion.shield_amount = 100.0;
        champion.health = 100.0;

        champion.take_auto_attack_damage(&dummy);

        assert_eq!(champion.physical_shield_amount, 40.0);
        assert_eq!(champion.shield_amount, 0.0);
        assert_eq!(champion.health, 40.0);
    }
}