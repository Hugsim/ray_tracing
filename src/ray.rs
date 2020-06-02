use crate::vec3::*;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Pos3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(self, t: f64) -> Pos3 {
        self.origin + self.direction * t
    }

    pub fn new(origin: Pos3, direction: Vec3) -> Ray {
        Ray {
            origin, 
            direction,
        }
    }
}