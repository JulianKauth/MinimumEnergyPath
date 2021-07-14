use crate::pes::PES;
use crate::pes::Gaussian;
use crate::image::Image;
use crate::chain::Chain;
use crate::point::Point;

mod point;
mod pes;
mod image;
mod chain;

fn main() {
    //create mep
    let pes = PES {
        gaussians: vec![
            Gaussian { a: 1e1, x0: 5.0, sig_x: 2.0, y0: 5.0, sig_y: 2.0 },
            Gaussian { a: 1e1, x0: 0.0, sig_x: 2.0, y0: 5.0, sig_y: 2.0 },
            Gaussian { a: 1e1, x0: 5.0, sig_x: 2.0, y0: 0.0, sig_y: 2.0 }
        ]
    };
    //create chain
    let chain = Chain::new(
        Point { x: 7.5, y: 0.0 },
        Point { x: 0.0, y: 7.5 },
        20,
    );
    //set up our image generator
    let img = Image::new(0.0, 10.0, 0.0, 10.0, 200, 200);
    img.paint("test.png", &chain, &pes);

    // iterate until we reached a stable state
    for i in 0..4{
        //move to a better position
        chain.iterate();
        println!("{}", chain.energy(&pes));
        // save the state
        img.paint(&*format!("progress_{}.png", i), &chain, &pes);
    }
    //iterate
    //  move chain
    //  image
}
