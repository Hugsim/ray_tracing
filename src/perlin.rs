use rand::{thread_rng, Rng};
use std::convert::TryInto;

use crate::consts::*;
use crate::vec3::*;

pub struct Perlin {
    rand_vecs: Vec<Vec3>,
    perm_x: Vec<u8>,
    perm_y: Vec<u8>,
    perm_z: Vec<u8>,
}

impl Perlin {
    #[allow(dead_code)]
    pub fn new() -> Perlin {
        let mut rng = thread_rng();

        let mut rand_vecs: Vec<Vec3> = Vec::with_capacity(PERLIN_POINT_COUNT);
        for _ in 0..PERLIN_POINT_COUNT {
            rand_vecs.push(Vec3::normalize(&random_vec_in_unit_sphere()));
        }

        let perm_x = Perlin::gen_perm(&mut rng);
        let perm_y = Perlin::gen_perm(&mut rng);
        let perm_z = Perlin::gen_perm(&mut rng);

        Perlin {
            rand_vecs,
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
        let uvw = p.map(|c| c - c.floor());

        let ijk = p.map(f64::floor);

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::from(0.0); 2]; 2]; 2];

        #[allow(clippy::needless_range_loop)]
        for x in 0..2 {
            for y in 0..2 {
                for z in 0..2 {
                    c[x][y][z] = self.rand_vecs[
                        self.perm_x[(ijk.x as usize + x) & 255] as usize ^
                        self.perm_y[(ijk.y as usize + y) & 255] as usize ^
                        self.perm_z[(ijk.z as usize + z) & 255] as usize 
                    ];
                    debug_assert!(!c[x][y][z].is_nan());
                }
            }
        }

        Perlin::trilinear_interp(c, uvw)
    } 


    pub fn turb(&self, p: Pos3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut p_temp = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(p);
            weight *= 0.5;
            p_temp *= 2.0;
        }

        accum.abs()
    }

    #[allow(clippy::needless_range_loop)]
    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], uvw: Vec3) -> f64 {
        let uvw_2 = uvw.map(|c| c * c * (3.0 - 2.0 * c));
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let ijk = Vec3::new(i as f64, j as f64, k as f64);
                    let weight_v = uvw - ijk;
                    
                    let val = (i as f64 * uvw_2.x + (1.0 - i as f64) * (1.0 - uvw_2.x)) *
                              (j as f64 * uvw_2.y + (1.0 - j as f64) * (1.0 - uvw_2.y)) *
                              (k as f64 * uvw_2.z + (1.0 - k as f64) * (1.0 - uvw_2.z)) * 
                              Vec3::dot(&c[i][j][k], &weight_v);
                    // let fst = ijk * uvw;
                    // let snd = Vec3::from(1.0) - ijk;
                    // let trd = Vec3::from(1.0) - uvw;
                    // let val = fst * snd * trd;
                    // let acc = val.fold(std::ops::Mul::mul);
                    // debug_assert!(acc >= 0.0);
                    // accum += acc * c[i][j][k];
                    accum += val;
                }
            }
        }
        accum
    }
}

impl Default for Perlin {
    fn default() -> Perlin { 
        Perlin::new()
    }
}