use super::*;

#[derive(Clone)]
pub struct Sphere {
    pub centre: Pos3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(centre: Pos3, radius: f64, material: Material) -> Sphere {
        Sphere { centre, radius, material }
    }

    pub fn uv(p: Pos3) -> (f64, f64) {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        assert!(!phi.is_nan());
        assert!(!theta.is_nan());

        let u = 1.0 - (phi + PI)  / (2.0 * PI);
        let v = (theta + PI / 2.0) / PI;

        assert!(!u.is_nan() && !v.is_nan());

        (u, v)
    }
}

impl Hit for Sphere {
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let t = (-half_b - root) / a;
            assert!(!t.is_nan());
            if (t_min < t) && (t < t_max) {
                let p = ray.at(t);
                let normal = Vec3::normalize(&(p - self.centre));
                assert!(!normal.is_nan());
                let (normal, side) = HitRecord::face(&normal, ray);
                let material = &self.material;

                let (u, v) = Sphere::uv(Vec3::normalize(&(p - self.centre)));

                return Some(
                    HitRecord {
                        t, 
                        u,
                        v,
                        p,
                        normal,
                        side,
                        material,
                    }
                );
            } 
            let t = (-half_b + root) / a;
            assert!(!t.is_nan());
            if (t_min < t) && (t < t_max) {
                let p = ray.at(t);
                let normal = Vec3::normalize(&(p - self.centre));
                

                let (_, side) = HitRecord::face(&normal, &ray);

                let material = &self.material;

                let (u, v) = Sphere::uv(Vec3::normalize(&(p - self.centre)));

                return Some(
                    HitRecord {
                        t, 
                        u,
                        v,
                        p,
                        normal,
                        side,
                        material,
                    }
                );
            } 
        }
        None
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(
            Aabb::new(
                self.centre - Vec3::from(self.radius),
                self.centre + Vec3::from(self.radius)
            )
        )
    }
}