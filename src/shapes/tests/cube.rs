use crate::{
    ray::Ray,
    shapes::{cube::Cube, Shape, ShapeType},
    tuple::Tuple,
};

#[test]
fn local_intersect_test() {
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
        assert_eq!(answer.2[0], local_intersect[0].value);
        assert_eq!(answer.2[1], local_intersect[1].value);
    }
}
