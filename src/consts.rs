pub const ASPECT_RATIO: f64 = 16.0/9.0;
pub const IMAGE_WIDTH: usize = 1920;
pub const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
pub const SAMPLES_PER_PIXEL: usize = 50;
pub const MAX_BOUNCES: usize = 20;