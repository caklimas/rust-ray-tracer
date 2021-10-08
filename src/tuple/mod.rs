pub mod ops;

#[cfg(test)]
mod tests;

pub const POINT_W: f64 = 1.0;
pub const VECTOR_W: f64 = 0.0;

#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, POINT_W)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, VECTOR_W)
    }

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
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

    pub fn negate(&self) -> Self {
        Self {
            x: -self.x(),
            y: -self.y(),
            z: -self.z(),
            w: -self.w(),
        }
    }

    pub fn magnitude(&self) -> f64 {
        let sum = self.x().powi(2) + self.y().powi(2) + self.z().powi(2) + self.w().powi(2);

        sum.sqrt()
    }

    pub fn normalize(&self) -> Self {
        if !self.is_vector() {
            panic!("Normalize can only be done on vectors");
        }

        let magnitude = self.magnitude();
        Self {
            x: self.x() / magnitude,
            y: self.y() / magnitude,
            z: self.z() / magnitude,
            w: self.w() / magnitude,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        (self.x() * other.x())
            + (self.y() * other.y())
            + (self.z() * other.z())
            + (self.w() * other.w())
    }

    pub fn cross(&self, other: &Self) -> Self {
        if !self.is_vector() || !other.is_vector() {
            panic!("Cross product can only be done on vectors");
        }

        Self::vector(
            (self.y() * other.z()) - (self.z() * other.y()),
            (self.z() * other.x()) - (self.x() * other.z()),
            (self.x() * other.y()) - (self.y() * other.x()),
        )
    }

    pub fn reflect(&self, normal: &Tuple) -> Tuple {
        let dot = (*normal * 2.0) * self.dot(&normal);
        self - &dot
    }
}
