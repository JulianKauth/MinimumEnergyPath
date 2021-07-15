#[macro_use]
extern crate serde_derive;

use crate::pes::PES;
use crate::pes::Gaussian;
use crate::image::{Image, ImageConfig};
use crate::chain::{Chain, ChainConfig};
use crate::point::Point;
use std::time::SystemTime;
use std::{fs, env};
use std::process::exit;

mod point;
mod pes;
mod image;
mod chain;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    convergence_limit: f64,
    pes: PES,
    path: ChainConfig,
    image: ImageConfig,
}

fn main() {
    // argument parsing first, in case we need to catch a --help and exit early
    arg_parse();

    // keep track of how long everything takes
    let mut start_time = SystemTime::now();

    // read the config
    let config = load_config();
    ensure_image_directory();

    //create mep
    let pes = config.pes;
    //create chain
    let mut chain_config = config.path;
    chain_config.relax_ends(&pes, config.convergence_limit);
    let mut chain = Chain::new(chain_config);

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
        print!("iteration: {:4} resulted in average energy: {:15.10} and took: ", counter, energy);
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
        exit(3);
    }
}

fn load_config() -> Config {
    match fs::read_to_string("MEP_config.txt") {
        Ok(data) => {
            println!("Read config file 'MEP_config.txt'.");
            match serde_json::de::from_str(&*data) {
                Ok(json) => {
                    println!("Successfully parsed the config file.");
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
                    println!("Simulating the example...");
                    sample_config()
                }
                Err(err) => {
                    println!("Failed to create an example config file called 'MEP_config.txt'!");
                    println!("Error: {}", err);
                    println!("Please create the file manually. It has to conform to the following structure:");
                    println!("{}", sample_config_text());
                    exit(2);
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
        scale: 0.7,
        gaussians: vec![
            Gaussian { a: -7.0, x0: 15.0, sig_x: 5.0, y0: 7.5, sig_y: 5.0 },
            Gaussian { a: -13.0, x0: 23.0, sig_x: 2.0, y0: 5.0, sig_y: 2.0 },
            Gaussian { a: -7.0, x0: 10.0, sig_x: 2.5, y0: 21.0, sig_y: 2.5 },
            Gaussian { a: -7.0, x0: 7.0, sig_x: 2.5, y0: 18.0, sig_y: 2.5 },
            Gaussian { a: 15.0, x0: 25.0, sig_x: 7.0, y0: 20.0, sig_y: 7.0 },
            Gaussian { a: 20.0, x0: 20.0, sig_x: 10.0, y0: -5.0, sig_y: 1.0 },
            Gaussian { a: 20.0, x0: -2.0, sig_x: 1.5, y0: 20.0, sig_y: 10.0 },
            Gaussian { a: 10.0, x0: 0.0, sig_x: 3.0, y0: 3.0, sig_y: 3.0 }
        ],
    };
    let chain_config = ChainConfig {
        spring_constant: 0.0,
        pin_ends: true,
        relax_ends: true,
        start: Point { x: 23.0, y: 5.0 },
        end: Point { x: 8.0, y: 19.0 },
        elements: 50,
    };

    let image_config = ImageConfig {
        contour_lines: 10.0,
        x0: 0.0,
        y0: 0.0,
        width: 27.0,
        height: 25.0,
        resolution_x: 2700,
        resolution_y: 2500,
        point_size: 0.1,
        line_width: 0.02,
    };

    Config {
        convergence_limit: stable_limit,
        pes: pes.clone(),
        path: chain_config,
        image: image_config,
    }
}

fn arg_parse() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {  // <=1, because the first one is always the program name
        return;
    }
    // we got an argument
    match &args[1][..] {
        "--explain-json" => {
            print_json_help();
            exit(0);
        }
        _ => {
            print_help_text();
            exit(0);
        }
    }
}

fn print_help_text() {
    let help_text = r#"
    Simulation of finding the Minimum Energy Path on a Potential Energy Surface

    usage:
    minimum_energy_path                   simulate according to the config file
    minimum_energy_path ( --help | -h )   show this help
    minimum_energy_path --explain-json    explain the config file
    "#;
    println!("{}", help_text);
}

fn print_json_help() {
    let json_explain = r#"
    The config file is used to configure the simulation as well as the process of saving the results.

    If no config file is found an example one is created which shows approximately the energy
    landscape that was used in the lecture.

    Most numbers can be given in floating point notation with the exception of the number of elements
    in the path and the resolution of the image. Units are all refering to the simulation. The
    resolution is only a mapping onto a canvas.

    The structure of the file has to be matched precisely, because I didn't implement any smart parsing.

    Now onto explaining the options available in the config file. The example config will look similar
    to this:

    {
      "convergence_limit": 1e-3,        // if the change in average energy after a simulation step is
                                        // smaller than this value the simulation will end.
      "pes": {
        "scale": 0.1,                   // by how much the PES should be scaled. As we are doing a
                                        // simple gradient descent this is the way to set the speed
                                        // and inversely to that the stability of the simulation

        "gaussians": [                  // The PES is a linear combination of gaussian functions.
          {"a": -7.0,                   // Each function has a preexponential factor a, the peak
            "x0": 15.0,"sig_x": 5.0,    // (or valley) position (x0|y0) as well as the standard
            "y0": 7.5,"sig_y": 5.0      // deviation in each direction.
          },
          //....
          {"a": 10.0,
            "x0": 0.0,"sig_x": 3.0,
            "y0": 3.0,"sig_y": 3.0
          }
        ]
      },
      "path": {                         // The path describes the inital guess for the MEP.
        "spring_constant": 0.3,         // How much the spring forces should be rescaled. Setting this
                                        // to zero disables springs. Large values lead to instability.
        "pin_ends": true,               // If the end are allowed to move. The ends do not have a
                                        // proper tangent.
        "relax_ends": true,             // If the ends should relax to their local minimum before they
                                        // are used to construct the line of points. This can greatly
                                        // improve results. Convergence criterium for having found the
                                        // local minimum is the same as is set for the whole simulation
        "start": {                      // Start ...
          "x": 21.5,
          "y": 5.5
        },
        "end": {                        // and end points of the line.
          "x": 9.5,
          "y": 19.5
        },
        "elements": 50                  // How many points are placed along the line. The program can
                                        // deal with many, but large numbers (more than 100) lead to
                                        // severe instability.
      },
      "image": {                        // This section configures the images that are saved
        "contour_lines": 10,            // how many "contour lines" should be shown. They are not real
                                        // contour lines, but instead the value is simply wrapped
                                        // around, turning from white to black. Setting this to 1 will
                                        // result in a grayscale image.
        "x0": 0.0,                      // Where the bottom left corner of the image should be,
        "y0": 0.0,
        "width": 27.0,                  // how large of an area should be depicted...
        "height": 25.0,
        "resolution_x": 2700,           // and with which resolution this area should be rendered.
        "resolution_y": 2500,           // The resulution is usually the bottleneck for the simulation.
        "point_size": 0.1,              // How large the points along the line should be draw,
        "line_width": 0.02              // and how large the lines. Important: This is in the same
                                        // units as the PES, not in pixel units.
      }
    }
    "#;
    println!("{}", json_explain);
}