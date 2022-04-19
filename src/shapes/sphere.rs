use crate::{material::Material, matrix::Matrix, ray::Ray, tuple::Tuple};

#[derive(PartialEq)]
pub struct Sphere {
    center: Tuple,
    pub material: Material,
    pub transform: Matrix,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Default::default(),
            material: Default::default(),
            transform: Default::default(),
        }
    }

    pub fn glass() -> Self {
        let material = Material {
            transparency: 1.0,
            refractive_index: 1.5,
            ..Default::default()
        };

        Self {
            center: Default::default(),
            material,
            transform: Default::default(),
        }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Option<Vec<f64>> {
        let sphere_to_ray = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Option::None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        let mut values = vec![t1, t2];
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Option::Some(values)
    }

    pub fn local_normal(&self, object_point: Tuple) -> Tuple {
        object_point - Tuple::point(0.0, 0.0, 0.0)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}
