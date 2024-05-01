use std::ops::{Add, AddAssign};

#[derive(Clone, Copy)]
pub struct Damage {
    pub(crate) physical_component: f32,
    pub(crate) magical_component: f32,
    pub(crate) true_component: f32,
}

impl AddAssign for Damage {
    fn add_assign(&mut self, rhs: Self) {
        self.physical_component = self.physical_component + rhs.physical_component;
        self.magical_component = self.magical_component + rhs.magical_component;
        self.true_component = self.true_component + rhs.true_component;
    }
}

impl Add for Damage {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Damage {
            physical_component: self.physical_component + rhs.physical_component,
            magical_component: self.magical_component + rhs.magical_component,
            true_component: self.true_component + rhs.true_component,
        }
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

#[cfg(test)]
mod tests {
    use crate::damage::Damage;

    #[test]
    fn test_damage() {
        let mut damage = Damage::new(100.0, 100.0, 100.0);
        assert_eq!(damage.total(), 300.0);

        damage.reduce_physical_damage(50.0);
        assert_eq!(damage.total(), 250.0);
        assert_eq!(damage.physical_component, 50.0);

        damage.reduce_magical_damage(50.0);
        assert_eq!(damage.total(), 200.0);
        assert_eq!(damage.magical_component, 50.0);

        damage.reduce_true_damage(50.0);
        assert_eq!(damage.total(), 150.0);
        assert_eq!(damage.true_component, 50.0);
    }

    #[test]
    fn test_add_assign() {
        let mut damage1 = Damage::new(100.0, 100.0, 100.0);
        let damage2 = Damage::new(50.0, 50.0, 50.0);

        damage1 += damage2;

        assert_eq!(damage1.total(), 450.0);
        assert_eq!(damage1.physical_component, 150.0);
        assert_eq!(damage1.magical_component, 150.0);
        assert_eq!(damage1.true_component, 150.0);
    }

    #[test]
    fn test_add() {
        let damage1 = Damage::new(100.0, 100.0, 100.0);
        let damage2 = Damage::new(50.0, 50.0, 50.0);

        let damage3 = damage1 + damage2;

        assert_eq!(damage3.total(), 450.0);
        assert_eq!(damage3.physical_component, 150.0);
        assert_eq!(damage3.magical_component, 150.0);
        assert_eq!(damage3.true_component, 150.0);
    }

    #[test]
    fn test_clone() {
        let damage1 = Damage::new(100.0, 100.0, 100.0);
        let mut damage2 = damage1.clone();

        assert_eq!(damage2.total(), 300.0);
        assert_eq!(damage2.physical_component, 100.0);
        assert_eq!(damage2.magical_component, 100.0);
        assert_eq!(damage2.true_component, 100.0);

        damage2.physical_component = 50.0;
        damage2.magical_component = 50.0;
        damage2.true_component = 50.0;

        assert_eq!(damage1.total(), 300.0);
        assert_eq!(damage1.physical_component, 100.0);
        assert_eq!(damage1.magical_component, 100.0);
        assert_eq!(damage1.true_component, 100.0);

        assert_eq!(damage2.total(), 150.0);
        assert_eq!(damage2.physical_component, 50.0);
        assert_eq!(damage2.magical_component, 50.0);
        assert_eq!(damage2.true_component, 50.0);
    }

    #[test]
    fn test_copy() {
        let damage1 = Damage::new(100.0, 100.0, 100.0);
        let mut damage2 = damage1;

        assert_eq!(damage2.total(), 300.0);
        assert_eq!(damage2.physical_component, 100.0);
        assert_eq!(damage2.magical_component, 100.0);
        assert_eq!(damage2.true_component, 100.0);

        damage2.physical_component = 50.0;
        damage2.magical_component = 50.0;
        damage2.true_component = 50.0;

        assert_eq!(damage1.total(), 300.0);
        assert_eq!(damage1.physical_component, 100.0);
        assert_eq!(damage1.magical_component, 100.0);
        assert_eq!(damage1.true_component, 100.0);

        assert_eq!(damage2.total(), 150.0);
        assert_eq!(damage2.physical_component, 50.0);
        assert_eq!(damage2.magical_component, 50.0);
        assert_eq!(damage2.true_component, 50.0);
    }
}
