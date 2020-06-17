pub const ASPECT_RATIO: f64 = 1.0;
pub const IMAGE_WIDTH: usize = 600;
pub const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
pub const SAMPLES_PER_PIXEL: usize = 10_000;
pub const MAX_BOUNCES: usize = 50;
pub const PERLIN_POINT_COUNT: usize = 256;