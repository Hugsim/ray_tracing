mod colour;
mod vec3;
mod ray;
mod hit;
mod utility;
mod camera;
mod material;
mod image;
mod scenes;
mod consts;
mod aabb;

use colour::*;
use vec3::*;
use ray::*;
use hit::*;
use utility::*;
use camera::*;
use material::*;
use crate::image::*;
use scenes::*;
use consts::*;
use aabb::*;

use ::image as ext_image;
use std::path::Path;

fn main() {
    let look_from = Pos3::new(13.0, 2.0, 3.0);
    let look_at = Pos3::new(0.0, 0.0, 0.0);
    let vup = Pos3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = camera::new(
        look_from,
        look_at, 
        vup, 
        20.0, 
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let objects = final_scene_1();

    let buffer = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        let col: Colour = (0..SAMPLES_PER_PIXEL)
            .map(|_| {
                let u = (x as f64 + random_zero_one()) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (y as f64 + random_zero_one()) / (IMAGE_HEIGHT as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                ray_colour(&objects, &ray, MAX_BOUNCES)
            })
            .sum();

        col / SAMPLES_PER_PIXEL as f64
    });

    eprintln!("Raytracing done, saving PPM to disk.");

    buffer.print();

    eprintln!("PPM saved, converting to PNG.");

    let img = ext_image::open(Path::new("./out/image.ppm")).expect("Failed reading PPM-file.");

    img.save(Path::new("./out/image.png")).expect("Failed writing PNG.");

    eprintln!("Done!");
}

fn hits_any<'w>(world: &'w Objects, t_min: f64, t_max: f64, ray: &Ray) -> Option<HitRecord<'w>> {
    let mut to_return: Option<HitRecord> = None;
    let mut closest_t = t_max;

    for obj in world {
        if let Some(hr) = obj.hit(ray, t_min, closest_t) {
            to_return = Some(hr);
            closest_t = hr.t;
        }
    }

    to_return
}

fn ray_colour(world: &Objects, ray: &Ray, depth: usize) -> Colour {
    if depth == 0 {
        return Colour::BLUE;
    }
    if let Some(hr) = hits_any(world, 0.001, INF, &ray) {
        if let Some((new_ray, attenuation)) = hr.material.scatter(ray, &hr) {
            return attenuation * ray_colour(world, &new_ray, depth - 1);
        } else {
            return Colour::from(0.0);
        }
    } 

    let unit = Vec3::normalize(&ray.direction);
    let t = 0.5 * (unit.y + 1.0);
    Colour::col_lerp(Colour::WHITE, Colour::BLUE, t)
}
