mod vec3;
mod colour;
mod ray;

use colour::*;
use vec3::*;
use ray::*;

fn main() {
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMAGE_WIDTH: usize = 384;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Pos3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Pos3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let v = j as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            
            let col = ray_colour(ray);
            col.print();
        }
    }

    eprintln!("Done!");
}

fn ray_colour(ray: Ray) -> Colour {
    if let Some(t) = hit_sphere(Pos3::new(0.0, 0.0, -1.0), 0.5, ray) {
        if t > 0.0 {
            let n = Vec3::normalize(&(ray.at(t) - Vec3::new(0.0, 0.0, -1.0)));
            return 0.5 * Colour::new(
                n.x + 1.0,
                n.y + 1.0,
                n.z + 1.0,
            );
        }
    } 
    let unit = Vec3::normalize(&ray.direction);
    let t = 0.5 * (unit.y + 1.0);
    Colour::col_lerp(Colour::WHITE, Colour::BLUE, t)
}

fn hit_sphere(centre: Pos3, radius: f64, ray: Ray) -> Option<f64> {
    let oc = ray.origin - centre;
    let a = ray.direction.length_squared();
    let half_b = Vec3::dot(&oc, &ray.direction);
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        None
    } else {
        Some( 
            (-half_b - discriminant.sqrt()) / a
        )
    }
}
