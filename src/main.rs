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
mod bvh;
mod texture;
mod perlin;

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
use bvh::*;
use texture::*;
use perlin::*;

use ::image as ext_image;
use std::path::Path;

fn main() {
    let look_from = Pos3::new(13.0, 2.0, 3.0);
    let look_at = Pos3::new(0.0, 0.0, 0.0);
    let vup = Pos3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

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

    let background = Colour::BLACK;

    eprintln!("Starting to build BVH.");

    let objects = texture_test(0.0, 1.0);

    eprintln!("Finished building BVH, starting actual ray tracing.");

    let buffer = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        let col: Colour = (0..SAMPLES_PER_PIXEL)
            .map(|_| {
                let u = (x as f64 + random_zero_one()) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (y as f64 + random_zero_one()) / (IMAGE_HEIGHT as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                assert!(!ray.direction.is_nan());
                let col = ray_colour(&objects, background, &ray, MAX_BOUNCES);
                assert!(!col.is_nan());
                col
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

fn ray_colour(world: &Objects, background_colour: Colour, ray: &Ray, depth: usize) -> Colour {
    if depth == 0 {
        return Colour::BLACK;
    }
    if let Some(hr) = world.hit(&ray, 0.001, INF) {
        if let Some((new_ray, attenuation)) = hr.material.scatter(ray, &hr) {
            let res = ray_colour(world, background_colour, &new_ray, depth - 1);
            let col = attenuation * res;
            assert!(!col.is_nan());
            col
        } else {
            hr.material.emit(hr.u, hr.v, hr.p)
        }
    } else {
        background_colour
    }
}
