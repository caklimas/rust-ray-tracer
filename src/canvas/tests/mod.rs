use crate::color::Color;
use super::Canvas;

#[test]
fn new_test() {
    let width = 10;
    let height = 20;
    let canvas = Canvas::new(width, height);

    assert_eq!(height, canvas.pixels.len());
    
    for row in &canvas.pixels {
        assert_eq!(width, row.len());
        
        for color in row {
            assert_eq!(true, color.equals(&Color::black()));
        }
    }
}

#[test]
fn write_pixel_test() {
    let x = 2;
    let y = 3;
    let mut canvas = Canvas::new(10, 20);
    let red = Color::new(1.0, 0.0, 0.0);
    let red_result = red.clone();

    canvas.write_pixel(x, y, red);

    assert_eq!(true, canvas.pixels[y][x].equals(&red_result));
}

#[test]
fn to_ppm_test() {
    let mut canvas = Canvas::new(5, 3);
    let c1 = Color::new(1.5, 0.0, 0.0);
    let c2 = Color::new(0.0, 0.5, 0.0);
    let c3 = Color::new(-0.5, 0.0, 1.0);
    canvas.write_pixel(0, 0, c1);
    canvas.write_pixel(2, 1, c2);
    canvas.write_pixel(4, 2, c3);

    let ppm_string = canvas.to_ppm();
    let expected = "P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255";

    assert_eq!(expected, ppm_string.as_str());
}