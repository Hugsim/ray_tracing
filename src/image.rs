use crate::colour::*;
use crate::consts::*;

use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Image {
    pixels: Vec<Vec<Colour>>,
}

impl Image {
    pub fn new(width: usize, height: usize, f: impl Fn(usize, usize) -> Colour + Sync) -> Image {
        let num_lines: AtomicUsize = AtomicUsize::new(0);
        Image {
            pixels: (0..height)
                        .into_par_iter()
                        .rev()
                        .map(|y| {
                                eprintln!("Starting scanline {} ({}/{})", y, num_lines.fetch_add(1, Ordering::Relaxed) + 1, IMAGE_HEIGHT);
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
                col.print();
            }
        }
    }
}