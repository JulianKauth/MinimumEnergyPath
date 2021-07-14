use crate::pes::PES;
use crate::point::{Chain, Point};

extern crate image;

#[derive(Debug, Copy, Clone)]
struct Image {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    resolution: u32,
    pes: PES,
    points: Chain,
}

impl Image {
    ///turns a given pixel position into a coordinate point that can be used on the PES
    fn point_for_pixel(&self, x: u32, y: u32) -> Point {
        let pes_x = self.x0 + (self.x1 - self.x0) / self.resolution * x;
        let pes_y = self.y0 + (self.y1 - self.y0) / self.resolution * y;
        Point { x: pes_x, y: pes_y }
    }

    ///turns the given point into a valid pixel coordinate, or returns an error
    fn pixel_for_point(&self, p: Point) -> Result<(u32, u32), Err> {
        if p.x < self.x0 || p.x >= self.x1 || p.y < self.y0 || p.y >= self.y1 {
            Err("Test")
        } else {
            let px = (p.x - self.x0) / (self.x1 - self.x0) * self.resolution;
            let py = (p.y - self.y0) / (self.y1 - self.y0) * self.resolution;
            (px, py)
        }
    }

    pub fn paint(&self, filename: &str) {
        // initialize the image buffer
        let mut imgbuf = image::ImageBuffer::new(self.resolution, self.resolution);

        // figure out what our minimum and maximum pixel values will be
        let min = for (x, y, _pixel) in imgbuf.enumerate_pixels_mut() {
            self.pes.energy_at(self.point_for_pixel(x, y))
        }.min();
        let max = for (x, y, _pixel) in imgbuf.enumerate_pixels_mut() {
            self.pes.energy_at(self.point_for_pixel(x, y))
        }.max();

        // paint the PES rescaled to 0-255
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let energy = self.pes.energy_at(self.point_for_pixel(x, y));
            let intensity = ((energy - min) / (max - min) * 255) as u8;
            *pixel = image::Rgb([intensity, intensity, intensity]);
        }

        //add the points along our chain
        for point in self.points {
            match self.pixel_for_point(point) {
                Ok((x, y)) => { imgbuf.put_pixel(x, y, image::Rgb([255, 0, 0])) }
                Err(_err) => {}
            }
        }

        imgbuf.save(filename).unwrap();
    }
}