use crate::{color::Color, matrix::Matrix, tuple::Tuple};

use super::Pattern;

pub struct TestPattern {
    transform: Matrix,
}

impl TestPattern {
    pub fn new() -> Self {
        TestPattern {
            transform: Matrix::identity(4),
        }
    }
}

impl Pattern for TestPattern {
    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn color_at(&self, point: &Tuple) -> Color {
        Color::new(point.x(), point.y(), point.z())
    }
}
