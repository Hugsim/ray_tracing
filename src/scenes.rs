#![allow(clippy::redundant_clone)]
#![allow(unused_variables)]

use crate::material::*;
use crate::vec3::*;
use crate::colour::*;
use crate::utility::*;
use crate::bvh::*;
use crate::texture::*;
use crate::perlin::*;
use crate::hit::*;
use crate::camera::*;
use crate::consts::*;

use std::path::Path;

pub type Objects = Vec<Box<dyn Hit>>;

pub fn final_scene_2(t_min: f64, t_max: f64) -> (Camera, Objects) {
    // Floor boxes
    let ground_mat = Material::Lambertian {
        albedo: solid_colour(Colour::new(0.48, 0.83, 0.53)),
    };

    let mut objects: Objects = Vec::new();

    let boxes_per_side = 20;
    let mut box_vec: Objects = Vec::with_capacity(boxes_per_side * boxes_per_side);

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let width = 100.0;
            let x_0 = -1000.0 + i as f64 * width;
            let z_0 = -1000.0 + j as f64 * width;
            let y_0 = 0.0;
            let x_1 = x_0 + width;
            let y_1 = random_in_range(1.0, 100.0);
            let z_1 = z_0 + width;

            box_vec.push(
                Box::new(
                    Cuboid::new(
                        Pos3::new(x_0, y_0, z_0),
                        Pos3::new(x_1, y_1, z_1),
                        ground_mat.clone(),
                    )   
                )
            );
        }
    }

    objects.push(
        Box::new(
            Bvh::new(
                box_vec,
                t_min,
                t_max,
            )
        )  
    );

    // Light
    let light_mat = Material::DiffuseLight {
        emit: solid_colour(Colour::from(7.0)),
    };
    objects.push(
        Box::new(
            XZRect::new(
                123.0, 
                423.0, 
                147.0, 
                412.0, 
                554.0, 
                light_mat,
            )
        )
    );

    // Moving orange sphere
    let centre = Pos3::new(400.0, 400.0, 200.0);
    let vel = Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_mat = Material::Lambertian {
        albedo: solid_colour(Colour::new(0.7, 0.3, 0.1)),
    };
    objects.push(
        Box::new(
            LinearMove::new(
                Sphere::new(
                    centre,
                    50.0,
                    moving_sphere_mat
                ),
                vel,
            )
        )
    );

    // Glass sphere 
    objects.push(
        Box::new(
            Sphere::new(
                Pos3::new(260.0, 150.0, 45.0),
                50.0,
                Material::Dielectric {
                    refractive_index: 1.5,
                },
            )
        )
    );

    // Metal sphere in back
    objects.push(
        Box::new(
            Sphere::new(
                Pos3::new(0.0, 150.0, 145.0),
                50.0,
                Material::Metal {
                    albedo: Colour::new(0.8, 0.8, 0.9),
                    fuzziness: 10.0,
                },
            )
        )
    );

    // Shiny blue sphere
    let boundary = Sphere::new(
        Pos3::new(360.0, 150.0, 145.0),
        70.0,
        Material::Dielectric {
            refractive_index: 1.5,
        },
    );
    objects.push(
        Box::new(
            boundary.clone()
        )
    );
    objects.push(
        Box::new(
            ConstantMedium::new(
                boundary.clone(),
                0.2,
                Material::Isotropic {
                    albedo: solid_colour(Colour::new(0.2, 0.4, 0.9))
                },
            )
        )
    );

    // Fog
    let boundary = Sphere::new(
        Pos3::from(0.0),
        5000.0,
        Material::Dielectric {
            refractive_index: 1.5,
        },
    );
    objects.push(
        Box::new(
            ConstantMedium::new(
                boundary,
                0.0001,
                Material::Lambertian {
                    albedo: solid_colour(Colour::from(1.0))
                },
            )
        )
    );

    // Earth sphere
    let earth_mat = Material::Lambertian {
        albedo: image(Path::new("assets/earthmap.jpg"))
    };
    objects.push(
        Box::new(
            Sphere::new(
                Pos3::new(400.0, 200.0, 400.0),
                100.0,
                earth_mat,
            )
        )
    );

    // Perlin sphere
    let perlin_mat = Material::Lambertian {
        albedo: noise(Perlin::new(), 0.1)
    };
    objects.push(
        Box::new(
            Sphere::new(
                Pos3::new(220.0, 280.0, 300.0),
                80.0,
                perlin_mat,
            )
        )
    );

    // Box of spheres
    let white = Material::Lambertian {
        albedo: solid_colour(Colour::from(0.73))
    };
    let num_spheres = 1000;
    let mut box_vec: Objects = Vec::with_capacity(num_spheres);
    for i in 0..num_spheres {
        box_vec.push(
            Box::new(
                Sphere::new(
                    random_vec_in_range(0.0, 165.0),
                    10.0,
                    white.clone()
                )
            )
        );
    }
    objects.push(
        Box::new(
            Translate::new(
                RotateY::new(
                    Bvh::new(
                        box_vec,
                        t_min,
                        t_max,
                    ),
                    15.0
                ),
                Vec3::new(-100.0, 270.0, 395.0),
            )
        )
    );

    let look_from = Pos3::new(478.0, 278.0, -600.0);
    let look_at = Pos3::new(278.0, 278.0, 0.0);
    let vup = Pos3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let fov = 40.0;

    let camera = Camera::new(
        look_from,
        look_at, 
        vup, 
        fov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (camera, objects)
}

