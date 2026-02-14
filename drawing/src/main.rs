mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

//** Point
//** Line
//** Triangle
//** Rectangle
//** Circle
//** Pentagon
//** Cube

fn main() {
    let mut image = Image::blank(1000, 1000);
    let (width, height) = (image.width, image.height);

    // --- Test Point ---
    gs::Point::new(10, 10).draw(&mut image);

    // --- Test Line ---
    gs::Line::new(&gs::Point::new(20, 20), &gs::Point::new(100, 20)).draw(&mut image);

    // --- Test Triangle ---
    gs::Triangle::new(
        &gs::Point::new(50, 150),
        &gs::Point::new(10, 200),
        &gs::Point::new(90, 200),
    ).draw(&mut image);

    // --- Test Rectangle ---
    gs::Rectangle::new(&gs::Point::new(110, 150), &gs::Point::new(200, 200)).draw(&mut image);

    // --- Test Pentagon ---
    gs::Pentagon::new(gs::Point::new(300, 200), 50).draw(&mut image);

    // --- Test Circle ---
    gs::Circle::new(&gs::Point::new(500, 200), 40).draw(&mut image);
    
    // --- Test Cube ---
    gs::Cube::new(&gs::Point::new(600, 200), 50, 60, 25).draw(&mut image);

    // --- Draw random shapes ---
    for _ in 0..30 {
        gs::Point::random(width, height).draw(&mut image);
        gs::Line::random(width, height).draw(&mut image);
        gs::Triangle::random(width, height).draw(&mut image);
        gs::Rectangle::random(width, height).draw(&mut image);
        gs::Pentagon::random(width, height).draw(&mut image);
        gs::Circle::random(width, height).draw(&mut image);
        gs::Cube::random(width, height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
