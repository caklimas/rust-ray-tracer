use crate::{
    floating_point::EPSILON, intersection::Intersection, material::Material, matrix::Matrix,
    ray::Ray, shape::Shape, tuple::Tuple,
};

#[cfg(test)]
mod tests;

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
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}

impl Shape for Plane {
    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut xs = Vec::new();
        if ray.direction.y().abs() < EPSILON {
            return xs;
        }

        let t = (-ray.origin.y()) / ray.direction.y();
        let intersection = Intersection::new(self, t);
        xs.push(intersection);
        xs
    }

    fn local_normal(&self, _object_point: Tuple) -> Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
    }
}
