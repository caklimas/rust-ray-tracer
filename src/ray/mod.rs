use crate::tuple::Tuple;

#[cfg(test)]
mod tests;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        if !origin.is_point() {
            panic!("Origin must be a point");
        }

        if !direction.is_vector() {
            panic!("Direction must be a vector");
        }

        Self {
            origin,
            direction
        }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin.add(&self.direction.multiply(t))
    }
}