use crate::{color::Color, tuple::Tuple};

pub struct GradientPattern {
    a: Color,
    b: Color,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn color_at(&self, point: &Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x() - point.x().floor();
        self.a + distance * fraction
    }
}
