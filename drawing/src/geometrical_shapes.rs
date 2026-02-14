use raster::{Color, Image};

// ==============================================================================

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// ==============================================================================

#[derive(Debug, PartialEq, Clone, Copy)]
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
// ==============================================================================

#[derive(Debug, PartialEq)]
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
// ==============================================================================

#[derive(Debug, PartialEq)]
pub struct Pentagon {
    pub center: Point,
    pub radius: i32,
}

impl Pentagon {
    pub fn new(center: Point, radius: i32) -> Self {
        Pentagon { center, radius }
    }

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

// ==============================================================================
#[derive(Debug, PartialEq)]
pub struct Triangle {
    pub pnt1: Point,
    pub pnt2: Point,
    pub pnt3: Point,
}

impl Triangle {
    pub fn new(pnt1: &Point, pnt2: &Point, pnt3: &Point) -> Self {
        Triangle {
            pnt1: Point::new(pnt1.x, pnt1.y),
            pnt2: Point::new(pnt2.x, pnt2.y),
            pnt3: Point::new(pnt3.x, pnt3.y),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Triangle {
            pnt1: Point::random(width, height),
            pnt2: Point::random(width, height),
            pnt3: Point::random(width, height),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let g = self.color();
        Line {
            start: self.pnt1.clone(),
            end: self.pnt2.clone(),
        }
        .draw_with_color(image, g.clone());
        Line {
            start: self.pnt2.clone(),
            end: self.pnt3.clone(),
        }
        .draw_with_color(image, g.clone());
        Line {
            start: self.pnt3.clone(),
            end: self.pnt1.clone(),
        }
        .draw_with_color(image, g.clone());
    }

    fn color(&self) -> Color {
        Color::rgb(rand::random(), rand::random(), rand::random())
    }
}

// ==============================================================================

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub pnt1: Point,
    pub pnt2: Point,
}

impl Rectangle {
    pub fn new(pnt1: &Point, pnt2: &Point) -> Self {
        Rectangle {
            pnt1: *pnt1,
            pnt2: *pnt2,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Rectangle {
            pnt1: Point::random(width, height),
            pnt2: Point::random(width, height),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();

        let pnt3 = Point::new(self.pnt2.x, self.pnt1.y);
        let pnt4 = Point::new(self.pnt1.x, self.pnt2.y);

        Line::new(&self.pnt1, &pnt3).draw_with_color(image, color.clone());
        Line::new(&pnt3, &self.pnt2).draw_with_color(image, color.clone());
        Line::new(&self.pnt2, &pnt4).draw_with_color(image, color.clone());
        Line::new(&pnt4, &self.pnt1).draw_with_color(image, color.clone());
    }

    fn color(&self) -> Color {
        Color::rgb(
            rand::random_range(0..=255),
            rand::random_range(0..=255),
            rand::random_range(0..=255),
        )
    }
}

// ==============================================================================

#[derive(Debug, PartialEq)]
pub struct Cube {
    pub origin: Point,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
}

impl Cube {
    pub fn new(origin: &Point, width: i32, height: i32, depth: i32) -> Self {
        Cube {
            origin: *origin,
            width,
            height,
            depth,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Cube {
            origin: Point::random(width, height),
            width: rand::random::<i32>().abs() % 100 + 20,
            height: rand::random::<i32>().abs() % 100 + 20,
            depth: rand::random::<i32>().abs() % 100 + 20,
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, image: &mut Image) {
        let color = self.color();

        // Front face points
        let f1 = self.origin;
        let f2 = Point::new(f1.x + self.width, f1.y);
        let f3 = Point::new(f1.x + self.width, f1.y + self.height);
        let f4 = Point::new(f1.x, f1.y + self.height);

        // Back face points (shifted)
        let b1 = Point::new(f1.x + self.depth, f1.y - self.depth);
        let b2 = Point::new(f2.x + self.depth, f2.y - self.depth);
        let b3 = Point::new(f3.x + self.depth, f3.y - self.depth);
        let b4 = Point::new(f4.x + self.depth, f4.y - self.depth);

        // Front face
        Line::new(&f1, &f2).draw_with_color(image, color.clone());
        Line::new(&f2, &f3).draw_with_color(image, color.clone());
        Line::new(&f3, &f4).draw_with_color(image, color.clone());
        Line::new(&f4, &f1).draw_with_color(image, color.clone());

        // Back face
        Line::new(&b1, &b2).draw_with_color(image, color.clone());
        Line::new(&b2, &b3).draw_with_color(image, color.clone());
        Line::new(&b3, &b4).draw_with_color(image, color.clone());
        Line::new(&b4, &b1).draw_with_color(image, color.clone());

        // Connections
        Line::new(&f1, &b1).draw_with_color(image, color.clone());
        Line::new(&f2, &b2).draw_with_color(image, color.clone());
        Line::new(&f3, &b3).draw_with_color(image, color.clone());
        Line::new(&f4, &b4).draw_with_color(image, color.clone());
    }

    fn color(&self) -> Color {
        Color::rgb(
            rand::random_range(0..=255),
            rand::random_range(0..=255),
            rand::random_range(0..=255),
        )
    }
}

// ==============================================================================

#[derive(Debug, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Circle {
            center: *center,
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Circle {
            center: Point::random(width, height),
            radius: rand::random::<i32>().abs() % 100,
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let x0 = self.center.x;
        let y0 = self.center.y;
        let radius = self.radius;

        let mut x = radius;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            image.display(x0 + x, y0 + y, color.clone());
            image.display(x0 + y, y0 + x, color.clone());
            image.display(x0 - y, y0 + x, color.clone());
            image.display(x0 - x, y0 + y, color.clone());
            image.display(x0 - x, y0 - y, color.clone());
            image.display(x0 - y, y0 - x, color.clone());
            image.display(x0 + y, y0 - x, color.clone());
            image.display(x0 + x, y0 - y, color.clone());

            if err <= 0 {
                y += 1;
                err += 2 * y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(rand::random(), rand::random(), rand::random())
    }
}

// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_new() {
        let p = Point::new(10, 20);
        assert_eq!(p, Point { x: 10, y: 20 });
    }

    #[test]
    fn test_point_random() {
        let p = Point::random(100, 200);
        assert!(p.x < 100);
        assert!(p.y < 200);
    }

    #[test]
    fn test_line_new() {
        let p1 = Point::new(10, 20);
        let p2 = Point::new(30, 40);
        let l = Line::new(&p1, &p2);
        assert_eq!(l, Line { start: p1, end: p2 });
    }

    #[test]
    fn test_line_random() {
        let l = Line::random(100, 200);
        assert!(l.start.x < 100);
        assert!(l.start.y < 200);
        assert!(l.end.x < 100);
        assert!(l.end.y < 200);
    }

    #[test]
    fn test_triangle_new() {
        let p1 = Point::new(10, 20);
        let p2 = Point::new(30, 40);
        let p3 = Point::new(50, 60);
        let t = Triangle::new(&p1, &p2, &p3);
        assert_eq!(t, Triangle { pnt1: p1, pnt2: p2, pnt3: p3 });
    }

    #[test]
    fn test_triangle_random() {
        let t = Triangle::random(100, 200);
        assert!(t.pnt1.x < 100);
        assert!(t.pnt1.y < 200);
        assert!(t.pnt2.x < 100);
        assert!(t.pnt2.y < 200);
        assert!(t.pnt3.x < 100);
        assert!(t.pnt3.y < 200);
    }

    #[test]
    fn test_rectangle_new() {
        let p1 = Point::new(10, 20);
        let p2 = Point::new(30, 40);
        let r = Rectangle::new(&p1, &p2);
        assert_eq!(r, Rectangle { pnt1: p1, pnt2: p2 });
    }

    #[test]
    fn test_rectangle_random() {
        let r = Rectangle::random(100, 200);
        assert!(r.pnt1.x < 100);
        assert!(r.pnt1.y < 200);
        assert!(r.pnt2.x < 100);
        assert!(r.pnt2.y < 200);
    }

    #[test]
    fn test_cube_new() {
        let p = Point::new(10, 20);
        let c = Cube::new(&p, 100, 200, 50);
        assert_eq!(c, Cube { origin: p, width: 100, height: 200, depth: 50 });
    }

    #[test]
    fn test_cube_random() {
        let c = Cube::random(100, 200);
        assert!(c.origin.x < 100);
        assert!(c.origin.y < 200);
        assert!(c.width >= 20 && c.width < 120);
        assert!(c.height >= 20 && c.height < 120);
        assert!(c.depth >= 20 && c.depth < 120);
    }

    #[test]
    fn test_circle_new() {
        let p = Point::new(10, 20);
        let c = Circle::new(&p, 100);
        assert_eq!(c, Circle { center: p, radius: 100 });
    }

    #[test]
    fn test_circle_random() {
        let c = Circle::random(100, 200);
        assert!(c.center.x < 100);
        assert!(c.center.y < 200);
        assert!(c.radius < 100);
    }

    #[test]
    fn test_pentagon_new() {
        let p = Point::new(10, 20);
        let pentagon = Pentagon::new(p, 100);
        assert_eq!(pentagon, Pentagon { center: p, radius: 100 });
    }

    #[test]
    fn test_pentagon_random() {
        let p = Pentagon::random(100, 200);
        assert!(p.center.x < 100);
        assert!(p.center.y < 200);
        assert!(p.radius >= 20 && p.radius < 120);
    }

    #[test]
    fn test_pentagon_points() {
        let p = Point::new(100, 100);
        let pentagon = Pentagon::new(p, 50);
        let points = pentagon.points();
        assert_eq!(points.len(), 5);

        let expected_points = [
            Point::new(100, 50),
            Point::new(147, 84),
            Point::new(129, 140),
            Point::new(70, 140),
            Point::new(52, 84),
        ];

        for i in 0..5 {
            assert!(
                (points[i].x - expected_points[i].x).abs() <= 1,
                "x diff for point {}",
                i
            );
            assert!(
                (points[i].y - expected_points[i].y).abs() <= 1,
                "y diff for point {}",
                i
            );
        }
    }
}
