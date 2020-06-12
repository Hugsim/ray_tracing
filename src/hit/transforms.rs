use super::*;

#[derive(Debug)]
pub struct LinearMove<O: Hit> {
    pub obj: O,
    pub vel: Vec3,
}

impl<O: Hit> LinearMove<O> {
    pub fn new(vel: Vec3, obj: O) -> LinearMove<O> {
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
    pub fn new(obj: O) -> FlipNormals<O> {
        FlipNormals(obj)
    }
}

impl<O: Hit> Hit for FlipNormals<O> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(hr) = self.0.hit(ray, t_min, t_max) {
            Some(
                HitRecord {
                    //normal: -hr.normal,
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