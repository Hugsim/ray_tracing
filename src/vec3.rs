use std::ops::{Add, AddAssign, Neg, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use rand::{thread_rng, Rng};
use crate::utility::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Pos3 = Vec3;

impl Vec3 {
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn as_string(self) -> String {
        format!("{} {} {}", self.x, self.y, self.z)
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn normalize(v: &Vec3) -> Vec3 {
        *v / v.length()
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - 2.0 * Vec3::dot(&self, &normal) * normal
    }

    pub fn refract(self, eta_over_etap: f64, normal: Vec3) -> Option<Vec3> {
        let uv = Vec3::normalize(&self);
        let dt = Vec3::dot(&uv, &normal);

        let disc = 1.0 - eta_over_etap * eta_over_etap * ( 1.0 - dt * dt );

        if disc > 0.0 {
            Some(eta_over_etap * (uv - dt * normal) - disc.sqrt() * normal)
        } else {
            None
        }
    }
}

pub fn random_vec() -> Vec3 {
    let mut rng = thread_rng();
    
    Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0))
}

pub fn random_unit_vec() -> Vec3 {
    let mut rng = thread_rng();
    let a: f64 = rng.gen_range(0.0, 2.0 * PI);
    let z: f64 = rng.gen_range(-1.0, 1.0);
    let r: f64 = (1.0 - z * z).sqrt();
    Vec3::new(
        r * a.cos(),
        r * a.sin(),
        z,
    )
}

pub fn random_vec_in_unit_sphere() -> Vec3 {
    loop {
        let v = random_vec();
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

pub fn random_vec_in_hemisphere(normal: &Vec3) -> Vec3 {
    let vec = random_vec_in_unit_sphere();
    if Vec3::dot(normal, &vec) > 0.0 {
        vec
    } else {
        -vec
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}
