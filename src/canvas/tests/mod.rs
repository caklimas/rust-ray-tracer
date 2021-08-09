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