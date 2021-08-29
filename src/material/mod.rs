use std::ops::RangeInclusive;
use crate::color::Color;

#[cfg(test)]
mod tests;

pub const REFLECTION_RANGE: RangeInclusive<f64> = 0.0..=1.0;

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        if !REFLECTION_RANGE.contains(&ambient) {
            panic_reflection("ambient");
        } else if !REFLECTION_RANGE.contains(&diffuse) {
            panic_reflection("diffuse");
        } else if !REFLECTION_RANGE.contains(&specular) {
            panic_reflection("specular");
        }

        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess
        }
    }

    
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0
        }
    }
}

fn panic_reflection(name: &str) {
    panic!("{} must be between {} and {}", name, REFLECTION_RANGE.start(), REFLECTION_RANGE.end());
}