use crate::floating_point::FloatingPoint;

#[cfg(test)]
mod tests;

pub const POINT_W: f64 = 1.0;
pub const VECTOR_W: f64 = 0.0;

pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64
}

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, false)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, true)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn w(&self) -> f64 {
        self.w
    }

    pub fn is_vector(&self) -> bool {
        self.w == VECTOR_W
    }

    pub fn is_point(&self) -> bool {
        !self.is_vector()
    }

    pub fn add(&self, other: &Tuple) -> Tuple {
        if self.is_point() && other.is_point() {
            panic!("Can't add two points together");
        }

        Tuple {
            x: self.x() + other.x(),
            y: self.y() + other.y(),
            z: self.z() + other.z(),
            w: self.w() + other.w()
        }
    }

    pub fn equals(&self, other: &Tuple) -> bool {
        FloatingPoint::equals(self.x(), other.x()) &&
        FloatingPoint::equals(self.y(), other.y()) &&
        FloatingPoint::equals(self.z(), other.z()) &&
        FloatingPoint::equals(self.w(), other.w())
    }

    fn new(x: f64, y: f64, z: f64, is_vector: bool) -> Self {
        let w = if is_vector { VECTOR_W } else { POINT_W };
        Tuple {
            x,
            y,
            z,
            w
        }
    }
}