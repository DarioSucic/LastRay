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
