use crate::point::Point;
use crate::pes::PES;


#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct ChainConfig {
    pub(crate) use_springs: bool,
    pub(crate) pin_ends: bool,
    pub(crate) start: Point,
    pub(crate) end: Point,
    pub(crate) elements: usize,
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
        self.elements.iter().map(|&p| pes.energy_at(p)).sum()
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
                    false,
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
                self.config.use_springs,
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
                false,
            ));
        }

        self.elements = next_instance;
    }
}