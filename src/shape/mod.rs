use crate::{material::Material, matrix::Matrix, ray::Ray, tuple::Tuple};

pub trait Shape {
    fn get_material(&self) -> &Material;
    fn get_transform(&self) -> &Matrix;

    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.get_transform().inverse() * world_point;
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let world_normal = self.get_transform().inverse().transpose() * object_normal;
        let world_normal = Tuple::vector(world_normal.x(), world_normal.y(), world_normal.z());
        world_normal.normalize()
    }

    fn transform_ray(&self, ray: &Ray) -> Ray {
        ray.transform(&self.get_transform().inverse())
    }
}
