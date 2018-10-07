#[derive(Debug)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug)]
pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * (self.radius * self.radius)
    }

    fn perimeter(&self) -> f64 {
        2.0 * 3.14159 * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // Heron's Formula
        let p = (self.a + self.b + self.c) / 2.0;
        (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}
