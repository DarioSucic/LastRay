use crate::*;
pub struct Sphere<'a> {
    pub pos: Vec3,
    pub radius: f32,
    pub material: &'a MaterialType,
}

impl<'a> Intersectable for Sphere<'a> {
    fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit> {
        let oc = ray.origin - self.pos;

        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_root = discriminant.sqrt();

        let t = (-b - discriminant_root) / a;
        if t < tmax && t > tmin {
            let point = ray.at_distance(t);
            let out_normal = (point - self.pos) / self.radius;

            let front_face = ray.direction.dot(out_normal) < 0.0;
            let normal = if front_face { out_normal } else { -out_normal };

            return Some(Hit {
                point,
                normal,
                t,
                front_face,
                material: self.material,
            });
        }

        let t = (-b + discriminant_root) / a;
        if t < tmax && t > tmin {
            let point = ray.at_distance(t);
            let out_normal = (point - self.pos) / self.radius;

            let front_face = ray.direction.dot(out_normal) < 0.0;
            let normal = if front_face { out_normal } else { -out_normal };

            return Some(Hit {
                point,
                normal,
                t,
                front_face,
                material: self.material,
            });
        }

        None
    }
}
