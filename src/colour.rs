use std::ops::{Add, AddAssign, Mul, MulAssign, Div, Index};
use std::iter::Sum;
use crate::utility::*;

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn as_string(&self) -> String {
        if self.r.is_nan() || self.g.is_nan() || self.b.is_nan() {
            eprintln!("Got a NaN when printing colour, replacing with magenta.");
            assert!(!self.is_nan());
            format!("{} {} {}", 255, 0, 255)
        } else {
            format!("{} {} {}", self.r, self.g, self.b)
        }
    }

    pub fn map(self, mut f: impl FnMut(f64) -> f64) -> Colour {
        Colour {
            r: f(self.r),
            g: f(self.g),
            b: f(self.b),
        }
    }

    pub fn is_nan(&self) -> bool {
        self.any(f64::is_nan)
    }

    pub fn any_negative(&self) -> bool {
        self.any(|c| c < 0.0)
    }

    pub fn all_positive_or_zero(&self) -> bool {
        self.all(|c| c >= 0.0)
    }

    pub fn any(&self, mut f: impl FnMut(f64) -> bool) -> bool {
        f(self.r) || f(self.g) || f(self.b)
    }

    pub fn all(&self, mut f: impl FnMut(f64) -> bool) -> bool {
        f(self.r) && f(self.g) && f(self.b)
    }

    pub fn print(&self) {
        let col = self.map(|c| c.sqrt());
        let col = col.map(|c| 
            (256.0 * clamp(0.0, 0.999, c)).floor()
        );
        println!("{}", col.as_string());
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

#[allow(dead_code)]
pub fn rand_colour() -> Colour {    
    Colour::new(random_zero_one(), random_zero_one(), random_zero_one())
}

impl From<f64> for Colour {
    fn from(v: f64) -> Self {
        Colour { 
            r: v,
            g: v,
            b: v,
        }
    }
}

impl Index<usize> for Colour {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Indexing into Colour out of bounds.")
        }
    }
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

impl Sum for Colour {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Colour::from(0.0), std::ops::Add::add)
    }
}

impl Mul<f64> for Colour {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        self.map(|c| c * rhs)
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

impl Div<f64> for Colour {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        self.map(|c| c / rhs)
    }
}
