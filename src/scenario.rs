use std::ops::Rem;
use std::time::Duration;
use crate::build::Build;
use crate::champion::Champion;
use crate::damage::Damage;
use crate::effects::DamageType;

pub struct Scenario {
    pub first_actor: u8, // 0 = you, 1 = enemy; maybe should be an enum
    pub first_hit_reaction_delay: Duration, // In seconds
    pub champ1_build: Build,
    // TODO: Champ1 ability rotation, hit chance
    pub champ2_build: Build,
    // TODO: Champ2 ability rotation, hit chance
}

const TICKS_PER_SECOND: f32 = 30.0;
const TICK_SECOND: f32 = 1.0 / TICKS_PER_SECOND;

impl Scenario {

    pub fn new(first_actor: u8, first_hit_reaction_delay: Duration, champ1_build: Build, champ2_build: Build) -> Scenario {
        Scenario {
            first_actor,
            first_hit_reaction_delay,
            champ1_build,
            champ2_build,
        }
    }

    pub fn calculate_scenario(&mut self) {
        let mut tick = 0;
        let first_hit_reaction_delay_in_ticks = (TICKS_PER_SECOND * self.first_hit_reaction_delay.as_secs_f32()).round() as i32;

        let mut champ1 = self.champ1_build.champion.clone();
        let champ1_as_in_ticks = (TICKS_PER_SECOND / champ1.as_).round() as i32;

        let mut champ2 = self.champ2_build.champion.clone();
        let champ2_as_in_ticks = (TICKS_PER_SECOND / champ2.as_).round() as i32;

        println!("Calculating scenario between:");
        println!("{} at level {}", champ1.name, champ1.level);
        println!("{} at level {}", champ2.name, champ2.level);
        println!("{} ({}) will attack first", if self.first_actor == 0 { champ1.name.as_str() } else { champ2.name.as_str() }, if self.first_actor == 0 { champ1.level } else { champ2.level });
        println!("First hit reaction delay: {} seconds", self.first_hit_reaction_delay.as_secs_f32());
        println!();


        while champ1.health > 0.0 && champ2.health > 0.0 {
            // Decrement time left on self-applied DoT effects
            self.decrement_own_effect_time_left(&mut champ1);
            self.decrement_own_effect_time_left(&mut champ2);

            let mut total_damage = Damage::new(0.0, 0.0, 0.0);

            total_damage += self.calculate_aa_damage_and_side_effects(
                tick,
                first_hit_reaction_delay_in_ticks,
                &mut champ1, champ1_as_in_ticks,
                &mut champ2, champ2_as_in_ticks
            );
            // let dot_damage = self.calculate_dot_damage

            if total_damage.total() > 0.0 {
                println!("A total of {} damage was dealt this tick. \n", total_damage.total());
            }

            tick += 1;
        }

        if champ1.health <= 0.0 {
            println!("{} ({}) wins!", champ2.name, champ2.level);
        } else {
            println!("{} ({}) wins!", champ1.name, champ2.level);
        }

        println!("{} ({}): {}", champ1.name, champ1.level, champ1.health);
        println!("{} ({}): {}", champ2.name, champ1.level, champ2.health);
        println!("The fight would have lasted {} seconds.", tick as f32 / TICKS_PER_SECOND);
    }

    fn apply_duration_on_hit_effects(&mut self, receiver: &mut Champion, giver: &mut Champion) {
        for (_, effect) in giver.friendly_duration_on_hit_effects.iter() {
            receiver.apply_enemy_duration_on_hit_effect(effect.clone());
        }
    }

    fn apply_stacking_on_hit_effects(&mut self, receiver: &mut Champion, giver: &mut Champion) {
        for (_, effect) in giver.friendly_stacking_on_hit_effects.iter() {
            receiver.apply_enemy_stacking_on_hit_effect(effect.clone());
        }
    }

    fn decrement_own_effect_time_left(&mut self, champ: &mut Champion) {
        for (id, effect) in champ.friendly_duration_on_hit_effects.iter_mut() {
            if effect.effect_time_left <= Duration::from_secs(0) {
                champ.friendly_duration_on_hit_effects.remove(id);

                continue;
            }

            if effect.finite_time_left {
                effect.reduce_time_left(Duration::from_secs_f32(TICK_SECOND));
            }
        }

        for (id, effect) in champ.friendly_stacking_on_hit_effects.iter_mut() {
            if effect.effect_time_left <= Duration::from_secs(0) {
                champ.friendly_stacking_on_hit_effects.remove(id);

                continue;
            }

            if effect.finite_time_left {
                effect.reduce_time_left(Duration::from_secs_f32(TICK_SECOND));
            }
        }
    }

