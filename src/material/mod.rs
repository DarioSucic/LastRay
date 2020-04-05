pub use crate::*;

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::*;
pub use lambertian::*;
pub use metal::*;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<(Vec3, Ray)>;
}

impl dyn Material {
    pub fn from_tobj(material: tobj::Material) -> Box<dyn Material + Send + Sync> {
        let ka = material.ambient;
        let kd = material.diffuse;
        let fuzziness = 1.0 - material.shininess / 100.0;
        let refractive_index = material.optical_density;

        match material.illumination_model {
            Some(T) => match T {
                0..=2 => Box::new(Lambertian { albedo: kd.into() }),
                4 => Box::new(Dielectric {
                    albedo: kd.into(),
                    fuzziness,
                    refractive_index,
                }),
                _ => Box::new(Metal {
                    albedo: kd.into(),
                    fuzziness,
                }),
            },
            _ => Box::new(Lambertian { albedo: kd.into() }),
        }
    }
}
