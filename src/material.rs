use crate::ray::*;
use crate::colour::*;
use crate::hit::*;
use crate::vec3::*;
use crate::utility::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Material {
    Lambertian {
        albedo: Colour,
    },
    Metal {
        albedo: Colour,
        fuzziness: f64,
    },
    Dielectric {
        refractive_index: f64,
    }
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<(Ray, Colour)> {
        match self {
            Material::Lambertian { albedo } => {
                let scatter_direction = hr.normal + random_unit_vec();
                let scattered = Ray::new(hr.p, scatter_direction, ray.time);
                let attenuation = albedo;

                Some((scattered, *attenuation))
            },

            Material::Metal { albedo, fuzziness } => {
                let reflected = ray.direction.reflect(hr.normal);
                let scattered = Ray::new(hr.p, reflected + *fuzziness * random_vec_in_unit_sphere(), ray.time);
                let attenuation = albedo;

                Some((scattered, *attenuation))
            },

            Material::Dielectric { refractive_index } => {
                let (normal, eta_over_eta) = match hr.side {
                    Side::Outside => (hr.normal, 1.0 / refractive_index),
                    Side::Inside => (-hr.normal, *refractive_index),
                };

                let cos_theta = min(Vec3::dot(&-Vec3::normalize(&ray.direction), &normal), 1.0);
                
                let ray = if schlick(cos_theta, eta_over_eta) > random_zero_one() {
                    Ray::new(hr.p, Vec3::normalize(&ray.direction).reflect(normal), ray.time)
                } else if let Some(refracted) = ray.direction.refract(eta_over_eta, normal) {
                    Ray::new(hr.p, refracted, ray.time)
                } else {
                    Ray::new(hr.p, Vec3::normalize(&ray.direction).reflect(normal), ray.time)
                };

                Some((ray, Colour::from(1.0)))
            },
        }
    }
}