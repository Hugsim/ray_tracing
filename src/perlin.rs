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
        let ijk = p.map(|c| (((4.0 * c) as isize) & 255) as f64);

        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

        for x in 0..2 {
            for y in 0..2 {
                for z in 0..2 {
                    c[x][y][z] = self.rand_float[
                        self.perm_x[(ijk.x as usize + x) & 255] as usize ^
                        self.perm_y[(ijk.y as usize + y) & 255] as usize ^
                        self.perm_z[(ijk.z as usize + z) & 255] as usize 
                    ];
                    if c[x][y][z].is_nan() {
                        panic!("part of array is NaN! {}", c[x][y][z])
                    }
                }
            }
        }

        Perlin::trilinear_interp(c, uvw)
    } 

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], uvw: Vec3) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let ijk = Vec3::new(i as f64, j as f64, k as f64);
                    accum += (i as f64 * uvw.x + (1.0 - i as f64) * (1.0 - uvw.x)) *
                             (j as f64 * uvw.y + (1.0 - j as f64) * (1.0 - uvw.y)) *
                             (k as f64 * uvw.z + (1.0 - k as f64) * (1.0 - uvw.z)) * c[i][j][k];
                    // let fst = ijk * uvw;
                    // let snd = Vec3::from(1.0) - ijk;
                    // let trd = Vec3::from(1.0) - uvw;
                    // let val = fst * snd * trd;
                    // let acc = val.fold(std::ops::Mul::mul);
                    // accum += acc * c[i][j][k]
                }
            }
        }
        accum
    }
}