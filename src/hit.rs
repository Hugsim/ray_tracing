use crate::Vec3::*;
use crate::Ray::*;

#[derive(Debug, Clone, Copy)]
pub struct Hit_Record {
    p: Pos3,
    normal: Vec3,
    t: f64,
}

pub trait Hit {
    pub fn hit(ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit_Record>;
}