use crate::vec3::*;
use crate::ray::*;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Pos3,
    pub max: Pos3,
}

impl Aabb {
    // Check 3.5 in book 2 for maybe optimized version
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        // for a in 0..=2 {
        //     let t0 = (self.min[a] - ray.origin[a]) / ray.direction[a];
        //     let t1 = (self.max[a] - ray.origin[a]) / ray.direction[a];

        //     let new_t_min = t0.max(t_min);
        //     let new_t_max = t1.min(t_max);
        //     if new_t_max <= new_t_min {
        //         return false;
        //     }
        // }
        // true

        let inv_d = ray.direction.map(|x| 1. / x);
        assert!(inv_d.is_nan());
        let t0 = (self.min - ray.origin) * inv_d;
        let t1 = (self.max - ray.origin) * inv_d;
        let (t0, t1) = (
            inv_d.zip_with3(t0, t1, |i, a, b| if i < 0. { b } else { a }),
            inv_d.zip_with3(t0, t1, |i, a, b| if i < 0. { a } else { b }),
        );
        let start = t_min.max(t0.fold(f64::max));
        let end = t_max.min(t1.fold(f64::min));
        end > start
    }

    pub fn new(min: Pos3, max: Pos3) -> Aabb {
        Aabb {
            min,
            max,
        }
    }

    pub fn surround(bb1: Option<Aabb>, bb2: Option<Aabb>) -> Option<Aabb> {
        match (bb1, bb2) {
            (None, _) | (_, None) => None,
            (Some(bb1), Some(bb2)) => 
                Some(
                    Aabb {
                        min: Pos3 {
                            x: bb1.min.x.min(bb2.min.x),
                            y: bb1.min.y.min(bb2.min.y),
                            z: bb1.min.z.min(bb2.min.z),
                        },
                        max: Pos3 {
                            x: bb1.max.x.max(bb2.max.x),
                            y: bb1.max.y.max(bb2.max.y),
                            z: bb1.max.z.max(bb2.max.z),
                        }
                    }
                )
        }
    }
}
