use crate::{
    color::Color,
    intersection::Intersection,
    matrix::{
        transformation::{scale, translate},
        Matrix,
    },
    point_light::PointLight,
    ray::Ray,
    tuple::Tuple,
};

use super::{view_transform, World};

#[test]
fn intersect_test() {
    let world: World = Default::default();
    let xs = world.intersect(&Ray::new(
        Tuple::point(0.0, 0.0, -5.0),
        Tuple::vector(0.0, 0.0, 1.0),
    ));

    assert_eq!(4, xs.len());
    assert_eq!(4.0, xs[0].value);
    assert_eq!(4.5, xs[1].value);
    assert_eq!(5.5, xs[2].value);
    assert_eq!(6.0, xs[3].value);
}

#[test]
fn shade_hit_test() {
    let world: World = Default::default();
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape = &world.objects[0];
    let intersection = Intersection::new(shape, 4.0);
    let comps = intersection.prepare_computations(&ray);

    let color = world.shade_hit(&comps);

    assert_eq!(Color::new(0.38066, 0.47583, 0.2855), color);
}

#[test]
fn shade_hit_from_inside_test() {
    let mut world: World = Default::default();
    world.light = PointLight::new(Color::white(), Tuple::point(0.0, 0.25, 0.0));
    let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape = &world.objects[1];
    let intersection = Intersection::new(shape, 0.5);
    let comps = intersection.prepare_computations(&ray);

    let color = world.shade_hit(&comps);

    assert_eq!(Color::new(0.90498, 0.90498, 0.90498), color);
}

#[test]
fn color_at_ray_misses() {
    let world: World = Default::default();
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));

    let c = world.color_at(&ray);

    assert_eq!(Color::black(), c);
}

#[test]
fn color_at_ray_hit() {
    let world: World = Default::default();
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

    let color = world.color_at(&ray);

    assert_eq!(Color::new(0.38066, 0.47583, 0.2855), color);
}

#[test]
fn color_at_behind_ray() {
    let mut world: World = Default::default();
    let outer = &mut world.objects[0];
    outer.material.ambient = 1.0;
    let inner = &mut world.objects[1];
    inner.material.ambient = 1.0;
    let ray = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));

    let c = world.color_at(&ray);

    assert_eq!(world.objects[1].material.color, c);
}

#[test]
fn view_transform_default_orientation_test() {
    let t = view_transform(
        &Tuple::point(0.0, 0.0, 0.0),
        &Tuple::point(0.0, 0.0, -1.0),
        &Tuple::vector(0.0, 1.0, 0.0),
    );

    assert_eq!(Matrix::identity(4), t);
}

#[test]
fn view_transform_positive_z_test() {
    let t = view_transform(
        &Tuple::point(0.0, 0.0, 0.0),
        &Tuple::point(0.0, 0.0, 1.0),
        &Tuple::vector(0.0, 1.0, 0.0),
    );

    assert_eq!(scale(-1.0, 1.0, -1.0), t);
}

#[test]
fn view_transformation_moves_world_test() {
    let t = view_transform(
        &Tuple::point(0.0, 0.0, 8.0),
        &Tuple::point(0.0, 0.0, 0.0),
        &Tuple::vector(0.0, 1.0, 0.0),
    );

    assert_eq!(translate(0.0, 0.0, -8.0), t);
}

#[test]
fn view_transformation_arbitrary_test() {
    let t = view_transform(
        &Tuple::point(1.0, 3.0, 2.0),
        &Tuple::point(4.0, -2.0, 8.0),
        &Tuple::vector(1.0, 1.0, 0.0),
    );

    assert_eq!(
        Matrix::new(
            4,
            4,
            Some(vec![
                vec![-0.50709, 0.50709, 0.67612, -2.36643],
                vec![0.76772, 0.60609, 0.12122, -2.82843],
                vec![-0.35857, 0.59761, -0.71714, 0.00000],
                vec![0.00000, 0.00000, 0.00000, 1.00000]
            ])
        ),
        t
    );
}
