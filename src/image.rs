use crate::pes::PES;
use crate::chain::Chain;
use crate::point::Point;
use self::image::{ImageBuffer, Rgb};

extern crate image;


// todo: make the point size configurable
// todo: make the line width configurable

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct ImageConfig {
    pub(crate) contour_lines: f64,
    pub(crate) x0: f64,
    pub(crate) y0: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) resolution_x: i32,
    pub(crate) resolution_y: i32,
    pub(crate) point_size: f64,
    pub(crate) line_width: f64,
}

#[derive(Debug)]
pub struct Image {
    config: ImageConfig,
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Image {
    pub fn new(image_config: ImageConfig, pes: &PES) -> Self {
        let mut img = Image {
            config: image_config,
            image: image::ImageBuffer::new(image_config.resolution_x as u32, image_config.resolution_y as u32),
        };
        img.initialize_pes_image(pes);
        img
    }

    ///draw the PES so we don't need to query the PES for every single pixel in every loop
    fn initialize_pes_image(&mut self, pes: &PES) {
        let config = self.config;
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        for (x, y, _pixel) in self.image.enumerate_pixels_mut() {
            let energy = pes.energy_at(config.point_for_pixel(x, y));
            if energy > max {
                max = energy;
            }
            if energy < min {
                min = energy;
            }
        }

        // paint the PES rescaled to 0-255
        for (x, y, pixel) in self.image.enumerate_pixels_mut() {
            let energy = pes.energy_at(config.point_for_pixel(x, y));
            let intensity = (energy - min) / (max - min) * 255f64 * self.config.contour_lines;
            let intensity = (intensity as u32 & 0xffu32) as u8;
            *pixel = image::Rgb([intensity, intensity, intensity]);
        }
    }

    fn draw_line(&self, image_buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, start: Point, end: Point, color: &[u8; 3]) {
        //https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)
        let (img_start_x, img_start_y) = self.config.pixel_for_point(start);
        let (img_end_x, img_end_y) = self.config.pixel_for_point(end);
        let dx = img_end_x - img_start_x;
        let dy = img_end_y - img_start_y;
        let steps = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() };
        let dx = dx as f64 / steps as f64;
        let dy = dy as f64 / steps as f64;
        let mut x = img_start_x as f64;
        let mut y = img_start_y as f64;
        for _ in 0..steps as u32 {
            if self.config.line_width / self.config.width * self.config.resolution_x as f64 <= 1.0 &&
                self.config.line_width / self.config.height * self.config.resolution_y as f64 <= 1.0 {
                if self.config.pixel_in_image(x as i32, y as i32) {
                    image_buffer.put_pixel(x as u32, y as u32, image::Rgb(*color));
                }
            } else {
                self.draw_circle(image_buffer, self.config.point_for_pixel(x as u32, y as u32), self.config.line_width, color);
            }

            x += dx;
            y += dy;
        }
    }

    fn draw_circle(&self, image_buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, p: Point, radius: f64, color: &[u8; 3]) {
        let (x, y) = self.config.pixel_for_point(p);
        let rx = (radius * self.config.resolution_x as f64 / self.config.width) as i32;
        let ry = (radius * self.config.resolution_y as f64 / self.config.height) as i32;
        for dx in -rx..rx + 1 {
            for dy in -ry..ry + 1 {
                let pos_x = x + dx;
                let pos_y = y + dy;
                if self.config.pixel_in_image(pos_x, pos_y)
                    && p.distance_sq(self.config.point_for_pixel(pos_x as u32, pos_y as u32)) < radius.powi(2) {
                    image_buffer.put_pixel(pos_x as u32, pos_y as u32, image::Rgb(*color))
                }
            }
        }
    }

    fn draw_chain(&self, image_buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, points: &Chain) {
        for point in &points.elements {
            self.draw_circle(image_buffer, *point, self.config.point_size, &[255u8, 0, 0]);
        }
    }

    fn draw_gradients(&self, image_buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, points: &Chain, pes: &PES) {
        for point in &points.elements {
            self.draw_line(image_buffer, *point, *point + pes.gradient_at(*point), &[0, 0, 255u8]);
        }
    }

    fn draw_connections(&self, image_buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, points: &Chain) {
        for i in 0..points.config.elements {
            let a = *points.elements.get(i).unwrap();
            let b = *points.elements.get(i + 1).unwrap();
            self.draw_line(image_buffer, a, b, &[0, 255u8, 0])
        }
    }

    pub fn paint(&self, filename: &str, points: &Chain, pes: &PES) {
        let mut image_buffer = self.image.clone();

        //add the points along our chain
        self.draw_chain(&mut image_buffer, points);

        //add the gradients of the points
        self.draw_gradients(&mut image_buffer, points, pes);

        //add the connections between the points
        self.draw_connections(&mut image_buffer, points);

        image_buffer.save(filename).unwrap();
    }
}

impl ImageConfig {
    ///turns a given pixel position into a coordinate point that can be used on the PES
    fn point_for_pixel(&self, x: u32, y: u32) -> Point {
        let pes_x = self.x0 + self.width * x as f64 / self.resolution_x as f64;
        let pes_y = self.y0 + self.height - self.height * y as f64 / self.resolution_y as f64;
        Point { x: pes_x, y: pes_y }
    }

    ///turns the given point into a pixel coordinate, no matter if that point is actually on the canvas
    fn pixel_for_point(&self, p: Point) -> (i32, i32) {
        let px = (p.x - self.x0) / self.width * self.resolution_x as f64;
        let py = (p.y - self.y0) / self.height * self.resolution_y as f64;
        (px as i32, self.resolution_y - 1 - py as i32)
    }

    fn pixel_in_image(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.resolution_x && y >= 0 && y < self.resolution_y
    }
}