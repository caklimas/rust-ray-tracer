use crate::{
    intersection::Intersection, material::Material, matrix::Matrix, ray::Ray, tuple::Tuple,
};

use self::{plane::Plane, sphere::Sphere};

pub mod plane;
pub mod sphere;

#[cfg(test)]
mod tests;

pub struct Shape {
    shape_type: ShapeType,
}

impl Shape {
    pub fn new(shape_type: ShapeType) -> Self {
        Self { shape_type }
    }

    pub fn get_material(&self) -> &Material {
        match &self.shape_type {
            ShapeType::Plane(shape) => &shape.material,
            ShapeType::Sphere(shape) => &shape.material,
        }
    }

    pub fn get_transform(&self) -> &Matrix {
        match &self.shape_type {
            ShapeType::Plane(shape) => &shape.transform,
            ShapeType::Sphere(shape) => &shape.transform,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let new_ray = self.transform_ray(ray);
        self.local_intersect(&new_ray)
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let value = match &self.shape_type {
            ShapeType::Plane(shape) => shape.local_intersect(ray),
            ShapeType::Sphere(shape) => shape.local_intersect(ray),
        };

        match value {
            Some(ts) => ts
                .iter()
                .map(|&t| Intersection::new(self, t))
                .collect::<Vec<_>>(),
            None => vec![],
        }
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.get_transform().inverse() * world_point;
        let object_normal = self.local_normal(object_point);
        let world_normal = self.get_transform().inverse().transpose() * object_normal;
        let world_normal = Tuple::vector(world_normal.x(), world_normal.y(), world_normal.z());
        world_normal.normalize()
    }

    pub fn local_normal(&self, object_point: Tuple) -> Tuple {
        match &self.shape_type {
            ShapeType::Plane(shape) => shape.local_normal(object_point),
            ShapeType::Sphere(shape) => shape.local_normal(object_point),
        }
    }

    pub fn transform_ray(&self, ray: &Ray) -> Ray {
        ray.transform(&self.get_transform().inverse())
    }
}

pub enum ShapeType {
    Plane(Plane),
    Sphere(Sphere),
}
