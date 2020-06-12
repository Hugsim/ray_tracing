use crate::colour::*;
use crate::vec3::*;
use crate::perlin::*;
use crate::utility::*;

use std::sync::Arc;
use std::path::Path;

use ::image::*;

pub type Texture = Arc<dyn Fn(f64, f64, Vec3) -> Colour + Send + Sync>;

pub fn solid_colour(col: Colour) -> Texture {
    Arc::new(
        move |_, _, _| col
    )
}

pub fn checkered(t1: Texture, t2: Texture) -> Texture {
    Arc::new(
        move |u, v, p| {
            let val = (10.0 * p).map(f64::sin).fold(std::ops::Mul::mul);

            if val < 0.0 {
                t1(u, v, p)
            } else {
                t2(u, v, p)
            }
        }
    )
}

pub fn noise(noise: Perlin, scale: f64) -> Texture {
    Arc::new(
        move |_, _, p| {
            Colour::from(1.0) * 0.5 * (1.0 + (scale * p.z + 5.0 * noise.turb(p * scale, 7)).sin())
        }
    )
}

pub fn image(image: &Path) -> Texture {
    let image = image::open(image);

    if let Ok(image) = image {

        if let DynamicImage::ImageRgb8(image) = image {
            Arc::new(
                move |u, v, _| {
                    let (width, height) = &image.dimensions();
                    let width = *width;
                    let height = *height;

                    let u = clamp(0.0, 1.0, u);
                    let v = 1.0 - clamp(0.0, 1.0, v);

                    let mut x = (u * width as f64).floor() as u32;
                    let mut y = (v * height as f64).floor() as u32;

                    if x >= width {
                        x = width - 1;
                    }
                    if y >= height {
                        y = height - 1;
                    }

                    let pixel = image.get_pixel(x, y).channels();

                    Colour::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64).map(|c| c / 255.0)
                }
            )
        } else {
            panic!("Got weird image when opening texture");
        }
    } else {
        eprintln!("Failed opening image for a texture!");
        Arc::new(
            move |_, _, _| {
                Colour::MAGENTA
            }
        )
    }    
}
