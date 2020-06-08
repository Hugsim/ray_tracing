use crate::colour::*;
use crate::vec3::*;
use crate::perlin::*;

use std::sync::Arc;

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

pub fn noise(noise: Perlin) -> Texture {
    Arc::new(
        move |_, _, p| {
            Colour::from(1.0) * noise.noise(p)
        }
    )
}