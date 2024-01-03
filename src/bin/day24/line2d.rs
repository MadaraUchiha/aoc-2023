#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2D {
    pub start: (i64, i64),
    pub end: (i64, i64),
}

impl Line2D {
    pub fn new(start: (i64, i64), end: (i64, i64)) -> Self {
        Self { start, end }
    }

    pub fn slope(&self) -> f64 {
        let dx = self.end.0 - self.start.0;
        let dy = self.end.1 - self.start.1;
        dy as f64 / dx as f64
    }

    pub fn y_intercept(&self) -> f64 {
        self.start.1 as f64 - self.slope() * self.start.0 as f64
    }

    pub fn intersection_point(&self, other: &Self) -> Option<(f64, f64)> {
        let m1 = self.slope();
        let m2 = other.slope();
        if m1 == m2 {
            return None;
        }
        let b1 = self.y_intercept();
        let b2 = other.y_intercept();
        let x = (b2 - b1) / (m1 - m2);
        let y = m1 * x + b1;
        Some((x, y))
    }

    pub fn is_point_after_start(&self, point: (f64, f64)) -> bool {
        let signum = (
            (point.0 - self.start.0 as f64).signum(),
            (point.1 - self.start.1 as f64).signum(),
        );
        let velocity_direction = (
            (self.end.0 - self.start.0).signum() as f64,
            (self.end.1 - self.start.1).signum() as f64,
        );

        signum == velocity_direction
    }
}
