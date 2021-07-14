use crate::point::Point;
use crate::pes::PES;

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