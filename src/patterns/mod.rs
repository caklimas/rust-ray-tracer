use crate::{color::Color, matrix::Matrix, shape::Shape, tuple::Tuple};

use self::{
    checker::CheckerPattern, gradient::GradientPattern, ring::RingPattern, stripe::StripePattern,
    test_pattern::TestPattern,
};

pub mod checker;
pub mod gradient;
pub mod ring;
pub mod stripe;
pub mod test_pattern;

#[cfg(test)]
mod tests;

pub struct Pattern {
    pub pattern_type: PatternType,
    pub transform: Matrix,
}

impl Pattern {
    pub fn new(pattern_type: PatternType) -> Self {
        Self {
            pattern_type,
            transform: Default::default(),
        }
    }

    pub fn color_at_object(&self, object: &dyn Shape, point: &Tuple) -> Color {
        let object_space = &object.get_transform().inverse() * point;
        let pattern_space = &self.transform.inverse() * object_space;
        self.color_at(&pattern_space)
    }

    pub fn color_at(&self, point: &Tuple) -> Color {
        match &self.pattern_type {
            PatternType::Checker(pattern) => pattern.color_at(point),
            PatternType::Gradient(pattern) => pattern.color_at(point),
            PatternType::Ring(pattern) => pattern.color_at(point),
            PatternType::Stripe(pattern) => pattern.color_at(point),
            PatternType::Test => TestPattern::color_at(point),
        }
    }
}

pub enum PatternType {
    Checker(CheckerPattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
    Stripe(StripePattern),
    Test,
}
