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
    let mut canvas = Canvas::new(10, 20);
    let red = Color::new(1.0, 0.0, 0.0);
    let red_result = red.clone();

    canvas.write_pixel(2, 3, red);

    assert_eq!(true, canvas.pixels[3][2].equals(&red_result));
}