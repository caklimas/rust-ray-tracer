use crate::{color::Color, tuple::Tuple};

pub mod stripe;

#[cfg(test)]
mod tests;

pub trait Pattern {
    fn color_at(&self, point: &Tuple) -> Color;
}
