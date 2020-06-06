use crate::vec3::*;
use crate::ray::*;
use crate::utility::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera {
    origin: Pos3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Pos3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time_start: f64,
    time_end: f64,
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rand = self.lens_radius * random_vec_in_unit_disk();
        let offset = self.u * rand.x + self.v * rand.y;

        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset;
        let time = random_in_range(self.time_start, self.time_end);

        Ray::new(
            self.origin + offset, 
            direction,
            time,
        )
    }
}

pub fn new(
        look_from: Pos3, 
        look_at: Pos3, 
        vup: Vec3, 
        vert_fov_deg: f64, 
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time_start: f64,
        time_end: f64,
) -> Camera {
    let theta = deg_to_rad(vert_fov_deg);
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = Vec3::normalize(&(look_from - look_at));
    let u = Vec3::normalize(
        &Vec3::cross(&vup, &w)
    );
    let v = Vec3::cross(&w, &u);

    let origin = look_from;
    let horizontal = focus_dist * viewport_width * u;
    let vertical = focus_dist * viewport_height * v;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
    let lens_radius = aperture / 2.0;

    Camera {
        origin, 
        horizontal, 
        vertical, 
        lower_left_corner,
        u,
        v,
        w,
        lens_radius,
        time_start,
        time_end,
    }
}
