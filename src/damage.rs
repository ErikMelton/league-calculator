use std::ops::AddAssign;

pub struct Damage {
    pub(crate) physical_component: f32,
    pub(crate) magical_component: f32,
    pub(crate) true_component: f32,
}

impl AddAssign for Damage {
    fn add_assign(&mut self, rhs: Self) {
        self.physical_component += rhs.physical_component;
        self.magical_component += rhs.magical_component;
        self.true_component += rhs.true_component;
    }
}

impl Damage {
    pub fn new(physical_component: f32, magical_component: f32, true_component: f32) -> Self {
        Damage {
            physical_component,
            magical_component,
            true_component,
        }
    }

    pub fn total(&self) -> f32 {
        self.physical_component + self.magical_component + self.true_component
    }

    pub fn reduce_physical_damage(&mut self, reduction: f32) {
        self.physical_component -= reduction;
    }

    pub fn reduce_magical_damage(&mut self, reduction: f32) {
        self.magical_component -= reduction;
    }

    pub fn reduce_true_damage(&mut self, reduction: f32) {
        self.true_component -= reduction;
    }
}
