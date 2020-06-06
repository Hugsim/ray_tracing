use crate::vec3::*;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Pos3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn at(self, t: f64) -> Pos3 {
        self.origin + self.direction * t
    }

    pub fn new(origin: Pos3, direction: Vec3, time: f64) -> Ray {
        Ray {
            origin, 
            direction,
            time,
        }
    }
}