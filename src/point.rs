use std::iter::Sum;
use std::ops::{Add, AddAssign, Sub, Mul, Div};
use std::f64::consts::FRAC_PI_2;
use crate::pes::PES;


#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Point {
    pub fn move_to_minimum(&mut self, pes: &PES, convergence_limit: f64) {
        let mut last_energy: f64;
        let mut energy = pes.energy_at(*self);
        loop {
            // move point towards minimum
            *self += pes.gradient_at(*self);

            // update energy values
            last_energy = energy;
            energy = pes.energy_at(*self);

            // stop iterating if we can't change anything anymore
            if last_energy - energy < convergence_limit {
                break;
            }
        }
    }

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
        let x = if f64::is_nan(self.x / scale) { 0.0 } else { self.x / scale };
        let y = if f64::is_nan(self.y / scale) { 0.0 } else { self.y / scale };
        Point { x, y }
    }

    pub fn dot_product(&self, other: Point) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn move_perpendicular_to(&self, prev: Point, next: Point, gradient: Point, spring_effect: f64) -> Self {
        let tangent = (prev - next).normed();
        let normal = tangent.rotate(FRAC_PI_2);

        // get the step distance for this vector
        // see https://www.youtube.com/watch?v=ePIwYHF2O4s
        let alpha = normal.dot_product(gradient);
        let gradient_forces = alpha * normal;

        let a = prev - *self; //vector from this point to the previous one
        let b = next - *self; //vector from this point to the next one
        let spring = spring_effect * (a + b) / 2; //how much we need to move to land in the middle of the two other points
        let spring_forces = tangent.dot_product(spring) * tangent;
        //let spring_forces = spring;

        *self + gradient_forces + spring_forces
    }
}

impl Sum for Point {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        let mut re = Point { x: 0.0, y: 0.0 };
        for point in iter {
            re += point;
        }
        re
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

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl Div<u32> for Point {
    type Output = Point;

    fn div(self, i: u32) -> Self::Output {
        Point {
            x: self.x / i as f64,
            y: self.y / i as f64,
        }
    }
}