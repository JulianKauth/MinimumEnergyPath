use crate::point::Point;
use crate::pes::PES;


#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct ChainConfig {
    pub(crate) spring_constant: f64,
    pub(crate) pin_ends: bool,
    pub(crate) relax_ends: bool,
    pub(crate) start: Point,
    pub(crate) end: Point,
    pub(crate) elements: usize,
}

impl ChainConfig {
    pub fn relax_ends(&mut self, pes: &PES, convergence_limit: f64) {
        if self.relax_ends {
            self.start.move_to_minimum(pes, convergence_limit);
            self.end.move_to_minimum(pes, convergence_limit);
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chain {
    pub(crate) config: ChainConfig,
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
        Chain {
            config,
            elements,
        }
    }

    pub fn energy(&self, pes: &PES) -> f64 {
        let total_energy: f64 = self.elements.iter().map(|&p| pes.energy_at(p)).sum();
        total_energy / self.config.elements as f64
    }

    pub fn iterate(&mut self, pes: &PES) {
        let size = self.elements.len();
        let mut next_instance = Vec::with_capacity(size);

        // start point
        if self.config.pin_ends {
            next_instance.push(*self.elements.get(0).unwrap());
        } else {
            let this = *self.elements.get(0).unwrap();
            let next = *self.elements.get(1).unwrap();
            next_instance.push(
                this.move_perpendicular_to(
                    this,
                    next,
                    pes.gradient_at(this),
                    0.0,
                )
            );
        }

        // all the points that have two neighbors. Start and end need to be treated separately
        for i in 1..size - 1 {
            let prev = *self.elements.get(i - 1).unwrap();
            let this = *self.elements.get(i).unwrap();
            let next = *self.elements.get(i + 1).unwrap();
            next_instance.push(this.move_perpendicular_to(
                prev,
                next,
                pes.gradient_at(this),
                self.config.spring_constant,
            ));
        }

        // end point
        if self.config.pin_ends {
            next_instance.push(*self.elements.get(size - 1).unwrap());
        } else {
            let prev = *self.elements.get(size - 2).unwrap();
            let this = *self.elements.get(size - 1).unwrap();
            next_instance.push(this.move_perpendicular_to(
                prev,
                this,
                pes.gradient_at(this),
                0.0,
            ));
        }

        self.elements = next_instance;
    }
}