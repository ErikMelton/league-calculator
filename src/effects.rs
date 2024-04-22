use std::ops::Rem;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum DamageType {
    Physical,
    Magical,
    True,
}

#[derive(Debug, Clone)]
pub enum EffectTickRate {
    PerSecond = 30,
    PerHalfSecond = 15,
    PerQuarterSecond = 8,
    PerHalfQuarterSecond = 4,
}

#[derive(Debug, Clone)]
pub struct DoTEffect {
    pub(crate) id: String,
    pub(crate) damage_over_time: f32,
    pub(crate) damage_type: DamageType,
    pub(crate) damage_time_left: Duration,
    pub(crate) tick_rate: EffectTickRate,
    pub(crate) effect_time_left: Duration,
    pub(crate) finite_time_left: bool,
}

#[derive(Debug, Clone)]
pub struct LimitedUseOnHitEffect {
    pub(crate) id: String,
    pub(crate) damage: f32,
    pub(crate) damage_type: DamageType,
    pub(crate) num_uses: i32,
    pub(crate) effect_time_left: Duration,
    pub(crate) finite_time_left: bool,
}

#[derive(Debug, Clone)]
pub struct StackingOnHitEffect {
    pub(crate) id: String,
    pub(crate) damage_over_time: f32,
    pub(crate) damage_type: DamageType,
    pub(crate) damage_time_left: Duration,
    pub(crate) max_stacks: i32,
    pub(crate) current_stacks: i32,
    pub(crate) effect_time_left: Duration,
    pub(crate) finite_time_left: bool
}

impl StackingOnHitEffect {
    pub(crate) fn new(id: &str, damage_over_time: f32, damage_type: DamageType, max_stacks: i32, damage_time_left: Duration, effect_time_left: Duration, finite_time_left: bool) -> Self {
        StackingOnHitEffect {
            id: id.to_string(),
            damage_over_time,
            damage_type,
            max_stacks,
            current_stacks: 0,
            effect_time_left,
            damage_time_left,
            finite_time_left,
        }
    }

    pub fn set_current_stacks(&mut self, current_stacks: i32) {
        self.current_stacks = current_stacks;
    }

    pub fn set_time_left(&mut self, time_left: Duration) {
        self.damage_time_left = time_left;
    }

    pub fn reduce_damage_time_left(&mut self, time: Duration) {
        self.damage_time_left -= time;
    }

    pub fn set_effect_time_left(&mut self, time_left: Duration) {
        self.effect_time_left = time_left;
    }

    pub fn reduce_effect_time_left(&mut self, time: Duration) {
        self.effect_time_left -= time;
    }

    pub fn reduce_current_stacks(&mut self) {
        self.current_stacks -= 1;
    }
}

impl DoTEffect {
    pub(crate) fn new(id: &str, damage_over_time: f32, damage_type: DamageType, time_left: Duration,
                      tick_rate: EffectTickRate, effect_time_left: Duration, finite_time_left: bool) -> Self {
        DoTEffect {
            id: id.to_string(),
            damage_over_time,
            damage_type,
            damage_time_left: time_left,
            tick_rate,
            effect_time_left,
            finite_time_left
        }
    }

    pub fn set_time_left(&mut self, time_left: Duration) {
        self.damage_time_left = time_left;
    }

    pub fn reduce_time_left(&mut self, time: Duration) {
        self.damage_time_left -= time;
    }
}

impl LimitedUseOnHitEffect {
    pub(crate) fn new(id: &str, damage: f32, damage_type: DamageType, num_uses: i32, effect_time_left: Duration, finite_time_left: bool) -> Self {
        LimitedUseOnHitEffect {
            id: id.to_string(),
            damage,
            damage_type,
            num_uses,
            effect_time_left,
            finite_time_left,
        }
    }

    pub fn reduce_num_uses(&mut self) {
        self.num_uses -= 1;
    }

    pub fn set_effect_time_left(&mut self, time_left: Duration) {
        self.effect_time_left = time_left;
    }

    pub fn reduce_effect_time_left(&mut self, time: Duration) {
        self.effect_time_left -= time;
    }
}

impl PartialEq for &DoTEffect {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq for &LimitedUseOnHitEffect {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq for &StackingOnHitEffect {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq for DamageType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DamageType::Physical, DamageType::Physical) => true,
            (DamageType::Magical, DamageType::Magical) => true,
            (DamageType::True, DamageType::True) => true,
            _ => false,
        }
    }
}

impl EffectTickRate {
    pub fn value(&self) -> i32 {
        match self {
            EffectTickRate::PerSecond => 30,
            EffectTickRate::PerHalfSecond => 15,
            EffectTickRate::PerQuarterSecond => 8,
            EffectTickRate::PerHalfQuarterSecond => 4,
        }
    }
}

