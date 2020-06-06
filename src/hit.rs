use crate::vec3::*;
use crate::ray::*;
use crate::material::*;
use crate::aabb::*;
use crate::scenes::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Side {
    Outside,
    Inside,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitRecord<'m> {
    pub p: Pos3,
    pub normal: Vec3,
    pub t: f64,
    pub side: Side,
    pub material: &'m Material,
}

pub trait Hit: std::fmt::Debug + Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub centre: Pos3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(centre: Pos3, radius: f64, material: Material) -> Sphere {
        Sphere { centre, radius, material }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let t = (-half_b - root) / a;
            if (t_min < t) && (t < t_max) {
                let p = ray.at(t);
                let normal = (p - self.centre) / self.radius;
                let (normal, side) = 
                    if Vec3::dot(&ray.direction, &normal) > 0.0 {
                        (-normal, Side::Inside) // front_face = false
                    } else {
                        (normal, Side::Outside) // front_face = true
                    };
                let material = &self.material;

                return Some(
                    HitRecord {
                        t, 
                        p,
                        normal,
                        side,
                        material,
                    }
                );
            } 
            let t = (-half_b + root) / a;
            if (t_min < t) && (t < t_max) {
                let p = ray.at(t);
                let normal = (p - self.centre) / self.radius;
                let side = 
                    if Vec3::dot(&ray.direction, &normal) < 0.0 {
                        Side::Outside
                    } else {
                        Side::Inside
                    };
                let material = &self.material;

                return Some(
                    HitRecord {
                        t, 
                        p,
                        normal,
                        side,
                        material,
                    }
                );
            } 
        }
        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        Some(
            Aabb::new(
                self.centre - Vec3::from(self.radius),
                self.centre - Vec3::from(self.radius)
            )
        )
    }
}

#[derive(Debug)]
pub struct LinearMove<O> {
    pub obj: O,
    pub vel: Vec3,
}

impl<O> LinearMove<O> {
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

impl Hit for Objects {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut to_return: Option<HitRecord> = None;
        let mut closest_t = t_max;

        for obj in self {
            if let Some(hr) = obj.hit(ray, t_min, closest_t) {
                to_return = Some(hr);
                closest_t = hr.t;
            }
        }

        to_return
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        if self.is_empty() {
            return None;
        }

        let mut output: Option<Aabb> = self[0].bounding_box(t0, t1);

        for obj in &self[1..] {
            let bb = obj.bounding_box(t0, t1); 
            output = Aabb::surround(output, bb);
        }

        output
    }
}
