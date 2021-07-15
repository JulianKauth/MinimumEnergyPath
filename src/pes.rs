use crate::point::Point;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
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

    ///returns the negative gradient of the gauss function at the given point.
    /// negative, because we want the arrows to point downhill
    #[inline]
    fn gradient_at(&self, p: Point) -> Point {
        let dx = self.value_at(p) * (p.x - self.x0) / (2.0 * self.sig_x.powi(2));
        let dy = self.value_at(p) * (p.y - self.y0) / (2.0 * self.sig_y.powi(2));
        Point { x: dx, y: dy }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PES {
    pub(crate) gaussians: Vec<Gaussian>,
}

impl PES {
    pub(crate) fn energy_at(&self, p: Point) -> f64 {
        self.gaussians.iter().map(|g| g.value_at(p)).sum()
    }

    pub fn gradient_at(&self, p: Point) -> Point {
        self.gaussians.iter().map(|g| g.gradient_at(p)).sum()
    }
}