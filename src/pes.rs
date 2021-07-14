use crate::point::Point;
use crate::point::Line;

pub struct Gaussian {
    pub(crate) a: f64,
    pub(crate) x0: f64,
    pub(crate) sig_x: f64,
    pub(crate) y0: f64,
    pub(crate) sig_y: f64,
}

impl Gaussian {
    #[inline]
    fn value_at(&self, p: Point) -> f64 {
        let exponent_x = (p.x - self.x0).powi(2) / (2.0 * self.sig_x.powi(2));
        let exponent_y = (p.y - self.y0).powi(2) / (2.0 * self.sig_y.powi(2));
        self.a * (-(exponent_x + exponent_y)).exp()
    }

    #[inline]
    fn gradient_at(&self, p: Point) -> Line {
        let dx = self.value_at(p) * (-(p.x - self.x0) / (2.0 * self.sig_x.powi(2)));
        let dy = self.value_at(p) * (-(p.y - self.y0) / (2.0 * self.sig_y.powi(2)));
        Line { normal: Point { x: dx, y: dy } }
    }
}

pub struct PES {
    pub(crate) gaussians: Vec<Gaussian>,
}

impl PES {
    pub(crate) fn energy_at(&self, p: Point) -> f64 {
        self.gaussians.iter().map(|g| g.value_at(p)).sum()
    }

    fn gradient_at(&self, p: Point) -> Line{
        self.gaussians.iter().map(|g| g.gradient_at(p)).sum()
    }
}