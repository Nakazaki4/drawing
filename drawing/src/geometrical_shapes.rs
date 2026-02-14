use raster::{Color, Image};
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn random(width: i32, height: i32) -> Self {
        Line {
            start: Point::random(width, height),
            end: Point::random(width, height),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let x0 = self.start.x;
        let y0 = self.start.y;
        let x1 = self.end.x;
        let y1 = self.end.y;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut tx = x0;
        let mut ty = y0;
        let mut idp = 2 * dy - dx;
        let color = self.color();

        if dx > dy {
            // Slope < 1
            loop {
                image.display(tx, ty, color.clone());
                if tx == x1 {
                    break;
                }
                tx += sx;
                if idp > 0 {
                    ty += sy;
                    idp += 2 * (dy - dx);
                } else {
                    idp += 2 * dy;
                }
            }
        } else {
            // Slope >= 1
            idp = 2 * dx - dy;
            loop {
                image.display(tx, ty, color.clone());
                if ty == y1 {
                    break;
                }
                ty += sy;
                if idp > 0 {
                    tx += sx;
                    idp += 2 * (dx - dy);
                } else {
                    idp += 2 * dx;
                }
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(
            rand::random_range(0..=255),
            rand::random_range(0..=255),
            rand::random_range(0..=255),
        )
    }
}

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn random(width: i32, height: i32) -> Self {
        Point {
            x: rand::random_range(0..width),
            y: rand::random_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        let red: u8 = rand::random_range(0..=255);
        let green: u8 = rand::random_range(0..=255);
        let blue: u8 = rand::random_range(0..=255);
        Color::rgb(red, green, blue)
    }
}

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