pub fn cornell_box_smoke(t_min: f64, t_max: f64) -> Objects {
    let red =   Material::Lambertian { 
        albedo: solid_colour(Colour::new(0.65, 0.05, 0.05))
    };
    let white = Material::Lambertian { 
        albedo: solid_colour(Colour::from(0.73))
    };
    let black = Material::Lambertian { 
        albedo: solid_colour(Colour::from(0.0))
    };
    let green = Material::Lambertian { 
        albedo: solid_colour(Colour::new(0.12, 0.45, 0.15))
    };
    let light = Material::DiffuseLight {
        emit: solid_colour(Colour::from(15.0))
    };

    vec![
        Box::new(
            FlipNormals (
                YZRect::new(
                    0.0,
                    555.0,
                    0.0,
                    555.0,
                    555.0,
                    green.clone()
                )
            )
        ),
        Box::new(
            YZRect::new(
                0.0,
                555.0,
                0.0,
                555.0,
                0.0,
                red.clone()
            )
        ),
        Box::new(
            XZRect::new(
                213.0,
                343.0,
                227.0,
                332.0,
                554.0,
                light.clone()
            )
        ),
        Box::new(
            FlipNormals (
                XZRect::new(
                    0.0,
                    555.0,
                    0.0,
                    555.0,
                    0.0,
                    white.clone()
                )
            )
        ),
        Box::new(
            XZRect::new(
                0.0,
                555.0,
                0.0,
                555.0,
                555.0,
                white.clone()
            )
        ),
        Box::new(
            FlipNormals (
                XYRect::new(
                    0.0,
                    555.0,
                    0.0,
                    555.0,
                    555.0,
                    white.clone()
                )
            )
        ),

        Box::new(
            ConstantMedium::new(
                Translate::new(
                    RotateY::new(
                        Cuboid::new(
                            Pos3::new(0.0, 0.0, 0.0),
                            Pos3::new(165.0, 330.0, 165.0),
                            white.clone()
                        ),
                        15.0
                    ),
                    Vec3::new(265.0, 0.0, 295.0)
                ),
                0.01,
                white.clone(),
            )
        ),
        Box::new(
            ConstantMedium::new(
                Translate::new(
                    RotateY::new(
                        Cuboid::new(
                            Pos3::new(0.0, 0.0, 0.0),
                            Pos3::new(165.0, 165.0, 165.0),
                            white.clone()
                        ),
                        -18.0
                    ),
                    Vec3::new(130.0, 0.0, 65.0)
                ),
                0.01,
                black.clone(),
            )
        ),
    ]
}

