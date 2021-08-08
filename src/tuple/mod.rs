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
        return self.w == 1.0;
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