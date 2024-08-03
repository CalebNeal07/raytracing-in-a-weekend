#[derive(Clone, Copy)]
pub struct Interval(pub f64, pub f64);

impl Interval {
    pub const EMPTY: Interval = Interval(f64::INFINITY, f64::NEG_INFINITY);
    pub const UNIVERSE: Interval = Interval(f64::NEG_INFINITY, f64::INFINITY);

    pub fn min(self) -> f64 {
        self.0
    }

    pub fn max(self) -> f64 {
        self.1
    }

    pub fn size(self) -> f64 {
        self.1 - self.0
    }

    pub fn contains(self, x: f64) -> bool {
        self.0 <= x && x <= self.1
    }

    pub fn surrounds(self, x: f64) -> bool {
        self.0 < x && x < self.1
    }

    pub fn clamp(self, x: f64) -> f64 {
        if (x < self.0) {
            self.0
        } else if x > self.1 {
            self.1
        } else {
            x
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval(f64::INFINITY, f64::NEG_INFINITY)
    }
}
