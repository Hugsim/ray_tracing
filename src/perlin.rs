use rand::{thread_rng, Rng};
use std::convert::TryInto;

use crate::consts::*;
use crate::vec3::*;

pub struct Perlin {
    rand_float: Vec<f64>,
    perm_x: Vec<u8>,
    perm_y: Vec<u8>,
    perm_z: Vec<u8>,
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut rng = thread_rng();

        let mut rand_float: Vec<f64> = Vec::with_capacity(PERLIN_POINT_COUNT);
        for i in 0..PERLIN_POINT_COUNT {
            rand_float.push(rng.gen());
            eprintln!("")
        }
        let perm_x = Perlin::gen_perm(&mut rng);
        let perm_y = Perlin::gen_perm(&mut rng);
        let perm_z = Perlin::gen_perm(&mut rng);

        Perlin {
            rand_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    fn gen_perm(rng: &mut impl Rng) -> Vec<u8> {
        let mut p: Vec<u8> = Vec::with_capacity(PERLIN_POINT_COUNT);
        for i in 0..PERLIN_POINT_COUNT {
            p.push((i as usize).try_into().unwrap());
        }
        for i in (1..PERLIN_POINT_COUNT).rev() {
            p.swap(i, rng.gen_range(0, i));
        }
        p
    }

    pub fn noise(&self, p: Pos3) -> f64 {
        let uvw = p.map(f64::fract);
        let ijk = uvw.map(|c| (((4.0 * c) as isize) & 255) as f64);

        self.rand_float[self.perm_x[ijk.x as usize] as usize ^ self.perm_y[ijk.y as usize] as usize ^ self.perm_z[ijk.z as usize] as usize]
    } 
}