use crate::{floating_point::FloatingPoint, tuple::Tuple};
use super::Matrix;

#[test]
fn new_test() {
    let mut elements = Vec::new();
    elements.push(vec![1.0, 2.0, 3.0, 4.0]);
    elements.push(vec![5.5, 6.5, 7.5, 8.5]);
    elements.push(vec![9.0, 10.0, 11.0, 12.0]);
    elements.push(vec![13.5, 14.5, 15.5, 16.5]);

    let matrix = Matrix::new(4, 4, Option::Some(elements));

    assert_eq!(true, FloatingPoint::equals(1.0, matrix.get(0, 0)));
    assert_eq!(true, FloatingPoint::equals(4.0, matrix.get(0, 3)));
    assert_eq!(true, FloatingPoint::equals(5.5, matrix.get(1, 0)));
    assert_eq!(true, FloatingPoint::equals(7.5, matrix.get(1, 2)));
    assert_eq!(true, FloatingPoint::equals(11.0, matrix.get(2, 2)));
    assert_eq!(true, FloatingPoint::equals(13.5, matrix.get(3, 0)));
    assert_eq!(true, FloatingPoint::equals(15.5, matrix.get(3, 2)));
}

#[test]
fn new_empty_test() {
    let rows = 4;
    let columns = 4;
    let matrix = Matrix::new(rows, columns, Option::None);
    for row in 0..rows {
        for column in 0..columns {
            assert_eq!(true, FloatingPoint::equals(matrix.elements[row][column], 0.0));
        }
    }
}

#[test]
#[should_panic]
fn new_row_panic_test() {
    let mut elements = Vec::new();
    elements.push(vec![1.0, 2.0, 3.0, 4.0]);
    elements.push(vec![5.5, 6.5, 7.5, 8.5]);
    elements.push(vec![9.0, 10.0, 11.0, 12.0]);

    Matrix::new(4, 4, Option::Some(elements));
}

#[test]
#[should_panic]
fn new_column_panic_test() {
    let mut elements = Vec::new();
    elements.push(vec![1.0, 2.0, 3.0, 4.0]);
    elements.push(vec![5.5, 6.5, 7.5, 8.5]);
    elements.push(vec![9.0, 10.0, 11.0, 12.0]);
    elements.push(vec![9.0, 10.0, 11.0]);

    Matrix::new(4, 4, Option::Some(elements));
}

#[test]
fn identity_test() {
    let identity = Matrix::identity(4);
    let mut elements = Vec::new();
    elements.push(vec![0.0, 1.0, 2.0, 4.0]);
    elements.push(vec![1.0, 2.0, 4.0, 8.0]);
    elements.push(vec![2.0, 4.0, 8.0, 16.0]);
    elements.push(vec![4.0, 8.0, 16.0, 32.0]);

    let matrix = Matrix::new(4, 4, Option::Some(elements));

    let result = matrix.multiply(&identity);

    assert_eq!(result, matrix);
}

#[test]
fn identity_tuple_test() {
    let identity = Matrix::identity(4);
    let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);

    let result = identity.multiply_tuple(&tuple);

    assert_eq!(result, tuple);
}

#[test]
fn equals_test() {
    let mut elements = Vec::new();
    elements.push(vec![1.0, 2.0, 3.0, 4.0]);
    elements.push(vec![5.5, 6.5, 7.5, 8.5]);
    elements.push(vec![9.0, 10.0, 11.0, 12.0]);
    elements.push(vec![13.5, 14.5, 15.5, 16.5]);

    let mut other_elements = Vec::new();
    other_elements.push(vec![1.0, 2.0, 3.0, 4.0]);
    other_elements.push(vec![5.5, 6.5, 7.5, 8.5]);
    other_elements.push(vec![9.0, 10.0, 11.0, 12.0]);
    other_elements.push(vec![13.5, 14.5, 15.5, 16.5]);

    let matrix = Matrix::new(4, 4, Option::Some(elements));
    let other = Matrix::new(4, 4, Option::Some(other_elements));

    assert_eq!(matrix, other);
}

#[test]
fn not_equals_test() {
    let mut elements = Vec::new();
    elements.push(vec![1.0, 2.0, 3.0, 4.0]);
    elements.push(vec![5.5, 6.5, 7.5, 8.5]);
    elements.push(vec![9.0, 10.0, 11.0, 12.0]);
    elements.push(vec![13.5, 14.5, 15.5, 16.5]);

    let mut other_elements = Vec::new();
    other_elements.push(vec![1.0, 2.0, 3.0, 4.0]);
    other_elements.push(vec![5.5, 6.5, 7.5, 8.5]);
    other_elements.push(vec![13.5, 14.5, 15.5, 16.5]);
    other_elements.push(vec![9.0, 10.0, 11.0, 12.0]);

    let matrix = Matrix::new(4, 4, Option::Some(elements));
    let other = Matrix::new(4, 4, Option::Some(other_elements));

    assert_ne!(matrix, other);
}

