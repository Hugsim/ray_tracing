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
                let scattered = Ray::new(hr.p, scatter_direction);
                let attenuation = albedo;

                Some((scattered, *attenuation))
            },

            Material::Metal { albedo, fuzziness } => {
                let reflected = ray.direction.reflect(hr.normal);
                let scattered = Ray::new(hr.p, reflected + *fuzziness * random_vec_in_unit_sphere());
                let attenuation = albedo;

                Some((scattered, *attenuation))
            },

            Material::Dielectric { refractive_index } => {
                // let eta_over_eta = match hr.side {
                //     Side::Outside => {
                //         1.0 / reflective_index
                //     },
                //     Side::Inside => {
                //         *reflective_index
                //     }
                // };
                // let unit_direction = Vec3::normalize(&ray.direction);

                // let cos_theta = min(Vec3::dot(&-unit_direction, &hr.normal), 1.0);
                // let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                
                // let vec = if (eta_over_eta * sin_theta > 1.0) || (schlick(cos_theta, eta_over_eta) > random_zero_one()) {
                //     unit_direction.reflect(hr.normal)
                // } else {
                //     Vec3::normalize(&unit_direction).refract(eta_over_eta, Vec3::normalize(&hr.normal))
                // };

                // Some((Ray::new(hr.p, vec), Colour::new(1.0, 1.0, 1.0)))

                let (normal, eta_over_eta) = match hr.side {
                    Side::Outside => (hr.normal, 1.0 / refractive_index),
                    Side::Inside => (-hr.normal, *refractive_index),
                };

                let cos_theta = min(Vec3::dot(&-Vec3::normalize(&ray.direction), &normal), 1.0);
                
                let ray = if schlick(cos_theta, eta_over_eta) > random_zero_one() {
                    Ray::new(hr.p, Vec3::normalize(&ray.direction).reflect(normal))
                } else if let Some(refracted) = ray.direction.refract(eta_over_eta, normal) {
                    Ray::new(hr.p, refracted)
                } else {
                    Ray::new(hr.p, Vec3::normalize(&ray.direction).reflect(normal))
                };

                Some((ray, Colour::from(1.0)))
            },
        }
    }
}