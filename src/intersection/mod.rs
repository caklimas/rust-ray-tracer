use crate::sphere::Sphere;

#[cfg(test)]
mod tests;

pub struct Intersection {
    pub object: Sphere,
    pub value: f64
}

impl Intersection {
    pub fn new(object: Sphere, value: f64) -> Self {
        Intersection {
            object,
            value
        }
    }
}