use std::time::Duration;

pub enum DamageType {
    Physical,
    Magical,
    True,
}

pub enum TickRate {
    PerSecond,
    PerHalfSecond,
    PerQuarterSecond,
    PerHalfQuarterSecond,
}

pub struct DoTEffect {
    pub(crate) id: String,
    pub(crate) damage: f32,
    pub(crate) damage_over_time: f32,
    pub(crate) damage_type: DamageType,
    pub(crate) time_left: Duration,
    pub(crate) tick_rate: TickRate
}

pub struct LimitedUseOnHitEffect {
    pub(crate) id: String,
    pub(crate) damage: f32,
    pub(crate) damage_type: DamageType,
    pub(crate) num_uses: i32,
}

pub struct StackingOnHitEffect {
    pub(crate) id: String,
    pub(crate) damage: f32,
    pub(crate) damage_over_time: f32,
    pub(crate) damage_type: DamageType,
    pub(crate) max_stacks: i32,
    pub(crate) current_stacks: i32,
    pub(crate) time_left: Duration,
}

impl StackingOnHitEffect {
    pub(crate) fn new(id: &str, damage: f32, damage_over_time: f32, damage_type: DamageType, max_stacks: i32, time_left: Duration) -> Self {
        StackingOnHitEffect {
            id: id.to_string(),
            damage,
            damage_over_time,
            damage_type,
            max_stacks,
            current_stacks: 0,
            time_left,
        }
    }

    pub fn set_current_stacks(&mut self, current_stacks: i32) {
        self.current_stacks = current_stacks;
    }

    pub fn set_time_left(&mut self, time_left: Duration) {
        self.time_left = time_left;
    }

    pub fn reduce_time_left(&mut self, time: Duration) {
        self.time_left -= time;
    }

    pub fn reduce_current_stacks(&mut self) {
        self.current_stacks -= 1;
    }
}

impl DoTEffect {
    pub(crate) fn new(id: &str, damage: f32, damage_over_time: f32, damage_type: DamageType, time_left: Duration, tick_rate: TickRate) -> Self {
        DoTEffect {
            id: id.to_string(),
            damage,
            damage_over_time,
            damage_type,
            time_left,
            tick_rate,
        }
    }

    pub fn set_time_left(&mut self, time_left: Duration) {
        self.time_left = time_left;
    }

    pub fn reduce_time_left(&mut self, time: Duration) {
        self.time_left -= time;
    }
}

impl LimitedUseOnHitEffect {
    pub(crate) fn new(id: &str, damage: f32, damage_type: DamageType, num_uses: i32) -> Self {
        LimitedUseOnHitEffect {
            id: id.to_string(),
            damage,
            damage_type,
            num_uses,
        }
    }

    pub fn reduce_num_uses(&mut self) {
        self.num_uses -= 1;
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
