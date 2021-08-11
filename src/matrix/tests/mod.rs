use crate::floating_point::FloatingPoint;
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

    assert_eq!(true, matrix.equals(&other));
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

    assert_eq!(false, matrix.equals(&other));
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

    assert_eq!(true, actual.equals(&expected));
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