use crate::{
    color::Color,
    floating_point::EPSILON,
    intersection::{
        intersections::Intersections, prepare_computation::PrepareComputationConfig, Intersection,
    },
    matrix::{
        transformation::{scale, translate},
        Matrix,
    },
    plane::Plane,
    point_light::PointLight,
    ray::Ray,
    sphere::Sphere,
    tuple::Tuple,
    world::DEFAULT_REMAINING,
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
    let intersection = Intersection::new(&*world.objects[0], 4.0);
    let comps = intersection.prepare_computations(&ray, Option::None);

    let color = world.shade_hit(intersection.object, &comps, DEFAULT_REMAINING);

    assert_eq!(Color::new(0.38066, 0.47583, 0.2855), color);
}

#[test]
fn shade_hit_from_inside_test() {
    let mut world: World = Default::default();
    world.light = PointLight::new(Color::white(), Tuple::point(0.0, 0.25, 0.0));
    let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let intersection = Intersection::new(&*world.objects[1], 0.5);
    let comps = intersection.prepare_computations(&ray, Option::None);

    let color = world.shade_hit(intersection.object, &comps, DEFAULT_REMAINING);

    assert_eq!(Color::new(0.90498, 0.90498, 0.90498), color);
}

#[test]
fn color_at_ray_misses() {
    let world: World = Default::default();
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));

    let c = world.color_at(&ray, DEFAULT_REMAINING);

    assert_eq!(Color::black(), c);
}

#[test]
fn color_at_ray_hit() {
    let world: World = Default::default();
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

    let color = world.color_at(&ray, DEFAULT_REMAINING);

    assert_eq!(Color::new(0.38066, 0.47583, 0.2855), color);
}

#[test]
fn color_at_behind_ray() {
    let light = PointLight::new(Color::white(), Tuple::point(-10.0, 10.0, -10.0));
    let mut s1 = Sphere::new();
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    s1.material.ambient = 1.0;

    let mut s2 = Sphere::new();
    s2.transform = scale(0.5, 0.5, 0.5);
    s2.material.ambient = 1.0;

    let world = World::new(light, vec![Box::new(s1), Box::new(s2)]);
    let ray = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));

    let c = world.color_at(&ray, DEFAULT_REMAINING);

    assert_eq!(world.objects[1].get_material().color, c);
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

#[test]
fn no_shadow_nothing_colinear_with_point_and_light() {
    let world: World = Default::default();
    let point = Tuple::point(0.0, 10.0, 0.0);

    assert_eq!(false, world.is_shadowed(&point));
}

#[test]
fn shadow_object_between_point_and_light() {
    let world: World = Default::default();
    let point = Tuple::point(10.0, -10.0, 10.0);

    assert_eq!(true, world.is_shadowed(&point));
}

#[test]
fn no_shadow_object_behind_light() {
    let world: World = Default::default();
    let point = Tuple::point(-20.0, 20.0, -20.0);

    assert_eq!(false, world.is_shadowed(&point));
}

#[test]
fn no_shadow_object_behind_point() {
    let world: World = Default::default();
    let point = Tuple::point(-2.0, 2.0, -2.0);

    assert_eq!(false, world.is_shadowed(&point));
}

#[test]
fn shade_hit_intersection_in_shadow_test() {
    let mut world: World = World::new(
        PointLight::new(Color::white(), Tuple::point(0.0, 0.0, -10.0)),
        Vec::new(),
    );
    world.objects.push(Box::new(Sphere::new()));

    let mut sphere2 = Sphere::new();
    sphere2.transform = translate(0.0, 0.0, 10.0);
    world.objects.push(Box::new(sphere2));

    let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
    let i = Intersection::new(&*world.objects[1], 4.0);

    let comps = i.prepare_computations(&ray, Option::None);
    let c = world.shade_hit(i.object, &comps, DEFAULT_REMAINING);

    assert_eq!(Color::new(0.1, 0.1, 0.1), c);
}

