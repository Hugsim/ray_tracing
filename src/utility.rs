use rand::prelude as rng;

pub const PI: f64 = std::f64::consts::PI;
pub const INF: f64 = std::f64::INFINITY;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn clamp(min: f64, max: f64, x: f64) -> f64 {
    if x.is_nan() {
        return x;
    }
    
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random_zero_one() -> f64 {
    rng::random()
}

pub fn min(a: f64, b: f64) -> f64 {
    a.min(b)
}

pub fn schlick(cos: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}