use crate::point::Point;
use crate::pes::PES;
use std::f64::consts::FRAC_PI_2;


#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct ChainConfig {
    pub(crate) start: Point,
    pub(crate) end: Point,
    pub(crate) elements: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chain {
    pub(crate) elements: Vec<Point>,
}

impl Chain {
    pub(crate) fn new(config: ChainConfig) -> Self {
        let x = config.start.x;
        let y = config.start.y;
        let dx = (config.end.x - config.start.x) / config.elements as f64;
        let dy = (config.end.y - config.start.y) / config.elements as f64;
        let mut elements = Vec::with_capacity(config.elements);
        for i in 0..=config.elements {
            elements.push(Point { x: x + dx * i as f64, y: y + dy * i as f64 })
        }
        Chain { elements }
    }

    pub fn energy(&self, pes: &PES) -> f64 {
        self.elements.iter().map(|&p| pes.energy_at(p)).sum()
    }

    //todo: implement the method with springs as well (make it an option to iterate)

    pub fn iterate(&mut self, pes: &PES, use_springs: bool) {
        let size = self.elements.len();

        let mut next_instance = Vec::with_capacity(size);
        next_instance.push(*self.elements.get(0).unwrap());
        // all the points that have two neighbors. Start and end don't move
        for i in 1..size - 1 {
            let prev = *self.elements.get(i - 1).unwrap();
            let this = *self.elements.get(i).unwrap();
            let next = *self.elements.get(i + 1).unwrap();

            let tangent = (prev - next).rotate(FRAC_PI_2).normed();
            let gradient = pes.gradient_at(this);

            // get the step distance for this vector
            // see https://www.youtube.com/watch?v=ePIwYHF2O4s
            let alpha = tangent.dot_product(gradient);
            next_instance.push(this + alpha * tangent);
        }
        next_instance.push(*self.elements.get(size - 1).unwrap());

        self.elements = next_instance;
    }
}