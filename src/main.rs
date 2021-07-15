#[macro_use]
extern crate serde_derive;

use crate::pes::PES;
use crate::pes::Gaussian;
use crate::image::{Image, ImageConfig};
use crate::chain::{Chain, ChainConfig};
use crate::point::Point;
use std::time::SystemTime;
use std::io::Error;
use std::{io, fs};
use std::process::exit;

mod point;
mod pes;
mod image;
mod chain;

//todo: implement the method with springs and make that an option in the json
//todo: make help text

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    convergence_limit: f64,
    use_springs: bool,
    pes: PES,
    path: ChainConfig,
    image: ImageConfig,
}

fn main() {
    // read the config
    // match load_config() {
    //     Ok(data) => {}
    //     Err(err) => {
    //         println!("Could not the config file 'MEP_config.txt'!");
    //         println!("Error: {}", err);
    //         exit(1);
    //     }
    // }


    // Important config at the top: convergence limit, PES, Chain, Image Config
    let stable_limit = 1e-6;
    let use_springs = false;

    //create mep
    let pes = PES {
        gaussians: vec![
            Gaussian { a: -5.0, x0: 5.0, sig_x: 2.0, y0: 5.0, sig_y: 2.0 },
            Gaussian { a: -5.0, x0: 0.0, sig_x: 2.0, y0: 5.0, sig_y: 2.0 },
            Gaussian { a: -5.0, x0: 5.0, sig_x: 2.0, y0: 0.0, sig_y: 2.0 }
        ]
    };
    //create chain
    let chain_config = ChainConfig {
        start: Point { x: 7.5, y: 0.0 },
        end: Point { x: 0.0, y: 7.5 },
        elements: 20,
    };
    let mut chain = Chain::new(chain_config);

    // keep track of how long everything takes
    let mut start_time = SystemTime::now();

    //set up our image generator
    print!("Image setup took: ");
    let image_config = ImageConfig {
        x0: -2.0,
        y0: -2.0,
        width: 10.0,
        height: 10.0,
        resolution_x: 2000,
        resolution_y: 2000,
    };
    let img = Image::new(image_config, &pes);
    print_elapsed_time(&mut start_time);
    println!();


    //todo for testing only
    let cfg: Config = Config {
        convergence_limit: stable_limit,
        use_springs,
        pes: pes.clone(),
        path: chain_config,
        image: image_config,
    };
    println!("{}", serde_json::to_string_pretty(&cfg).unwrap());


    //ensure the image directory exists
    if let Err(err) = std::fs::create_dir_all(std::path::Path::new("images/")) {
        println!("Could not create the directory for the resulting images!");
        println!("Error: {}", err);
        exit(2);
    }

    // iterate until we reached a stable state
    let mut counter = 0;
    let mut last_energy;
    let mut energy = chain.energy(&pes);
    println!("starting with initial energy: {}", energy);
    loop {
        // save the state
        img.paint(&*format!("images/progress_{:04}.png", counter), &chain, &pes);

        //move to a better position
        chain.iterate(&pes, use_springs);

        // increment counter and update energy values
        counter += 1;
        last_energy = energy;
        energy = chain.energy(&pes);

        // print info
        print!("iteration: {:4} resulted in energy: {:15.10} and took: ", counter, energy);
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

fn load_config() -> io::Result<String> {
    //TODO:
    // think about a json format
    // read a file
    // read the json into usable datatypes
    // if the file doesn't exist, create an example config
    // if the example config can't be created, fail

    let data = fs::read_to_string("MEP_config.txt")?;
    Ok(data)
}