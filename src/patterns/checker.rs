use crate::{color::Color, floating_point::FloatingPoint, tuple::Tuple};

pub struct CheckerPattern {
    a: Color,
    b: Color,
}

impl CheckerPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn color_at(&self, point: &Tuple) -> Color {
        let n = point.x().floor() + point.y().floor() + point.z().floor();
        if FloatingPoint::equals(n % 2.0, 0.0) {
            self.a
        } else {
            self.b
        }
    }
}
