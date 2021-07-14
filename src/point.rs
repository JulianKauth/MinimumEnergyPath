use std::iter::Sum;
use std::ops::{Add, AddAssign, Sub, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub(crate) normal: Point,
}

impl Sum for Line {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        let mut re = Point { x: 0.0, y: 0.0 };
        for line in iter {
            re += line.normal;
        }
        Line { normal: re }
    }
}


#[derive(Debug, Copy, Clone, )]
pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Point {
    pub fn distance_sq(&self, other: Point) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    pub fn rotate(&self, rad: f64) -> Self {
        Point {
            x: self.x * rad.cos() - self.y * rad.sin(),
            y: self.x * rad.sin() + self.y * rad.cos(),
        }
    }

    pub fn normed(&self) -> Self {
        let scale = (self.x.powi(2) + self.y.powi(2)).sqrt();
        Point {
            x: self.x / scale,
            y: self.y / scale,
        }
    }

    pub fn dot_product(&self, other: Point) -> f64{
        self.x * other.x + self.y * other.y
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Point> for f64{
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point{
            x: rhs.x * self,
            y: rhs.y * self
        }
    }
}