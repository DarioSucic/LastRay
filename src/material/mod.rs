pub use crate::*;

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::*;
pub use lambertian::*;
pub use metal::*;


// TODO: Rename to something more descriptive
pub enum MaterialType {
    Emitter(Box<dyn Material + Send + Sync>),
    Receiver(Box<dyn Material + Send + Sync>),
}
pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<(Vec3, Ray)>;
}

impl dyn Material {
    pub fn from_tobj(material: tobj::Material) -> MaterialType {
        let ka = material.ambient;
        let kd = material.diffuse;
        let fuzziness = 1.0 - material.shininess / 255.0;
        let refractive_index = material.optical_density;

        if ka.iter().sum::<f32>() > 0.0 {
            MaterialType::Emitter(Box::new(Lambertian { albedo: ka.into() }))
        } else {
            MaterialType::Receiver(match material.illumination_model {
                Some(mat_type) => match mat_type {
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
            })
        }
    }
}
