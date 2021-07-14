use crate::pes::PES;
use crate::pes::Gaussian;
use crate::point::{Chain, Point};

mod point;
mod pes;

fn main() {
    //create mep
    let pes = PES {
        gaussians: vec![
            Gaussian { a: 1.0, x0: 5.0, sig_x: 2.0, y0: 5.0, sig_y: 2.0 },
            Gaussian { a: 1.0, x0: 0.0, sig_x: 2.0, y0: 5.0, sig_y: 2.0 },
            Gaussian { a: 1.0, x0: 5.0, sig_x: 2.0, y0: 0.0, sig_y: 2.0 }
        ]
    };
    //create chain
    let chain = Chain::new(
        Point { x: 7.5, y: 0.0 },
        Point { x: 0.0, y: 7.5 },
        20,
    );
    //iterate
    //  move chain
    //  image
}
