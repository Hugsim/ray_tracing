use super::*;

#[derive(Debug)]
pub struct LinearMove<O: Hit> {
    pub obj: O,
    pub vel: Vec3,
}

impl<O: Hit> LinearMove<O> {
    #[allow(dead_code)]
    pub fn new(obj: O, vel: Vec3) -> LinearMove<O> {
        LinearMove {
            obj,
            vel,
        }
    }
}

impl<O: Hit> Hit for LinearMove<O> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.obj.hit(
            &Ray {
                origin: ray.origin - self.vel * ray.time,
                ..*ray
            }, 
            t_min, 
            t_max,
        )
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        if let Some(bound) = self.obj.bounding_box(t0, t1) {
            Aabb::surround(
                Some(
                    Aabb::new(
                        bound.min + t0 * self.vel, 
                        bound.max + t0 * self.vel
                    )
                ),
                Some(
                    Aabb::new(
                        bound.min + t1 * self.vel, 
                        bound.max + t1 * self.vel
                    )
                )
            )
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct FlipNormals<O: Hit>(pub O);

impl<O: Hit> FlipNormals<O> {
    #[allow(dead_code)]
    pub fn new(obj: O) -> FlipNormals<O> {
        FlipNormals(obj)
    }
}

impl<O: Hit> Hit for FlipNormals<O> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(hr) = self.0.hit(ray, t_min, t_max) {
            Some(
                HitRecord {
                    side: !hr.side,
                    ..hr
                }
            )
        }
        else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.0.bounding_box(t0, t1)
    }
}

pub struct Translate<O: Hit> {
    obj: O,
    offset: Vec3,
}

impl <O: Hit> Translate<O> {
    pub fn new(obj: O, offset: Vec3) -> Translate<O> {
        Translate {
            obj,
            offset,
        }
    }
}

impl<O: Hit> Hit for Translate<O> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let new_ray = Ray {
            origin: ray.origin - self.offset,
            ..*ray
        };

        if let Some(hr) = self.obj.hit(&new_ray, t_min, t_max) {
            let p = hr.p + self.offset;
            let (normal, side) = HitRecord::face(&hr.normal, &new_ray);
            Some(
                HitRecord {
                    p,
                    normal,
                    side,
                    ..hr
                }
            )
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.obj.bounding_box(t0, t1).map(|bb| 
            Aabb {
                min: bb.min + self.offset,
                max: bb.max + self.offset,
            }
        )
    }
} 

pub struct RotateY<O: Hit> {
    pub obj: O,
    sin_theta: f64,
    cos_theta: f64,
}

impl<O: Hit> RotateY<O> {
    pub fn new(obj: O, deg: f64) -> RotateY<O> {
        let rads = deg_to_rad(deg);
        let sin_theta = rads.sin();
        let cos_theta = rads.cos();

        RotateY {
            obj,
            sin_theta,
            cos_theta,
        }
    }
}

impl<O: Hit> Hit for RotateY<O> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        fn rot(p: Pos3, sin_theta: f64, cos_theta: f64) -> Pos3 {   
            Vec3::new(
                Vec3::dot(&p, &Vec3::new(cos_theta, 0.0, sin_theta)),
                Vec3::dot(&p, &Vec3::new(0.0, 1.0, 0.0)),
                Vec3::dot(&p, &Vec3::new(-sin_theta, 0.0, cos_theta)),
            )
        }

        let new_ray = Ray {
            origin: rot(ray.origin, -self.sin_theta, self.cos_theta),
            direction: rot(ray.direction, -self.sin_theta, self.cos_theta),
            ..*ray
        };

        self.obj.hit(&new_ray, t_min, t_max).map(|hr|
            HitRecord {
                p: rot(hr.p, self.sin_theta, self.cos_theta),
                normal: rot(hr.normal, self.sin_theta, self.cos_theta),
                ..hr
            }
        )
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        fn rot(p: Pos3, sin_theta: f64, cos_theta: f64) -> Pos3 {   
            Vec3::new(
                Vec3::dot(&p, &Vec3::new(cos_theta, 0.0, sin_theta)),
                Vec3::dot(&p, &Vec3::new(0.0, 1.0, 0.0)),
                Vec3::dot(&p, &Vec3::new(-sin_theta, 0.0, cos_theta)),
            )
        }

        self.obj.bounding_box(t0, t1).map(|bb| {
            let mut min = Vec3::from(std::f64::INFINITY);
            let mut max = Vec3::from(std::f64::NEG_INFINITY);

            for x in 0..3 {
                for y in 0..3 {
                    for z in 0..3 {
                        let xyz = Vec3::new(x as f64, y as f64, z as f64);

                        let xyz = xyz * bb.min + xyz.map(|c| 1.0 - c) * bb.min;

                        let tester = rot(xyz, self.cos_theta, self.sin_theta);

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }

            Aabb {
                min,
                max,
            }
        })
    }
}