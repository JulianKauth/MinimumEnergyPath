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


#[derive(Debug, Clone)]
pub struct Chain {
    pub(crate) elements: Vec<Point>,
}

impl Chain {
    pub(crate) fn new(start: Point, end: Point, num: usize) -> Self {
        let x = start.x;
        let y = start.y;
        let dx = (end.x - start.x) / num as f64;
        let dy = (end.y - start.y) / num as f64;
        let mut elements = Vec::with_capacity(num);
        for i in 0..=num {
            elements.push(Point { x: x + dx * i as f64, y: y + dy * i as f64 })
        }
        Chain { elements }
    }

    pub fn energy(&self, pes: PES) -> f64 {
        self.elements.iter().map(|&p| pes.energy_at(p)).sum()
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