pub fn cornell_box(t_min: f64, t_max: f64) -> Objects {
    let red =   Material::Lambertian { 
        albedo: solid_colour(Colour::new(0.65, 0.05, 0.05))
    };
    let white = Material::Lambertian { 
        albedo: solid_colour(Colour::from(0.73))
    };
    let green = Material::Lambertian { 
        albedo: solid_colour(Colour::new(0.12, 0.45, 0.15))
    };
    let light = Material::DiffuseLight {
        emit: solid_colour(Colour::from(15.0))
    };

    vec![
        Box::new(
            FlipNormals (
                YZRect::new(
                    0.0,
                    555.0,
                    0.0,
                    555.0,
                    555.0,
                    green.clone()
                )
            )
        ),
        Box::new(
            YZRect::new(
                0.0,
                555.0,
                0.0,
                555.0,
                0.0,
                red.clone()
            )
        ),
        Box::new(
            XZRect::new(
                213.0,
                343.0,
                227.0,
                332.0,
                554.0,
                light.clone()
            )
        ),
        Box::new(
            FlipNormals (
                XZRect::new(
                    0.0,
                    555.0,
                    0.0,
                    555.0,
                    0.0,
                    white.clone()
                )
            )
        ),
        Box::new(
            XZRect::new(
                0.0,
                555.0,
                0.0,
                555.0,
                555.0,
                white.clone()
            )
        ),
        Box::new(
            FlipNormals (
                XYRect::new(
                    0.0,
                    555.0,
                    0.0,
                    555.0,
                    555.0,
                    white.clone()
                )
            )
        ),

        Box::new(
            Translate::new(
                RotateY::new(
                    Cuboid::new(
                        Pos3::new(0.0, 0.0, 0.0),
                        Pos3::new(165.0, 330.0, 165.0),
                        white.clone()
                    ),
                    15.0
                ),
                Vec3::new(265.0, 0.0, 295.0)
            )
        ),
        Box::new(
            Translate::new(
                RotateY::new(
                    Cuboid::new(
                        Pos3::new(0.0, 0.0, 0.0),
                        Pos3::new(165.0, 165.0, 165.0),
                        white.clone()
                    ),
                    -18.0
                ),
                Vec3::new(130.0, 0.0, 65.0)
            )
        ),
    ]
}

pub fn rectangle_light_test(t_min: f64, t_max: f64) -> Objects {
    let perlin = noise(Perlin::new(), 4.0);

    vec![
        Box::new(
            Sphere {
                centre: Pos3::new(0.0, -1000.0, 0.0),
                material: Material::Lambertian {
                    albedo: perlin.clone(),
                },
                radius: 1000.0,
            }
        ),
        Box::new(
            Sphere {
                centre: Pos3::new(0.0, 2.0, 0.0),
                material: Material::Lambertian {
                    albedo: perlin.clone(),
                },
                radius: 2.0,
            }
        ),
        Box::new(
            XYRect {
                x0: 3.0,
                x1: 5.0,
                y0: 1.0,
                y1: 3.0,
                z: -2.0,
                material: Material::DiffuseLight {
                    emit: solid_colour(Colour::from(4.0))
                },
            }
        )
    ]
}

pub fn texture_test(t_min: f64, t_max: f64) -> Objects {
    let texture = image(Path::new("assets/earthmap.jpg"));

    vec![
        Box::new(
            Sphere {
                centre: Pos3::new(0.0, -1000.0, 0.0),
                material: Material::Lambertian {
                    albedo: solid_colour(Colour::from(0.6)),
                },
                radius: 1000.0,
            }
        ),
        Box::new(
            Sphere {
                centre: Pos3::new(0.0, 2.0, 0.0),
                material: Material::DiffuseLight {
                    emit: texture.clone(),
                },
                radius: 2.0,
            }
        ),
    ]
}

pub fn perlin_test(t_min: f64, t_max: f64) -> Objects {
    let perlin = noise(Perlin::new(), 4.0);

    vec![
        Box::new(
            Sphere {
                centre: Pos3::new(0.0, -1000.0, 0.0),
                material: Material::Lambertian {
                    albedo: perlin.clone(),
                },
                radius: 1000.0,
            }
        ),
        Box::new(
            Sphere {
                centre: Pos3::new(0.0, 2.0, 0.0),
                material: Material::Lambertian {
                    albedo: perlin.clone(),
                },
                radius: 2.0,
            }
        ),
    ]
}

