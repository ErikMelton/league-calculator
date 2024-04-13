pub struct Damage {
    pub(crate) physical_component: f32,
    pub(crate) magical_component: f32,
    pub(crate) true_component: f32,
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
}
