use crate::geometry::*;

pub struct Slow<T: Intersectable> {
    pub objects: Vec<T>,
}

impl<T: Intersectable> Intersectable for Slow<T> {
    fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit> {
        self.objects
            .iter()
            .filter_map(|o| o.intersect(&ray, tmin, tmax))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}

// (W * H) * S * O 