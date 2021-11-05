use crate::{color::Color, matrix::Matrix, tuple::Tuple};

use super::Pattern;

pub struct Gradient {
    a: Color,
    b: Color,
    transform: Matrix,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Default::default(),
        }
    }
}

impl Pattern for Gradient {
    fn get_transform(&self) -> &crate::matrix::Matrix {
        &self.transform
    }

    fn color_at(&self, point: &Tuple) -> crate::color::Color {
        let distance = self.b - self.a;
        let fraction = point.x() - point.x().floor();
        self.a + distance * fraction
    }
}
