use std::iter::Sum;
use std::ops::{Add, AddAssign, Sub, Mul, Div};
use std::f64::consts::FRAC_PI_2;


#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
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

    pub fn dot_product(&self, other: Point) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn move_perpendicular_to(&self, prev: Point, next: Point, gradient: Point, spring_effect: f64) -> Self {
        let tangent = (prev - next).rotate(FRAC_PI_2).normed();

        // get the step distance for this vector
        // see https://www.youtube.com/watch?v=ePIwYHF2O4s
        let alpha = tangent.dot_product(gradient);
        let gradient_forces = alpha * tangent;

        let a = prev - *self; //vector from this point to the previous one
        let b = next - *self; //vector from this point to the next one
        let spring_forces = spring_effect * (a + b) / 2; //how much we need to move to land in the middle of the two other points

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