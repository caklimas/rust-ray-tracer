use crate::{
    intersection::Intersection, material::Material, matrix::Matrix, ray::Ray, shape::Shape,
    tuple::Tuple,
};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    center: Tuple,
    pub material: Material,
    pub transform: Matrix,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Default::default(),
            material: Default::default(),
            transform: Default::default(),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let new_ray = ray.transform(&self.transform.inverse());
        let mut intersections = Vec::new();
        let sphere_to_ray = new_ray.origin - self.center;
        let a = new_ray.direction.dot(&new_ray.direction);
        let b = 2.0 * new_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return intersections;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        intersections.push(Intersection::new(self, t1));
        intersections.push(Intersection::new(self, t2));
        intersections.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());

        intersections
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

impl Shape for Sphere {
    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_transform(&self) -> &Matrix {
        &self.transform
    }
}
