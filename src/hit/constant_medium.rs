use super::*;

pub struct ConstantMedium<O: Hit> {
    boundary: O,
    phase_function: Material,
    neg_inv_density: f64,
}

impl<O: Hit> ConstantMedium<O> {
    pub fn new(boundary: O, density: f64, phase_function: Material) -> ConstantMedium<O> {
        ConstantMedium {
            boundary,
            phase_function,
            neg_inv_density: -(1.0 / density),
        }
    }
}
impl<O: Hit> Hit for ConstantMedium<O> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self.boundary.hit(ray, std::f64::NEG_INFINITY, std::f64::INFINITY) {
            None => None,
            Some(hr1) => {
                match self.boundary.hit(ray, hr1.t + 0.0001, std::f64::INFINITY) {
                    None => None,
                    Some(hr2) => {
                        let mut min = if hr1.t < t_min {
                            t_min
                        } else {
                            hr1.t
                        };
                        let max = if hr2.t > t_max {
                            t_max
                        } else {
                            hr2.t
                        };

                        if min >= max {
                            return None;
                        }

                        if min < 0.0 {
                            min = 0.0;
                        }

                        let ray_length = ray.direction.length();
                        let distance_inside_boundary = (max - min) * ray_length;
                        let hit_distance = self.neg_inv_density * rand::random::<f64>().ln();

                        if hit_distance > distance_inside_boundary {
                            return None;
                        }

                        let t = min + hit_distance / ray_length;
                        let p = ray.at(t);

                        let normal = Vec3::new(1.0, 0.0, 0.0);
                        let side = Side::Outside;
                        let material = &self.phase_function;

                        Some(
                            HitRecord {
                                t,
                                p,
                                normal,
                                side,
                                material,
                                u: hr2.u,
                                v: hr2.v,
                            }
                        )
                    }
                }
            }
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(t0, t1)
    }
}