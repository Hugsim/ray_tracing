use super::*;
use self::rect::*;
use self::transforms::*;

pub struct Cuboid {
    pub c0: Pos3,
    pub c1: Pos3,
    sides: Objects,
}

impl Cuboid {
    pub fn new(c0: Pos3, c1: Pos3, material: Material) -> Cuboid {
        let mut sides: Vec<Box<dyn Hit>> = Vec::with_capacity(6);

        sides.push(
            Box::new(
                XYRect::new(
                    c0.x,
                    c1.x,
                    c0.y,
                    c1.y,
                    c1.z,
                    material.clone(),
                )
            )
        );
        sides.push(
            Box::new(
                FlipNormals (
                    XYRect::new(
                        c0.x,
                        c1.x,
                        c0.y,
                        c1.y,
                        c0.z,
                        material.clone(),
                    )
                )
            )
        );

        sides.push(
            Box::new(
                XZRect::new(
                    c0.x,
                    c1.x,
                    c0.z,
                    c1.z,
                    c1.y,
                    material.clone(),
                )
            )
        );
        sides.push(
            Box::new(
                FlipNormals (
                    XZRect::new(
                        c0.x,
                        c1.x,
                        c0.z,
                        c1.z,
                        c0.y,
                        material.clone(),
                    )
                )
            )
        );

        sides.push(
            Box::new(
                YZRect::new(
                    c0.y,
                    c1.y,
                    c0.z,
                    c1.z,
                    c1.x,
                    material.clone(),
                )
            )
        );
        sides.push(
            Box::new(
                FlipNormals (
                    YZRect::new(
                        c0.y,
                        c1.y,
                        c0.z,
                        c1.z,
                        c0.x,
                        material.clone(),
                    )
                )
            )
        );

        Cuboid {
            c0,
            c1,
            sides,
        }
    }
}

impl Hit for Cuboid {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        Some(
            Aabb::new(
                self.c0,
                self.c1,
            )
        )
    }
}