use crate::{
    floating_point::FloatingPoint,
    ray::Ray,
    shapes::{cube::Cube, Shape, ShapeType},
    tuple::Tuple,
};

#[test]
fn ray_intersects_cube() {
    let cube = Shape::new(ShapeType::Cube(Cube::new()));
    let answers = vec![
        (
            Tuple::point(5.0, 0.5, 0.0),
            Tuple::vector(-1.0, 0.0, 0.0),
            vec![4.0, 6.0],
        ),
        (
            Tuple::point(-5.0, 0.5, 0.0),
            Tuple::vector(1.0, 0.0, 0.0),
            vec![4.0, 6.0],
        ),
        (
            Tuple::point(0.5, 5.0, 0.0),
            Tuple::vector(0.0, -1.0, 0.0),
            vec![4.0, 6.0],
        ),
        (
            Tuple::point(0.5, -5.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
            vec![4.0, 6.0],
        ),
        (
            Tuple::point(0.5, 0.0, 5.0),
            Tuple::vector(0.0, 0.0, -1.0),
            vec![4.0, 6.0],
        ),
        (
            Tuple::point(0.5, 0.0, -5.0),
            Tuple::vector(0.0, 0.0, 1.0),
            vec![4.0, 6.0],
        ),
        (
            Tuple::point(0.0, 0.5, 0.0),
            Tuple::vector(0.0, 0.0, 1.0),
            vec![-1.0, 1.0],
        ),
    ];

    for answer in answers {
        let r = Ray::new(answer.0, answer.1);
        let local_intersect = cube.local_intersect(&r);

        assert_eq!(true, local_intersect.len() == 2);
        assert_eq!(
            true,
            FloatingPoint::equals(answer.2[0], local_intersect[0].value)
        );
        assert_eq!(
            true,
            FloatingPoint::equals(answer.2[1], local_intersect[1].value)
        );
    }
}

#[test]
fn ray_misses_cube() {
    let c = Shape::new(ShapeType::Cube(Cube::new()));
    let answers = vec![
        (
            Tuple::point(-2.0, 0.0, 0.0),
            Tuple::vector(0.2673, 0.5345, 0.8118),
        ),
        (
            Tuple::point(0.0, -2.0, 0.0),
            Tuple::vector(0.8018, 0.2673, 0.5345),
        ),
        (
            Tuple::point(0.0, 0.0, -2.0),
            Tuple::vector(0.5345, 0.8018, 0.2673),
        ),
        (Tuple::point(2.0, 0.0, 2.0), Tuple::vector(0.0, 0.0, -1.0)),
        (Tuple::point(0.0, 2.0, 2.0), Tuple::vector(0.0, -1.0, 0.0)),
        (Tuple::point(2.0, 2.0, 0.0), Tuple::vector(-1.0, 0.0, 0.0)),
    ];

    for answer in answers {
        let r = Ray::new(answer.0, answer.1);
        let local_intersect = c.local_intersect(&r);

        assert_eq!(true, local_intersect.is_empty());
    }
}

#[test]
fn normal_on_cube() {
    let c = Shape::new(ShapeType::Cube(Cube::new()));
    let answers = vec![
        (Tuple::point(1.0, 0.5, -0.8), Tuple::vector(1.0, 0.0, 0.0)),
        (Tuple::point(-1.0, -0.2, 0.9), Tuple::vector(-1.0, 0.0, 0.0)),
        (Tuple::point(-0.4, 1.0, -0.1), Tuple::vector(0.0, 1.0, 0.0)),
        (Tuple::point(0.3, -1.0, -0.7), Tuple::vector(0.0, -1.0, 0.0)),
        (Tuple::point(-0.6, 0.3, 1.0), Tuple::vector(0.0, 0.0, 1.0)),
        (Tuple::point(0.4, 0.4, -1.0), Tuple::vector(0.0, 0.0, -1.0)),
        (Tuple::point(1.0, 1.0, 1.0), Tuple::vector(1.0, 0.0, 0.0)),
        (
            Tuple::point(-1.0, -1.0, -1.0),
            Tuple::vector(-1.0, 0.0, 0.0),
        ),
    ];

    for answer in answers {
        let normal = c.local_normal(answer.0);

        assert_eq!(answer.1, normal);
    }
}
