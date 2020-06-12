use super::*;

pub struct XYRect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub z: f64,
    pub material: Material,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, z: f64, material: Material) -> XYRect {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            z,
            material,
        }
    }
}


impl Hit for XYRect {
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.z - ray.origin.z) / (ray.direction.z);
        if t < t_min || t > t_max {
            return None;
        }
        let hit_point = ray.at(t);
        let x = hit_point.x;
        let y = hit_point.y;

        if self.x0 < x && x < self.x1 && self.y0 < y && y < self.y1 {
            let normal = Vec3::new(0.0, 0.0, 1.0);
            let (normal, side) = HitRecord::face(&normal, ray);
            let u = (x - self.x0) / (self.x1 - self.x0);
            let v = (y - self.y0) / (self.y1 - self.y0);
            let p = Pos3::new(x, y, self.z);

            Some (
                HitRecord {
                    material: &self.material,
                    normal,
                    t,
                    u,
                    v,
                    p,  
                    side,
                }
            )
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(
            Aabb::new(
                Pos3::new(self.x0, self.y0, self.z - 0.001), 
                Pos3::new(self.x1, self.y1, self.z + 0.001)
            )
        )
    }
}

pub struct XZRect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub y: f64,
    pub material: Material,
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, y: f64, material: Material) -> XZRect {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            y,
            material,
        }
    }
}

impl Hit for XZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.y - ray.origin.y) / (ray.direction.y);
        if t < t_min || t > t_max {
            return None;
        }
        let hit_point = ray.at(t);
        let x = hit_point.x;
        let z = hit_point.z;

        if self.x0 < x && x < self.x1 && self.z0 < z && z < self.z1 {
            let normal = Vec3::new(0.0, 1.0, 0.0);
            let (normal, side) = HitRecord::face(&normal, ray);
            Some (
                HitRecord {
                    material: &self.material,
                    normal,
                    p: Pos3::new(x, self.y, z),
                    t,
                    u: (x - self.x0) / (self.x1 - self.x0),
                    v: (z - self.z0) / (self.z1 - self.z0),
                    side,
                }
            )
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(
            Aabb::new(
                Pos3::new(self.x0, self.y - 0.001, self.z0), 
                Pos3::new(self.x1, self.y + 0.001, self.z1)
            )
        )
    }
}

pub struct YZRect {
    pub z0: f64,
    pub z1: f64,
    pub y0: f64,
    pub y1: f64,
    pub x: f64,
    pub material: Material,
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, x: f64, material: Material) -> YZRect {
        YZRect {
            z0,
            z1,
            y0,
            y1,
            x,
            material,
        }
    }
}

impl Hit for YZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.x - ray.origin.x) / (ray.direction.x);
        if t < t_min || t > t_max {
            return None;
        }
        let hit_point = ray.at(t);
        let z = hit_point.z;
        let y = hit_point.y;

        if self.z0 < z && z < self.z1 && self.y0 < y && y < self.y1 {
            let normal = Vec3::new(1.0, 0.0, 0.0);
            let (normal, side) = HitRecord::face(&normal, ray);
            Some (
                HitRecord {
                    material: &self.material,
                    normal,
                    p: Pos3::new(self.x, y, z),
                    t,
                    v: (z - self.z0) / (self.z1 - self.z0),
                    u: (y - self.y0) / (self.y1 - self.y0),
                    side,
                }
            )
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(
            Aabb::new(
                Pos3::new(self.x - 0.001, self.y0, self.z0), 
                Pos3::new(self.x + 0.001, self.y1, self.z1)
            )
        )
    }
}