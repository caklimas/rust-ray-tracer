use crate::{
    intersection::Intersection, material::Material, matrix::Matrix, ray::Ray, tuple::Tuple,
};

pub trait Shape {
    fn get_material(&self) -> &Material;
    fn get_transform(&self) -> &Matrix;
    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn local_normal(&self, object_point: Tuple) -> Tuple;

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let new_ray = self.transform_ray(ray);
        self.local_intersect(&new_ray)
    }

    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.get_transform().inverse() * world_point;
        let object_normal = self.local_normal(object_point);
        let world_normal = self.get_transform().inverse().transpose() * object_normal;
        let world_normal = Tuple::vector(world_normal.x(), world_normal.y(), world_normal.z());
        world_normal.normalize()
    }

    fn transform_ray(&self, ray: &Ray) -> Ray {
        ray.transform(&self.get_transform().inverse())
    }
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.get_material() == other.get_material() && self.get_transform() == other.get_transform()
    }
}
