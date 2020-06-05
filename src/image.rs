use rayon::prelude::*;
use crate::colour::*;

pub struct Image {
    pixels: Vec<Vec<Colour>>,
}

impl Image {
    pub fn new(width: usize, height: usize, f: impl Fn(usize, usize) -> Colour + Sync) -> Image {
        Image {
            pixels: (0..height)
                        .into_par_iter()
                        .rev()
                        .map(|y| {
                                eprintln!("Starting scanline {}", y);
                                (0..width)
                                .map(|x|
                                    f(x, y)
                                ).collect()
                            }
                        ).collect(),
        }
    }

    pub fn print(self) {
        for row in self.pixels {
            for col in row {
                col.print();
            }
        }
    }
}