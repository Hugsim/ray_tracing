mod colour;
mod vec3;
mod ray;
mod hit;
mod utility;
mod camera;
mod material;
mod image;

use colour::*;
use vec3::*;
use ray::*;
use hit::*;
use utility::*;
use camera::*;
use material::*;
use image::*;

type Objects = Vec<Box<dyn Hit>>;

fn main() {
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMAGE_WIDTH: usize = 1920;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 50;
    const MAX_BOUNCES: usize = 50;

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut objects: Objects = vec![];

    /*
            Two touching spheres
    */
    // let r = (PI / 4.0).cos();
    // objects.push(
    //     Box::new(
    //         Sphere {
    //             centre: Pos3::new(-r, 0.0, -1.0), 
    //             radius: r,
    //             material: Material::Lambertian {
    //                 albedo: Colour::new(0.0, 0.0, 1.0),
    //             },
    //         }
    //     )
    // );
    // objects.push(
    //     Box::new(
    //         Sphere {
    //             centre: Pos3::new(r, 0.0, -1.0), 
    //             radius: r,
    //             material: Material::Lambertian {
    //                 albedo: Colour::new(1.0, 0.0, 0.0),
    //             },
    //         }
    //     )
    // );


    /*
            Scene with three spheres, one metallic, one matte and one hollow glass
    */
    // objects.push(
    //     Box::new(
    //         Sphere {
    //             centre: Pos3::new(0.0, 0.0, -1.0), 
    //             radius: 0.5,
    //             material: Material::Lambertian {
    //                 albedo: Colour::new(0.1, 0.2, 0.5),
    //             },
    //         }
    //     )
    // );
    // objects.push(
    //     Box::new(
    //         Sphere {
    //             centre: Pos3::new(0.0, -100.5, -1.0), 
    //             radius: 100.0,
    //             material: Material::Lambertian {
    //                 albedo: Colour::new(0.8, 0.8, 0.0),
    //             },
    //         }
    //     )
    // );
    // objects.push(
    //     Box::new(
    //         Sphere {
    //             centre: Pos3::new(1.0, 0.0, -1.0), 
    //             radius: 0.5,
    //             material: Material::Metal {
    //                 albedo: Colour::new(0.8, 0.6, 0.2),
    //                 fuzziness: 0.0,
    //             },
    //         }
    //     )
    // );
    // objects.push(
    //     Box::new(
    //         Sphere {
    //             centre: Pos3::new(-1.0, 0.0, -1.0), 
    //             radius: 0.5,
    //             material: Material::Dielectric {
    //                 refractive_index: 1.5,
    //             },
    //         }
    //     )
    // );
    // objects.push(
    //     Box::new(
    //         Sphere {
    //             centre: Pos3::new(-1.0, 0.0, -1.0), 
    //             radius: -0.45,
    //             material: Material::Dielectric {
    //                 refractive_index: 1.5,
    //             },
    //         }
    //     )
    // );

    objects.push(
        Box::new(
            Sphere {
                centre: Pos3::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
                material: Material::Lambertian {
                    albedo: Colour::from(0.5),
                },
            }
        )
    );
    for a in -11..11 {
        for b in -11..11 {
            let mat = random_zero_one();
            let centre = Pos3::new(a as f64 + 0.9 * random_zero_one(), 0.2, b as f64 + 0.9 * random_zero_one());
            if (centre - Pos3::new(4.0, 0.2, 0.0)).length() > 0.8 {
                if mat < 0.8 {
                    let albedo = rand_colour() * rand_colour();
                    objects.push(
                        Box::new(
                            Sphere {
                                centre,
                                radius: 0.2,
                                material: Material::Lambertian {
                                    albedo,
                                },
                            }
                        )
                    );
                } else if mat < 0.95 {
                    let albedo = rand_colour() * rand_colour();
                    let fuzziness = random_zero_one() / 2.0 + 0.5;
                    objects.push(
                        Box::new(
                            Sphere {
                                centre,
                                radius: 0.2,
                                material: Material::Metal {
                                    albedo,
                                    fuzziness,
                                },
                            }
                        )
                    );
                } else {
                    objects.push(
                        Box::new(
                            Sphere {
                                centre,
                                radius: 0.2,
                                material: Material::Dielectric {
                                    refractive_index: 1.5,
                                },
                            }
                        )
                    );
                }
            }
        }
    }
    objects.push(
        Box::new(
            Sphere {
                centre: Pos3::new(0.0, 1.0, 0.0),
                radius: 1.0,
                material: Material::Dielectric {
                    refractive_index: 1.5,
                },
            }
        )
    );
    objects.push(
        Box::new(
            Sphere {
                centre: Pos3::new(-4.0, 1.0, 0.0),
                radius: 1.0,
                material: Material::Lambertian {
                    albedo: Colour::new(0.4, 0.2, 0.1),
                },
            }
        )
    );
    objects.push(
        Box::new(
            Sphere {
                centre: Pos3::new(4.0, 1.0, 0.0),
                radius: 1.0,
                material: Material::Metal {
                    albedo: Colour::new(0.7, 0.6, 0.5),
                    fuzziness: 0.0,
                },
            }
        )
    );

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
    );

    // for j in (0..IMAGE_HEIGHT).rev() {
    //     eprintln!("Scanlines remaining: {}", j);
    //     for i in 0..IMAGE_WIDTH {
    //         let mut col = Colour::from(0.0);
    //         for _ in 0..SAMPLES_PER_PIXEL {            
    //             let u = (i as f64 + random_zero_one()) / (IMAGE_WIDTH as f64 - 1.0);
    //             let v = (j as f64 + random_zero_one()) / (IMAGE_HEIGHT as f64 - 1.0);
    //             let ray = camera.get_ray(u, v);
    //             col += ray_colour(&objects, &ray, MAX_BOUNCES);
    //         }
    //         col.print();
    //     }
    // }

    Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        let col: Colour = (0..SAMPLES_PER_PIXEL)
            .map(|_| {
                let u = (x as f64 + random_zero_one()) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (y as f64 + random_zero_one()) / (IMAGE_HEIGHT as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                ray_colour(&objects, &ray, MAX_BOUNCES)
            })
            .sum();

        col / SAMPLES_PER_PIXEL as f64
    }).print();

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
