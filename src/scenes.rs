use crate::hit::*;
use crate::material::*;
use crate::vec3::*;
use crate::colour::*;
use crate::utility::*;
use crate::bvh::*;
use crate::texture::*;

pub type Objects = Vec<Box<dyn Hit>>;

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

pub fn test_bvh(t_min: f64, t_max: f64) -> Objects {
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
                                        albedo: albedo,
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
                                        albedo: albedo,
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

    objects
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