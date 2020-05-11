use crate::*;

#[derive(Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzziness: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<(Vec3, Ray)> {
        let sample = rng.gen::<(f32, f32)>();
        let sphere_sample = uniform_sample_sphere(&sample);

        let reflected = ray.direction.reflected(hit.normal) + self.fuzziness * sphere_sample;

        if reflected.dot(hit.normal) <= 0.0 {
            return None;
        }

        let inv_direction = Vec3::one() / reflected;

        let scattered = Ray {
            origin: hit.point,
            direction: reflected,
            inv_direction
        };

        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}
