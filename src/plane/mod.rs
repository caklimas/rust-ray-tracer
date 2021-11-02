use crate::{
    intersection::Intersection, material::Material, matrix::Matrix, ray::Ray, shape::Shape,
    tuple::Tuple,
};

#[cfg(test)]
mod tests;

pub struct Plane {
    material: Material,
    transform: Matrix,
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

    fn local_intersect(&self, _ray: &Ray) -> Vec<Intersection> {
        todo!()
    }

    fn local_normal(&self, _object_point: Tuple) -> Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
    }
}
