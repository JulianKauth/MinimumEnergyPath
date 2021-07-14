use crate::pes::PES;
use crate::pes::Gaussian;
use crate::image::Image;
use crate::chain::Chain;
use crate::point::Point;
use std::f64::consts::FRAC_PI_2;

mod point;
mod pes;
mod image;
mod chain;

fn main() {
    //create mep
    let pes = PES {
        gaussians: vec![
            Gaussian { a: -5.0, x0: 5.0, sig_x: 2.0, y0: 5.0, sig_y: 2.0 },
            Gaussian { a: -5.0, x0: 0.0, sig_x: 2.0, y0: 5.0, sig_y: 2.0 },
            Gaussian { a: -5.0, x0: 5.0, sig_x: 2.0, y0: 0.0, sig_y: 2.0 }
        ]
    };
    //create chain
    let mut chain = Chain::new(
        Point { x: 7.5, y: 0.0 },
        Point { x: 0.0, y: 7.5 },
        20,
    );
    //set up our image generator
    let img = Image::new(-2.0, 9.0, -2.0, 9.0, 2000, 2000);

    // iterate until we reached a stable state
    let stable_limit = 1e-6;
    let mut counter = 0;
    let mut last_energy;
    let mut energy = chain.energy(&pes);
    loop {
        // save the state
        img.paint(&*format!("../images/progress_{:04}.png", counter), &chain, &pes);

        //move to a better position
        chain.iterate(&pes);

        // increment counter and update energy values
        counter += 1;
        last_energy = energy;
        energy = chain.energy(&pes);
        println!("{}", energy);

        // stop the loop if the last iteration was barely able to improve the situation
        if last_energy - energy < stable_limit{
            break;
        }
    }
}
