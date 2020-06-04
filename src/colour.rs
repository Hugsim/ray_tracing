use std::ops::{Add, AddAssign, Mul, MulAssign};
use crate::utility::clamp;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn as_string(&self) -> String {
        format!("{} {} {}", self.r, self.g, self.b)
    }

    pub fn map(self, mut f: impl FnMut(f64) -> f64) -> Colour {
        Colour {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
        }
    }

    pub fn print(&self, samples_per_pixel: usize) {
        let val = 1.0 / samples_per_pixel as f64;
        let col = self.map(|c| (c * val).sqrt());
        let col = col.map(|c| 
            (256.0 * clamp(0.0, 0.999, c)).floor()
        );
        assert!(!col.r.is_nan());
    
        println!("{}", col.as_string());
    }

    pub fn from(v: f64) -> Colour {
        Colour { 
            r: v,
            g: v,
            b: v,
        }
    }

    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour {r, g, b}
    }

    pub fn col_lerp(c1: Colour, c2: Colour, t: f64) -> Colour {
        assert!(0.0 <= t && t <= 1.0);
    
        (1.0 - t) * c1 + t * c2
    }

    pub const RED: Colour = Colour {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };
    
    pub const GREEN: Colour = Colour {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };
    
    pub const BLUE: Colour = Colour {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };

    pub const YELLOW: Colour = Colour {
        r: 1.0,
        g: 1.0,
        b: 0.0,
    };
    
    pub const MAGENTA: Colour = Colour {
        r: 1.0,
        g: 0.0,
        b: 1.0,
    };
    
    pub const CYAN: Colour = Colour {
        r: 0.0,
        g: 1.0,
        b: 1.0,
    };

    pub const BLACK: Colour = Colour {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    
    pub const WHITE: Colour = Colour {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
}

impl Add for Colour {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Colour {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul<f64> for Colour {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul for Colour {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<Colour> for f64 {
    type Output = Colour;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Colour {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}
