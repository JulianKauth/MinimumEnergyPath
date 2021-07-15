#[macro_use]
extern crate serde_derive;

use crate::pes::PES;
use crate::pes::Gaussian;
use crate::image::{Image, ImageConfig};
use crate::chain::{Chain, ChainConfig};
use crate::point::Point;
use std::time::SystemTime;
use std::fs;
use std::process::exit;

mod point;
mod pes;
mod image;
mod chain;

//todo: make help text

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    convergence_limit: f64,
    pes: PES,
    path: ChainConfig,
    image: ImageConfig,
}

fn main() {
    // keep track of how long everything takes
    let mut start_time = SystemTime::now();

    // read the config
    let config = load_config();
    ensure_image_directory();

    //create mep
    let pes = config.pes;
    //create chain
    let mut chain = Chain::new(config.path);

    //set up our image generator
    let img = Image::new(config.image, &pes);

    print!("Setup took: ");
    print_elapsed_time(&mut start_time);

    // iterate until we reached a stable state
    let mut counter = 0;
    let mut last_energy;
    let mut energy = chain.energy(&pes);
    println!("starting with initial energy: {}", energy);
    loop {
        // save the state
        img.paint(&*format!("images/progress_{:04}.png", counter), &chain, &pes);

        //move to a better position
        chain.iterate(&pes);

        // increment counter and update energy values
        counter += 1;
        last_energy = energy;
        energy = chain.energy(&pes);

        // print info
        print!("iteration: {:4} resulted in energy: {:15.10} and took: ", counter, energy);
        print_elapsed_time(&mut start_time);

        // stop the loop if the last iteration was barely able to improve the situation
        if last_energy - energy < config.convergence_limit {
            break;
        }
    }
}

fn print_elapsed_time(time_instance: &mut SystemTime) {
    match time_instance.elapsed() {
        Ok(elapsed) => { println!("{}.{:03} sec", elapsed.as_secs(), elapsed.subsec_millis()); }
        Err(_) => { println!("<error>"); }
    }
    *time_instance = SystemTime::now();
}

fn ensure_image_directory() {
    //ensure the image directory exists
    if let Err(err) = std::fs::create_dir_all(std::path::Path::new("images/")) {
        println!("Could not create the directory for the resulting images!");
        println!("Error: {}", err);
        exit(4);
    }
}

fn load_config() -> Config {
    match fs::read_to_string("MEP_config.txt") {
        Ok(data) => {
            println!("Read config file 'MEP_config.txt'!");
            match serde_json::de::from_str(&*data) {
                Ok(json) => {
                    println!("Successfully parsed the config file!");
                    println!("Simulating...");
                    return json;
                }
                Err(err) => {
                    println!("Failed to parse JSON from the config file!");
                    println!("Error: {}", err);
                    println!("The file has to conform to the following structure:");
                    println!("{}", sample_config_text());
                    exit(1);
                }
            }
        }
        Err(err) => {
            println!("Could not read the config file 'MEP_config.txt'!");
            println!("Error: {}", err);

            // try to create an example config file
            match create_sample_config_file() {
                Ok(_) => {
                    println!("Created an example config file 'MEP_config.txt'!");
                    exit(2);
                }
                Err(err) => {
                    println!("Failed to create an example config file called 'MEP_config.txt'!");
                    println!("Error: {}", err);
                    println!("Please create the file manually. It has to conform to the following structure:");
                    println!("{}", sample_config_text());
                    exit(3);
                }
            }
        }
    }
}

fn create_sample_config_file() -> std::io::Result<()> {
    fs::write("MEP_config.txt", sample_config_text())
}

fn sample_config_text() -> String {
    serde_json::ser::to_string_pretty(&sample_config()).unwrap()
}

fn sample_config() -> Config {
    let stable_limit = 1e-4;

    let pes = PES {
        gaussians: vec![
            Gaussian { a: -5.0, x0: 5.0, sig_x: 2.0, y0: 5.0, sig_y: 2.0 },
            Gaussian { a: -5.0, x0: 0.0, sig_x: 2.0, y0: 5.0, sig_y: 2.0 },
            Gaussian { a: -5.0, x0: 5.0, sig_x: 2.0, y0: 0.0, sig_y: 2.0 }
        ]
    };
    let chain_config = ChainConfig {
        use_springs: false,
        pin_ends: false,
        start: Point { x: 7.5, y: 0.0 },
        end: Point { x: 0.0, y: 7.5 },
        elements: 20,
    };

    let image_config = ImageConfig {
        x0: -2.0,
        y0: -2.0,
        width: 10.0,
        height: 10.0,
        resolution_x: 2000,
        resolution_y: 2000,
        point_size: 0.1,
        line_width: 0.05,
    };

    Config {
        convergence_limit: stable_limit,
        pes: pes.clone(),
        path: chain_config,
        image: image_config,
    }
}