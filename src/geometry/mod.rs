use crate::material::*;

pub use ultraviolet::{
    geometry::{Aabb, Ray},
    vec::Vec3,
};

pub struct Hit<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a Box<dyn Material + Send + Sync>,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit>;
}

pub trait Bounded {
    fn bounds(&self) -> Aabb;
}
