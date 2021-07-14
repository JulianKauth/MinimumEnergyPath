use crate::pes::PES;
use crate::pes::Gaussian;
use crate::image::Image;
use crate::chain::Chain;
use crate::point::Point;
use std::time::SystemTime;

mod point;
mod pes;
mod image;
mod chain;

fn main() {
    // Important config at the top: convergence limit, PES, Chain, Image Config
    let stable_limit = 1e-6;

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

    // keep track of how long everything takes
    let mut start_time = SystemTime::now();

    //set up our image generator
    print!("Image setup took: ");
    let img = Image::new(-2.0, 9.0, -2.0, 9.0, 2000, 2000, &pes);
    print_elapsed_time(&mut start_time);
    println!();

    // iterate until we reached a stable state
    let mut counter = 0;
    let mut last_energy;
    let mut energy = chain.energy(&pes);
    println!("starting with initial energy: {}", energy);
    loop {
        // save the state
        img.paint(&*format!("../images/progress_{:04}.png", counter), &chain, &pes);

        //move to a better position
        chain.iterate(&pes);

        // increment counter and update energy values
        counter += 1;
        last_energy = energy;
        energy = chain.energy(&pes);

        // print info
        print!("iteration: {:.6} resulted in energy: {} and took: ", energy, energy);
        print_elapsed_time(&mut start_time);
        println!();

        // stop the loop if the last iteration was barely able to improve the situation
        if last_energy - energy < stable_limit {
            break;
        }
    }
}

fn print_elapsed_time(time_instance: &mut SystemTime) {
    match time_instance.elapsed() {
        Ok(elapsed) => { print!("{}.{:03} sec", elapsed.as_secs(), elapsed.subsec_millis()); }
        Err(_) => { print!("<error>"); }
    }
    *time_instance = SystemTime::now();
}