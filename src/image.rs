use crate::colour::*;
use crate::consts::*;

use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::path::Path;
use image::*;

pub struct Image {
    pixels: Vec<Vec<Colour>>,
}

impl Image {
    pub fn new(width: usize, height: usize, f: impl Fn(usize, usize) -> Colour + Sync) -> Image {
        let num_lines: AtomicUsize = AtomicUsize::new(1);
        Image {
            pixels: (0..height)
                        .into_par_iter()
                        .rev()
                        .map(|y| {
                                eprintln!("Starting scanline {} ({}/{})", y, num_lines.fetch_add(1, Ordering::Relaxed), IMAGE_HEIGHT);
                                (0..width)
                                .map(|x|
                                    f(x, y)
                                ).collect()
                            }
                        ).collect(),
        }
    }

    pub fn print(self) {
        print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
        for row in self.pixels {
            for col in row {
                debug_assert!(col.all_positive_or_zero());
                col.print();
            }
        }
    }

    pub fn save(self) {
        let image_path = Path::new("out/image.png");

        let mut img = RgbImage::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

        for (y, row) in self.pixels.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                debug_assert!(col.all_positive_or_zero());
                img.put_pixel(x as u32, y as u32, *image::Pixel::from_slice(&col.as_int_array()))
            }
        }

        img.save(image_path).expect("Failed writing PNG.");
    }
}