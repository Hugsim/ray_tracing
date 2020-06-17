use crate::ray::*;
use crate::colour::*;
use crate::hit::*;
use crate::vec3::*;
use crate::utility::*;
use crate::texture::*;

#[derive(Clone)]
pub enum Material {
    Lambertian {
        albedo: Texture,
    },
    Metal {
        albedo: Colour,
        fuzziness: f64,
    },
    Dielectric {
        refractive_index: f64,
    },
    DiffuseLight {
        emit: Texture,
    },
    Isotropic {
        albedo: Texture,
    },
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<(Ray, Colour)> {
        match self {
            Material::Lambertian { albedo } => {
                let scatter_direction = hr.normal + random_unit_vec();
                let scattered = Ray::new(hr.p, scatter_direction, ray.time);
                let attenuation = albedo(hr.u, hr.v, hr.p);

                Some((scattered, attenuation))
            },

            Material::Metal { albedo, fuzziness } => {
                let reflected = ray.direction.reflect(hr.normal);
                let scattered = Ray::new(hr.p, reflected + *fuzziness * random_vec_in_unit_sphere(), ray.time);

                Some((scattered, *albedo))
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
            Material::DiffuseLight { .. } => {
                None
            },
            Material::Isotropic { albedo } => {
                let ray = Ray::new(
                    hr.p, 
                    random_vec_in_unit_sphere(),
                    ray.time
                );
                let colour = albedo(hr.u, hr.v, hr.p);
                
                Some((ray, colour))
            },
        }
    }

    pub fn emit(&self, u: f64, v: f64, p: Pos3) -> Colour {
        match self {
            Material::DiffuseLight { emit } => {
                emit(u, v, p)
            },
            _ => {
                Colour::BLACK
            }
        }
    }
}