pub fn final_scene_1(t_min: f64, t_max: f64) -> Objects {
    let mut objects: Objects = vec![];

    objects.push(
        Box::new(
            Sphere {
                centre: Pos3::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
                material: Material::Lambertian {
                    albedo: solid_colour(Colour::from(0.5)),
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
                                    albedo: solid_colour(albedo),
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
                    albedo: solid_colour(Colour::new(0.4, 0.2, 0.1)),
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
                    albedo: Colour::new(0.8, 0.7, 0.6),
                    fuzziness: 0.2,
                },
            }
        )
    );

    objects
}

pub fn test_bvh(t_min: f64, t_max: f64) -> (Camera, Objects) {
    let mut objects: Objects = vec![];

    objects.push(
        Box::new(
            Sphere {
                centre: Pos3::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
                material: Material::Lambertian {
                    albedo: checkered(solid_colour(Colour::from(0.5)), solid_colour(Colour::from(1.0))),
                },
            }
        )
    );
    objects.push({
        let mut objects: Vec<Box<dyn Hit>> = vec![];
        for a in -11..11 {
            for b in -11..11 {
                let mat = random_zero_one();
                let centre = Pos3::new(a as f64 + 0.9 * random_zero_one(), 0.2, b as f64 + 0.9 * random_zero_one());
                if (centre - Pos3::new(4.0, 0.2, 0.0)).length() > 0.8 {
                    if mat < 0.8 {
                        let albedo = solid_colour(rand_colour() * rand_colour());
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
        Box::new(Bvh::new(objects, t_min, t_max))
    });
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
                    albedo: solid_colour(Colour::new(0.4, 0.2, 0.1)),
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

    let camera = Camera::new(
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

    (camera, objects)
}

pub fn three_different_objects() -> Objects {
    let mut objects: Objects = vec![];

    objects.push(
        Box::new(
            Sphere {
                centre: Pos3::new(0.0, 0.0, -1.0), 
                radius: 0.5,
                material: Material::Lambertian {
                    albedo: solid_colour(Colour::new(0.1, 0.2, 0.5)),
                },
            }
        )
    );
    objects.push(
        Box::new(
            Sphere {
                centre: Pos3::new(0.0, -100.5, -1.0), 
                radius: 100.0,
                material: Material::Lambertian {
                    albedo: solid_colour(Colour::new(0.8, 0.8, 0.0)),
                },
            }
        )
    );
    objects.push(
        Box::new(
            Sphere {
                centre: Pos3::new(1.0, 0.0, -1.0), 
                radius: 0.5,
                material: Material::Metal {
                    albedo: Colour::new(0.8, 0.6, 0.2),
                    fuzziness: 0.0,
                },
            }
        )
    );
    objects.push(
        Box::new(
            Sphere {
                centre: Pos3::new(-1.0, 0.0, -1.0), 
                radius: 0.5,
                material: Material::Dielectric {
                    refractive_index: 1.5,
                },
            }
        )
    );
    objects.push(
        Box::new(
            Sphere {
                centre: Pos3::new(-1.0, 0.0, -1.0), 
                radius: -0.45,
                material: Material::Dielectric {
                    refractive_index: 1.5,
                },
            }
        )
    );

    objects
}

pub fn two_touching_objects() -> Objects {
    let mut objects: Objects = vec![];

    let r = (PI / 4.0).cos();
    debug_assert!(!r.is_nan());
    objects.push(
        Box::new(
            Sphere {
                centre: Pos3::new(-r, 0.0, -1.0), 
                radius: r,
                material: Material::Lambertian {
                    albedo: solid_colour(Colour::new(0.0, 0.0, 1.0)),
                },
            }
        )
    );
    objects.push(
        Box::new(
            Sphere {
                centre: Pos3::new(r, 0.0, -1.0), 
                radius: r,
                material: Material::Lambertian {
                    albedo: solid_colour(Colour::new(1.0, 0.0, 0.0)),
                },
            }
        )
    );

    objects
}