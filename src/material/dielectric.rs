use crate::*;
#[derive(Debug)]
pub struct Dielectric {
    pub albedo: Vec3,
    pub fuzziness: f32,
    pub refractive_index: f32,
}

impl Dielectric {
    fn refract(uv: Vec3, n: Vec3, refractive_ratio: f32) -> Vec3 {
        let cos_theta = -uv.dot(n);
        let r_out_parallel = refractive_ratio * (uv + cos_theta * n);
        let r_out_perp = -(1.0 - r_out_parallel.mag_sq()).sqrt() * n;
        r_out_parallel + r_out_perp
    }

    fn shlick(&self, cosine: f32, refractive_ratio: f32) -> f32 {
        let r0 = (1.0 - refractive_ratio) / (1.0 + refractive_ratio);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut WyRand) -> Option<(Vec3, Ray)> {
        let refractive_ratio = if hit.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_dir = ray.direction.normalized();
        let cos_theta = (-unit_dir).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let reflect_prob = self.shlick(cos_theta, refractive_ratio);

        let direction;

        if refractive_ratio * sin_theta > 0.999 || rng.generate::<f32>() < reflect_prob {
            let reflected = unit_dir.reflected(hit.normal);
            direction = reflected;
        } else {
            let refracted = Dielectric::refract(unit_dir, hit.normal, refractive_ratio);
            direction = refracted;
        };

        let sample = (rng.generate::<f32>(), rng.generate::<f32>());
        let sphere_sample = uniform_sample_sphere(&sample);

        let direction = direction + sphere_sample * self.fuzziness;

        let inv_direction = Vec3::one() / direction;

        let scattered = Ray {
            origin: hit.point,
            direction: direction,
            inv_direction
        };

        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}
