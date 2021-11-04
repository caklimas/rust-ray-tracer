use crate::{
    color::Color,
    intersection::{
        intersection_computation::IntersectionComputation, intersections::Intersections,
        Intersection,
    },
    matrix::{
        transformation::{scale, translate},
        Matrix,
    },
    point_light::PointLight,
    ray::Ray,
    shape::Shape,
    sphere::Sphere,
    tuple::Tuple,
};

#[cfg(test)]
mod tests;

pub struct World {
    pub light: PointLight,
    pub objects: Vec<Box<dyn Shape>>,
}

impl World {
    pub fn new(light: PointLight, objects: Vec<Box<dyn Shape>>) -> Self {
        Self { light, objects }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        for o in self.objects.iter() {
            intersections.append(&mut o.intersect(ray));
        }

        intersections.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
        intersections
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = Intersections::new(self.intersect(ray));
        let hit = intersections.hit();
        match hit {
            Some(i) => {
                let comps = i.prepare_computations(ray);
                self.shade_hit(i.object, &comps)
            }
            None => Color::black(),
        }
    }

    pub fn shade_hit(&self, object: &dyn Shape, computations: &IntersectionComputation) -> Color {
        computations.object.get_material().lighting(
            object,
            &self.light,
            &computations.point,
            &computations.eye_v,
            &computations.normal_v,
            self.is_shadowed(&computations.over_point),
        )
    }

    pub fn is_shadowed(&self, point: &Tuple) -> bool {
        let v = &self.light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(*point, direction);
        let intersect = self.intersect(&r);
        let intersections = Intersections::new(intersect);
        if let Some(h) = intersections.hit() {
            h.value < distance
        } else {
            false
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Color::white(), Tuple::point(-10.0, 10.0, -10.0));
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.transform = scale(0.5, 0.5, 0.5);

        Self::new(light, vec![Box::new(s1), Box::new(s2)])
    }
}

pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix {
    let forward: Tuple = (to - from).normalize();
    let left = forward.cross(&up.normalize());
    let true_up = left.cross(&forward);
    let negated_forward = forward.negate();
    let negated_from = from.negate();
    let orientation = Matrix::new(
        4,
        4,
        Some(vec![
            vec![left.x(), left.y(), left.z(), 0.0],
            vec![true_up.x(), true_up.y(), true_up.z(), 0.0],
            vec![
                negated_forward.x(),
                negated_forward.y(),
                negated_forward.z(),
                0.0,
            ],
            vec![0.0, 0.0, 0.0, 1.0],
        ]),
    );

    orientation * translate(negated_from.x(), negated_from.y(), negated_from.z())
}
