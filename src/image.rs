use crate::pes::PES;
use crate::point::{Chain, Point};

extern crate image;

#[derive(Debug)]
pub struct Image {
    x0: f64,
    y0: f64,
    width: f64,
    height: f64,
    resolution: u32,
    pes: PES,
}

impl Image {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, resolution: u32, pes: PES) -> Self {
        Image {
            x0,
            y0,
            width: x1 - x0,
            height: y1 - y0,
            resolution,
            pes,
        }
    }

    ///turns a given pixel position into a coordinate point that can be used on the PES
    fn point_for_pixel(&self, x: u32, y: u32) -> Point {
        let pes_x = self.x0 + self.width * x as f64 / self.resolution as f64;
        let pes_y = self.y0 + self.height - self.height * y as f64 / self.resolution as f64;
        Point { x: pes_x, y: pes_y }
    }

    ///turns the given point into a valid pixel coordinate, or returns an error
    fn pixel_for_point(&self, p: Point) -> Option<(u32, u32)> {
        if p.x >= self.x0 && p.x < self.x0 + self.width && p.y >= self.y0 && p.y < self.y0 + self.height {
            let px = (p.x - self.x0) / self.width * self.resolution as f64;
            let py = (p.y - self.y0) / self.height * self.resolution as f64;
            Some((px as u32, self.resolution - 1 - py as u32))
        } else {
            None
        }
    }

    pub fn paint(&self, filename: &str, points: Chain) {
        // initialize the image buffer
        let mut imgbuf = image::ImageBuffer::new(self.resolution, self.resolution);

        // figure out what our minimum and maximum pixel values will be
        let min = imgbuf.enumerate_pixels_mut().map(
            |(x, y, _pixel)| self.pes.energy_at(self.point_for_pixel(x, y))
        ).min_by(|&x, &y| x.partial_cmp(&y).unwrap()).unwrap();
        let max = imgbuf.enumerate_pixels_mut().map(
            |(x, y, _pixel)| self.pes.energy_at(self.point_for_pixel(x, y))
        ).max_by(|&x, &y| x.partial_cmp(&y).unwrap()).unwrap();

        // paint the PES rescaled to 0-255
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let energy = self.pes.energy_at(self.point_for_pixel(x, y));
            let intensity = ((energy - min) / (max - min) * 255f64) as u8;
            *pixel = image::Rgb([intensity, intensity, intensity]);
        }

        //add the points along our chain
        for point in &points.elements {
            match self.pixel_for_point(*point) {
                Some((x, y)) => { imgbuf.put_pixel(x, y, image::Rgb([255, 0, 0])) }
                None => {}
            }
        }

        imgbuf.save(filename).unwrap();
    }
}