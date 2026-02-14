use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Point {
            x: rand::random::<i32>().abs() % width,
            y: rand::random::<i32>().abs() % height,
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(rand::random(), rand::random(), rand::random())
    }
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Line {
            start: *start,
            end: *end,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Line {
            start: Point::random(width, height),
            end: Point::random(width, height),
        }
    }

    fn draw_with_color(&self, image: &mut Image, color: Color) {
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

        if dx > dy {
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
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        self.draw_with_color(image, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(rand::random(), rand::random(), rand::random())
    }
}

pub struct Pentagon {
    center: Point,
    radius: i32,
}

impl Pentagon {
    pub fn random(width: i32, height: i32) -> Self {
        Pentagon {
            center: Point::random(width, height),
            radius: rand::random::<i32>().abs() % 100 + 20,
        }
    }

    fn points(&self) -> [Point; 5] {
        let mut points = [Point::new(0, 0); 5];
        for i in 0..5 {
            let angle = (i as f32) * 2.0 * std::f32::consts::PI / 5.0 - std::f32::consts::PI / 2.0;
            points[i] = Point::new(
                self.center.x + (self.radius as f32 * angle.cos()) as i32,
                self.center.y + (self.radius as f32 * angle.sin()) as i32,
            );
        }
        points
    }
}

impl Drawable for Pentagon {
    fn draw(&self, image: &mut Image) {
        let points = self.points();
        let color = self.color();

        for i in 0..5 {
            let line = Line::new(&points[i], &points[(i + 1) % 5]);
            line.draw_with_color(image, color.clone());
        }
    }

    fn color(&self) -> Color {
        Color::rgb(rand::random(), rand::random(), rand::random())
    }
}