impl PartialEq for EffectTickRate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EffectTickRate::PerSecond, EffectTickRate::PerSecond) => true,
            (EffectTickRate::PerHalfSecond, EffectTickRate::PerHalfSecond) => true,
            (EffectTickRate::PerQuarterSecond, EffectTickRate::PerQuarterSecond) => true,
            (EffectTickRate::PerHalfQuarterSecond, EffectTickRate::PerHalfQuarterSecond) => true,
            _ => false,
        }
    }
}

impl Rem<&EffectTickRate> for i32 {
    type Output = i32;

    fn rem(self, rhs: &EffectTickRate) -> Self::Output {
        self % rhs.value()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::effects::{DamageType, DoTEffect, EffectTickRate, LimitedUseOnHitEffect, StackingOnHitEffect};

    #[test]
    fn test_dot_effect_new() {
        let dot_effect = DoTEffect::new("burn", 5.0, DamageType::Magical, Duration::from_secs(5), EffectTickRate::PerSecond, Duration::from_secs(10), true);

        assert_eq!(dot_effect.id, "burn");
        assert_eq!(dot_effect.damage_over_time, 5.0);
        assert_eq!(dot_effect.damage_type, DamageType::Magical);
        assert_eq!(dot_effect.damage_time_left, Duration::from_secs(5));
        assert_eq!(dot_effect.tick_rate, EffectTickRate::PerSecond);
    }

    #[test]
    fn test_dot_effect_set_time_left() {
        let mut dot_effect = DoTEffect::new("burn", 5.0, DamageType::Magical, Duration::from_secs(5), EffectTickRate::PerSecond, Duration::from_secs(10), true);
        dot_effect.set_time_left(Duration::from_secs(10));

        assert_eq!(dot_effect.damage_time_left, Duration::from_secs(10));
    }

    #[test]
    fn test_dot_effect_reduce_time_left() {
        let mut dot_effect = DoTEffect::new("burn",  5.0, DamageType::Magical, Duration::from_secs(5), EffectTickRate::PerSecond, Duration::from_secs(10), true);
        dot_effect.reduce_time_left(Duration::from_secs(2));

        assert_eq!(dot_effect.damage_time_left, Duration::from_secs(3));
    }

    #[test]
    fn test_limited_use_on_hit_effect_new() {
        let limited_use_on_hit_effect = LimitedUseOnHitEffect::new("test", 50.0, DamageType::True, 5, Duration::from_secs(10), true);

        assert_eq!(limited_use_on_hit_effect.id, "test");
        assert_eq!(limited_use_on_hit_effect.damage, 50.0);
        assert_eq!(limited_use_on_hit_effect.damage_type, DamageType::True);
        assert_eq!(limited_use_on_hit_effect.num_uses, 5);
    }

    #[test]
    fn test_limited_use_on_hit_effect_reduce_num_uses() {
        let mut limited_use_on_hit_effect = LimitedUseOnHitEffect::new("test", 50.0, DamageType::True, 5, Duration::from_secs(10), true);
        limited_use_on_hit_effect.reduce_num_uses();

        assert_eq!(limited_use_on_hit_effect.num_uses, 4);
    }

    #[test]
    fn test_stacking_on_hit_effect_new() {
        let stacking_on_hit_effect = StackingOnHitEffect::new("test", 5.0, DamageType::True, 5, Duration::from_secs(5), Duration::from_secs(10), true);

        assert_eq!(stacking_on_hit_effect.id, "test");
        assert_eq!(stacking_on_hit_effect.damage_over_time, 5.0);
        assert_eq!(stacking_on_hit_effect.damage_type, DamageType::True);
        assert_eq!(stacking_on_hit_effect.max_stacks, 5);
        assert_eq!(stacking_on_hit_effect.current_stacks, 0);
        assert_eq!(stacking_on_hit_effect.damage_time_left, Duration::from_secs(5));
    }

    #[test]
    fn test_stacking_on_hit_effect_set_current_stacks() {
        let mut stacking_on_hit_effect = StackingOnHitEffect::new("test", 5.0, DamageType::True, 5, Duration::from_secs(5), Duration::from_secs(10), true);
        stacking_on_hit_effect.set_current_stacks(3);

        assert_eq!(stacking_on_hit_effect.current_stacks, 3);
    }

    #[test]
    fn test_stacking_on_hit_effect_set_time_left() {
        let mut stacking_on_hit_effect = StackingOnHitEffect::new("test", 5.0, DamageType::True, 5, Duration::from_secs(5), Duration::from_secs(10), true);
        stacking_on_hit_effect.set_time_left(Duration::from_secs(10));

        assert_eq!(stacking_on_hit_effect.damage_time_left, Duration::from_secs(10));
    }

    #[test]
    fn test_stacking_on_hit_effect_reduce_time_left() {
        let mut stacking_on_hit_effect = StackingOnHitEffect::new("test", 5.0, DamageType::True, 5, Duration::from_secs(5), Duration::from_secs(10), true);
        stacking_on_hit_effect.reduce_damage_time_left(Duration::from_secs(2));

        assert_eq!(stacking_on_hit_effect.damage_time_left, Duration::from_secs(3));
    }

    #[test]
    fn test_stacking_on_hit_effect_reduce_current_stacks() {
        let mut stacking_on_hit_effect = StackingOnHitEffect::new("test", 5.0, DamageType::True, 5, Duration::from_secs(5), Duration::from_secs(10), true);
        stacking_on_hit_effect.set_current_stacks(3);
        stacking_on_hit_effect.reduce_current_stacks();

        assert_eq!(stacking_on_hit_effect.current_stacks, 2);
    }

    #[test]
    fn test_partial_eq_for_dot_effect() {
        let dot_effect1 = DoTEffect::new("burn", 5.0, DamageType::Magical, Duration::from_secs(5), EffectTickRate::PerSecond, Duration::from_secs(10), true);
        let dot_effect2 = DoTEffect::new("burn", 5.0, DamageType::Magical, Duration::from_secs(5), EffectTickRate::PerSecond, Duration::from_secs(10), true);

        assert_eq!(&dot_effect1, &dot_effect2);
    }

    #[test]
    fn test_partial_eq_for_limited_use_on_hit_effect() {
        let limited_use_on_hit_effect1 = LimitedUseOnHitEffect::new("test", 50.0, DamageType::True, 5, Duration::from_secs(10), true);
        let limited_use_on_hit_effect2 = LimitedUseOnHitEffect::new("test", 50.0, DamageType::True, 5, Duration::from_secs(10), true);

        assert_eq!(&limited_use_on_hit_effect1, &limited_use_on_hit_effect2);
    }

    #[test]
    fn test_partial_eq_for_stacking_on_hit_effect() {
        let stacking_on_hit_effect1 = StackingOnHitEffect::new("test", 5.0, DamageType::True, 5, Duration::from_secs(5), Duration::from_secs(10), true);
        let stacking_on_hit_effect2 = StackingOnHitEffect::new("test", 5.0, DamageType::True, 5, Duration::from_secs(5), Duration::from_secs(10), true);

        assert_eq!(&stacking_on_hit_effect1, &stacking_on_hit_effect2);
    }

    #[test]
    fn test_partial_eq_for_damage_type() {
        let damage_type1 = DamageType::Physical;
        let damage_type2 = DamageType::Physical;

        let damage_type3 = DamageType::Magical;
        let damage_type4 = DamageType::Magical;

        let damage_type5 = DamageType::True;
        let damage_type6 = DamageType::True;

        assert_eq!(&damage_type1, &damage_type2);
        assert_eq!(&damage_type3, &damage_type4);
        assert_eq!(&damage_type5, &damage_type6);

        assert_ne!(&damage_type1, &damage_type3);
        assert_ne!(&damage_type1, &damage_type5);
        assert_ne!(&damage_type3, &damage_type5);
    }

    #[test]
    fn test_partial_eq_for_effect_tick_rate() {
        let effect_tick_rate1 = EffectTickRate::PerSecond;
        let effect_tick_rate2 = EffectTickRate::PerSecond;

        let effect_tick_rate3 = EffectTickRate::PerHalfSecond;
        let effect_tick_rate4 = EffectTickRate::PerHalfSecond;

        let effect_tick_rate5 = EffectTickRate::PerQuarterSecond;
        let effect_tick_rate6 = EffectTickRate::PerQuarterSecond;

        let effect_tick_rate7 = EffectTickRate::PerHalfQuarterSecond;
        let effect_tick_rate8 = EffectTickRate::PerHalfQuarterSecond;

        assert_eq!(&effect_tick_rate1, &effect_tick_rate2);
        assert_eq!(&effect_tick_rate3, &effect_tick_rate4);
        assert_eq!(&effect_tick_rate5, &effect_tick_rate6);
        assert_eq!(&effect_tick_rate7, &effect_tick_rate8);

        assert_ne!(&effect_tick_rate1, &effect_tick_rate3);
        assert_ne!(&effect_tick_rate1, &effect_tick_rate5);
        assert_ne!(&effect_tick_rate1, &effect_tick_rate7);
        assert_ne!(&effect_tick_rate3, &effect_tick_rate5);
        assert_ne!(&effect_tick_rate3, &effect_tick_rate7);
        assert_ne!(&effect_tick_rate5, &effect_tick_rate7);
    }
}
