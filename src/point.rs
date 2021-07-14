use crate::pes::PES;
use std::iter::Sum;
use std::ops::{Add, AddAssign};

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

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}