#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Interval { min: f64::INFINITY, max: f64::NEG_INFINITY }
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn size(self) -> f64 {
        self.max - self.min
    }

    pub fn contains(self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_size() {
        assert_eq!(2.0, Interval::new(-1.0, 1.0).size());
        assert_eq!(0.5, Interval::new(0.0, 0.5).size());
        assert_eq!(0.0, Interval::new(1.0, 1.0).size());
    }

    #[test]
    fn interval_contains() {
        let interval = Interval::new(-1.0, 1.0);

        assert!(interval.contains(-1.0));
        assert!(interval.contains(0.3));
        assert!(interval.contains(1.0));
    }

    #[test]
    fn interval_surrounds() {
        let interval = Interval::new(-1.0, 1.0);

        assert!(!interval.surrounds(-1.0));
        assert!(interval.surrounds(0.3));
        assert!(!interval.surrounds(1.0));
    }
}