    fn check_and_calculate_dot_effects(&mut self, tick: i32, champ1: &mut Champion, champ2: &mut Champion) {
        for (id, effect) in champ1.enemy_duration_on_hit_effects.iter_mut() {
            if effect.damage_time_left <= Duration::from_secs(0) {
                champ1.enemy_duration_on_hit_effects.remove(id);

                continue;
            }

            // TODO: Check if the damage time left is a multiple of the tick time and effect tick rate
            if tick % &effect.tick_rate == 0 {
                let mut damage = Damage::new(0.0, 0.0, 0.0);

                match effect.damage_type {
                    DamageType::Physical => {
                        damage.physical_component = effect.damage_over_time;
                    }
                    DamageType::Magical => {
                        damage.magical_component = effect.damage_over_time;
                    }
                    DamageType::True => {
                        damage.true_component = effect.damage_over_time;
                    }
                }

                champ1.take_dot_damage()

            }


            effect.effect_time_left -= Duration::from_secs_f32(TICK_SECOND);
        }
    }

    fn calculate_aa_damage_and_side_effects(&mut self, tick: i32,
                                            first_hit_reaction_delay_in_ticks: i32, mut champ1: &mut Champion,
                                            champ1_as_in_ticks: i32, mut champ2: &mut Champion,
                                            champ2_as_in_ticks: i32) -> Damage {
        if tick == 0 {
            return if self.champ1_acts_first() {
                let damage = champ2.take_auto_attack_damage(&mut champ1);
                self.apply_stacking_on_hit_effects(champ2, champ1);
                self.apply_duration_on_hit_effects(champ2, champ1);

                println!("{tick} | {} ({}) attacks {} ({}) for {} damage!", champ1.name, champ1.level, champ2.name, champ2.level, damage.total());

                damage
            } else {
                let damage = champ1.take_auto_attack_damage(&mut champ2);
                self.apply_stacking_on_hit_effects(champ1, champ2);
                self.apply_duration_on_hit_effects(champ1, champ2);

                println!("{tick} | {} ({}) attacks {} ({}) for {} damage!", champ2.name, champ2.level, champ1.name, champ2.level, damage.total());

                damage
            }
        } else if tick == first_hit_reaction_delay_in_ticks {
            return if self.champ1_acts_first() {
                let damage = champ2.take_auto_attack_damage(&mut champ1);
                self.apply_stacking_on_hit_effects(champ2, champ1);
                self.apply_duration_on_hit_effects(champ2, champ1);

                println!("{tick} | {} ({}) attacks {} ({}) for {} damage!", champ1.name, champ1.level, champ2.name, champ2.level, damage.total());

                damage
            } else {
                let damage = champ1.take_auto_attack_damage(&mut champ2);
                self.apply_stacking_on_hit_effects(champ1, champ2);
                self.apply_duration_on_hit_effects(champ1, champ2);

                println!("{tick} | {} ({}) attacks {} ({}) for {} damage!", champ2.name, champ2.level, champ1.name, champ1.level, damage.total());

                damage
            }
        } else if tick > first_hit_reaction_delay_in_ticks {
            let mut damage = Damage::new(0.0, 0.0, 0.0);

            if tick % champ1_as_in_ticks == 0 {
                damage += champ2.take_auto_attack_damage(&mut champ1);
                self.apply_stacking_on_hit_effects(champ2, champ1);
                self.apply_duration_on_hit_effects(champ2, champ1);

                println!("{tick} | {} ({}) attacks {} ({}) for {} damage!", champ1.name, champ1.level, champ2.name, champ2.level, damage.total());
            }

            if tick % champ2_as_in_ticks == 0 {
                damage += champ1.take_auto_attack_damage(&mut champ2);
                self.apply_stacking_on_hit_effects(champ1, champ2);
                self.apply_duration_on_hit_effects(champ1, champ2);

                println!("{tick} | {} ({}) attacks {} ({}) for {} damage!", champ2.name, champ2.level, champ1.name, champ1.level, damage.total());
            }

            return damage;
        }


        // TODO: Check DoT damage / stacking effect damage

        // TODO: Consider auto cancels

        Damage::new(0.0, 0.0, 0.0)
    }

    fn champ1_acts_first(&self) -> bool {
        self.first_actor == 0
    }
}
