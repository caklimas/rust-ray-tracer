use crate::{floating_point::EPSILON, material::Material, matrix::Matrix, ray::Ray, tuple::Tuple};

pub struct Plane {
    pub material: Material,
    pub transform: Matrix,
}

impl Plane {
    pub fn new() -> Self {
        Self {
            material: Default::default(),
            transform: Default::default(),
        }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Option<Vec<f64>> {
        if ray.direction.y().abs() < EPSILON {
            return Option::None;
        }

        let t = (-ray.origin.y()) / ray.direction.y();
        Option::Some(vec![t])
    }

    pub fn local_normal(&self, _object_point: Tuple) -> Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}
