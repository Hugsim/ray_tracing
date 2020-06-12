use crate::vec3::*;
use crate::ray::*;
use crate::material::*;
use crate::aabb::*;
use crate::scenes::*;
use crate::utility::*;

use std::ops::Not;

pub mod cuboid;
pub mod rect;
pub mod sphere;
pub mod transforms;

pub use cuboid::*;
pub use rect::*;
pub use sphere::*;
pub use transforms::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Side {
    Outside,
    Inside,
}

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Side::Outside => Side::Inside,
            Side::Inside => Side::Outside,
        }
    }
}

#[derive(Clone, Copy)]
pub struct HitRecord<'m> {
    pub p: Pos3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub side: Side,
    pub material: &'m Material,
}

impl HitRecord<'_> {
    pub fn face(normal: &Vec3, ray: &Ray) -> (Vec3, Side) {
        if Vec3::dot(&ray.direction, &normal) > 0.0 {
            (-*normal, Side::Inside) // front_face = false
        } else {
            (*normal, Side::Outside) // front_face = true
        }
    }
}

pub trait Hit: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb>;
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