#[test]
fn shade_hit_reflective_material() {
    let value = (2.0_f64).sqrt() / 2.0;
    let mut world = World::default();
    let mut shape = Plane::new();
    shape.material.reflective = 0.5;
    shape.transform = translate(0.0, -1.0, 0.0);
    let r = Ray::new(
        Tuple::point(0.0, 0.0, -3.0),
        Tuple::vector(0.0, -value, value),
    );
    world.objects.push(Box::new(shape));
    let i = Intersection::new(&*world.objects[2], (2.0_f64).sqrt());
    let comps = i.prepare_computations(&r, Option::None);

    let color = world.shade_hit(&*world.objects[2], &comps, DEFAULT_REMAINING);

    assert_eq!(Color::new(0.87676, 0.92434, 0.82917), color);
}

#[test]
fn hit_should_offset_point() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut shape = Sphere::new();
    shape.transform = translate(0.0, 0.0, 1.0);

    let i = Intersection::new(&shape, 5.0);
    let comps = i.prepare_computations(&r, Option::None);

    assert_eq!(true, comps.over_point.z() < -(EPSILON / 2.0));
    assert_eq!(true, comps.point.z() > comps.over_point.z());
}

#[test]
fn reflected_color_for_nonreflective_material() {
    let light = PointLight::new(Color::white(), Tuple::point(-10.0, 10.0, -10.0));
    let mut s1 = Sphere::new();
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    let mut s2 = Sphere::new();
    s2.transform = scale(0.5, 0.5, 0.5);
    s2.material.ambient = 1.0;
    let w = World::new(light, vec![Box::new(s1), Box::new(s2)]);
    let i = Intersection::new(&*w.objects[0], 1.0);
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let comps = i.prepare_computations(&r, Option::None);

    let color = w.reflected_color(&comps, DEFAULT_REMAINING);

    assert_eq!(Color::black(), color);
}

#[test]
fn reflected_color_for_reflective_material() {
    let v = (2.0_f64).sqrt() / 2.0;
    let mut w = World::default();
    let mut shape = Plane::default();
    shape.material.reflective = 0.5;
    shape.transform = translate(0.0, -1.0, 0.0);
    w.objects.push(Box::new(shape));
    let r = Ray::new(Tuple::point(0.0, 0.0, -3.0), Tuple::vector(0.0, -v, v));
    let i = Intersection::new(&*w.objects[2], (2.0_f64).sqrt());
    let comps = i.prepare_computations(&r, Option::None);

    let color = w.reflected_color(&comps, DEFAULT_REMAINING);

    assert_eq!(Color::new(0.19033, 0.23791, 0.14274), color);
}

#[test]
fn color_at_mutually_reflective_surfaces() {
    let mut lower = Plane::new();
    lower.material.reflective = 1.0;
    lower.transform = translate(0.0, -1.0, 0.0);
    let mut upper = Plane::new();
    upper.material.reflective = 1.0;
    upper.transform = translate(0.0, 1.0, 0.0);
    let world = World::new(
        PointLight::new(Color::white(), Tuple::point(0.0, 0.0, 0.0)),
        vec![Box::new(lower), Box::new(upper)],
    );
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));

    world.color_at(&r, DEFAULT_REMAINING);
}

#[test]
fn reflected_color_max_recursion_depth() {
    let v = (2.0_f64).sqrt() / 2.0;
    let mut w = World::default();
    let mut shape = Plane::new();
    shape.material.reflective = 0.5;
    shape.transform = translate(0.0, -1.0, 0.0);
    w.objects.push(Box::new(shape));
    let r = Ray::new(Tuple::point(0.0, 0.0, -3.0), Tuple::vector(0.0, -v, v));
    let i = Intersection::new(&*w.objects[2], (2.0_f64).sqrt());
    let comps = i.prepare_computations(&r, Option::None);

    let color = w.reflected_color(&comps, 0);

    assert_eq!(Color::black(), color);
}

#[test]
fn refracted_color_opaque_surface() {
    let w = World::default();
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let xs = Intersections::new(vec![
        Intersection::new(&*w.objects[0], 4.0),
        Intersection::new(&*w.objects[0], 6.0),
    ]);
    let mut config = PrepareComputationConfig::new(&xs);
    let comps = xs.collection[0].prepare_computations(&r, Option::Some(&mut config));

    let c = w.refracted_color(&comps, 5.0);

    assert_eq!(Color::black(), c);
}
