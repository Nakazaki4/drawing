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
        // let c = self.color();
        // for i in 0..image.width {
        //     image.display(i, 4, c.clone());
        // }
    }
    fn color(&self) -> Color {
        let red: u8 = rand::random_range(0..=255);
        let green: u8 = rand::random_range(0..=255);
        let blue: u8 = rand::random_range(0..=255);
        Color::rgb(red, green, blue)
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