#[test]
fn multiply_test() {
    let mut elements = Vec::new();
    elements.push(vec![1.0, 2.0, 3.0, 4.0]);
    elements.push(vec![5.0, 6.0, 7.0, 8.0]);
    elements.push(vec![9.0, 8.0, 7.0, 6.0]);
    elements.push(vec![5.0, 4.0, 3.0, 2.0]);

    let mut other_elements = Vec::new();
    other_elements.push(vec![-2.0, 1.0, 2.0, 3.0]);
    other_elements.push(vec![3.0, 2.0, 1.0, -1.0]);
    other_elements.push(vec![4.0, 3.0, 6.0, 5.0]);
    other_elements.push(vec![1.0, 2.0, 7.0, 8.0]);

    let matrix = Matrix::new(4, 4, Option::Some(elements));
    let other = Matrix::new(4, 4, Option::Some(other_elements));

    let actual = matrix.multiply(&other);
    let mut expected_elements = Vec::new();
    expected_elements.push(vec![20.0, 22.0, 50.0, 48.0]);
    expected_elements.push(vec![44.0, 54.0, 114.0, 108.0]);
    expected_elements.push(vec![40.0, 58.0, 110.0, 102.0]);
    expected_elements.push(vec![16.0, 26.0, 46.0, 42.0]);

    let expected = Matrix::new(4, 4, Option::Some(expected_elements));

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn multiply_fail_test() {
    let mut elements = Vec::new();
    elements.push(vec![1.0, 2.0, 3.0, 4.0]);
    elements.push(vec![5.0, 6.0, 7.0, 8.0]);
    elements.push(vec![9.0, 8.0, 7.0, 6.0]);
    elements.push(vec![5.0, 4.0, 3.0, 2.0]);

    let mut other_elements = Vec::new();
    other_elements.push(vec![-2.0, 1.0]);
    other_elements.push(vec![3.0, 2.0]);

    let matrix = Matrix::new(4, 4, Option::Some(elements));
    let other = Matrix::new(2, 2, Option::Some(other_elements));

    matrix.multiply(&other);
}

#[test]
fn multiply_tuple_test() {
    let mut elements = Vec::new();
    elements.push(vec![1.0, 2.0, 3.0, 4.0]);
    elements.push(vec![2.0, 4.0, 4.0, 2.0]);
    elements.push(vec![8.0, 6.0, 4.0, 1.0]);
    elements.push(vec![0.0, 0.0, 0.0, 1.0]);

    let matrix = Matrix::new(4, 4, Option::Some(elements));
    let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
    let actual = matrix.multiply_tuple(&tuple);

    assert_eq!(actual, Tuple::new(18.0, 24.0, 33.0, 1.0));
}

#[test]
fn transpose_test() {
    let mut elements = Vec::new();
    elements.push(vec![0.0, 9.0, 3.0, 0.0]);
    elements.push(vec![9.0, 8.0, 0.0, 8.0]);
    elements.push(vec![1.0, 8.0, 5.0, 3.0]);
    elements.push(vec![0.0, 0.0, 5.0, 8.0]);

    let matrix = Matrix::new(4, 4, Option::Some(elements));

    let mut expected_elements = Vec::new();
    expected_elements.push(vec![0.0, 9.0, 1.0, 0.0]);
    expected_elements.push(vec![9.0, 8.0, 8.0, 0.0]);
    expected_elements.push(vec![3.0, 0.0, 5.0, 5.0]);
    expected_elements.push(vec![0.0, 8.0, 3.0, 8.0]);

    let expected = Matrix::new(4, 4, Option::Some(expected_elements));
    let actual = matrix.transpose();

    assert_eq!(expected, actual);
}

#[test]
fn transpose_identity_test() {
    let identity = Matrix::identity(4);
    let actual = identity.transpose();

    assert_eq!(identity, actual);
}

#[test]
fn determinant_test() {
    let elements = vec![
        vec![1.0, 5.0],
        vec![-3.0, 2.0]
    ];

    let matrix = Matrix::new(2, 2, Option::Some(elements));
    let actual = matrix.determinant();

    assert_eq!(17.0, actual);

    let matrix = Matrix::new(3, 3, Option::Some(vec![
        vec![1.0, 2.0, 6.0],
        vec![-5.0, 8.0, -4.0],
        vec![2.0, 6.0, 4.0]
    ]));

    let actual = matrix.determinant();
    assert_eq!(-196.0, actual);

    let matrix = Matrix::new(4, 4, Option::Some(vec![
        vec![-2.0, -8.0, 3.0, 5.0],
        vec![-3.0, 1.0, 7.0, 3.0],
        vec![1.0, 2.0, -9.0, 6.0],
        vec![-6.0, 7.0, 7.0, -9.0]
    ]));

    let actual = matrix.determinant();
    assert_eq!(-4071.0, actual);
}

#[test]
fn submatrix_test() {
    let matrix = Matrix::new(3, 3, Option::Some(vec![
        vec![1.0, 5.0, 0.0],
        vec![-3.0, 2.0, 7.0],
        vec![0.0, 6.0, -3.0]
    ]));
    let actual = matrix.submatrix(0, 2);
    let expected = Matrix::new(2, 2, Option::Some(vec![
        vec![-3.0, 2.0],
        vec![0.0, 6.0]
    ]));

    assert_eq!(actual, expected);

    let matrix = Matrix::new(4, 4, Option::Some(vec![
        vec![-6.0, 1.0, 1.0, 6.0],
        vec![-8.0, 5.0, 8.0, 6.0],
        vec![-1.0, 0.0, 8.0, 2.0],
        vec![-7.0, 1.0, -1.0, 1.0]
    ]));
    let actual = matrix.submatrix(2, 1);
    let expected = Matrix::new(3, 3, Option::Some(vec![
        vec![-6.0, 1.0, 6.0],
        vec![-8.0, 8.0, 6.0],
        vec![-7.0, -1.0, 1.0]
    ]));

    assert_eq!(actual, expected);
}

#[test]
fn minor_test() {
    let matrix = Matrix::new(3, 3, Option::Some(vec![
        vec![3.0, 5.0, 0.0],
        vec![2.0, -1.0, -7.0],
        vec![6.0, -1.0, 5.0]
    ]));

    let actual = matrix.minor(1, 0);
    assert_eq!(actual, 25.0);
}

#[test]
fn cofactor_test() {
    let matrix = Matrix::new(3, 3, Option::Some(vec![
        vec![3.0, 5.0, 0.0],
        vec![2.0, -1.0, -7.0],
        vec![6.0, -1.0, 5.0]
    ]));

    let actual = matrix.cofactor(0, 0);
    assert_eq!(actual, -12.0);

    let actual = matrix.cofactor(1, 0);
    assert_eq!(actual, -25.0);
}

#[test]
fn inverse_test() {
    let matrix = Matrix::new(4, 4, Option::Some(vec![
        vec![-5.0, 2.0, 6.0, -8.0],
        vec![1.0, -5.0, 1.0, 8.0],
        vec![7.0, 7.0, -6.0, -7.0],
        vec![1.0, -3.0, 7.0, 4.0]
    ]));

    let actual = matrix.inverse();
    let expected = Matrix::new(4, 4, Option::Some(vec![
        vec![ 0.21805,  0.45113,  0.24060, -0.04511],
        vec![-0.80827, -1.45677, -0.44361,  0.52068],
        vec![-0.07895, -0.22368, -0.05263,  0.19737],
        vec![-0.52256, -0.81391, -0.30075,  0.30639]
    ]));

    assert_eq!(actual, expected);

    let matrix = Matrix::new(4, 4, Option::Some(vec![
        vec![ 8.0, -5.0,  9.0,  2.0],
        vec![ 7.0,  5.0,  6.0,  1.0],
        vec![-6.0,  0.0,  9.0,  6.0],
        vec![-3.0,  0.0, -9.0, -4.0]
    ]));

    let actual = matrix.inverse();
    let expected = Matrix::new(4, 4, Option::Some(vec![
        vec![-0.15385, -0.15385, -0.28205, -0.53846],
        vec![-0.07692,  0.12308,  0.02564,  0.03077],
        vec![ 0.35897,  0.35897,  0.43590,  0.92308],
        vec![-0.69231, -0.69231, -0.76923, -1.92308],
    ]));

    assert_eq!(actual, expected);

    let matrix = Matrix::new(4, 4, Option::Some(vec![
        vec![ 9.0,  3.0,  0.0,  9.0],
        vec![-5.0, -2.0, -6.0, -3.0],
        vec![-4.0,  9.0,  6.0,  4.0],
        vec![-7.0,  6.0,  6.0,  2.0]
    ]));

    let actual = matrix.inverse();
    let expected = Matrix::new(4, 4, Option::Some(vec![
        vec![-0.04074, -0.07778,  0.14444, -0.22222],
        vec![-0.07778,  0.03333,  0.36667, -0.33333],
        vec![-0.02901, -0.14630, -0.10926,  0.12963],
        vec![ 0.17778,  0.06667, -0.26667,  0.33333]
    ]));

    assert_eq!(actual, expected);
}

#[test]
fn inverse_full_test() {
    let a = Matrix::new(4, 4, Option::Some(vec![
        vec![ 3.0, -9.0,  7.0,  3.0],
        vec![ 3.0, -8.0,  2.0, -9.0],
        vec![-4.0,  4.0,  4.0,  1.0],
        vec![-6.0,  5.0, -1.0,  1.0]
    ]));

    let b = Matrix::new(4, 4, Option::Some(vec![
        vec![8.0,  2.0, 2.0, 2.0],
        vec![3.0, -1.0, 7.0, 0.0],
        vec![7.0,  0.0, 5.0, 4.0],
        vec![6.0, -2.0, 0.0, 5.0]
    ]));

    let c = a.multiply(&b);
    let actual = c.multiply(&b.inverse());

    assert_eq!(a, actual);
}

#[test]
#[should_panic]
fn inverse_panic_test() {
    let matrix = Matrix::new(4, 4, Option::Some(vec![
        vec![-4.0, 2.0, -2.0, -3.0],
        vec![9.0, 6.0, 2.0, 6.0],
        vec![0.0, -5.0, 1.0, -5.0],
        vec![0.0, 0.0, 0.0, 0.0]
    ]));

    matrix.inverse();
}