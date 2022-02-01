use crate::*;
#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit, rng: &mut Xoshiro256PlusPlus) -> Option<(Vec3, Ray)> {
        let sample = rng.gen::<(f32, f32)>();
        let direction = hit.normal + uniform_sample_sphere(&sample);
        let inv_direction = Vec3::one() / direction;
    
        let scattered = Ray {
            origin: hit.point,
            direction,
            inv_direction
        };

        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}
