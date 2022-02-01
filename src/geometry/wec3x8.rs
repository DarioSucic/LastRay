use std::simd::{f32x8, mask32x8};

use std::arch::x86_64::*;
use std::hint::unreachable_unchecked;
use std::mem;
use std::ops::*;

use crate::*;

// Tiny AVX wide vector implementation for intersection testing

pub fn merge_packed(mask: mask32x8, a: f32x8, b: f32x8) -> f32x8 {
    unsafe {
        mem::transmute(_mm256_blendv_ps(
            mem::transmute(b),
            mem::transmute(a),
            mem::transmute(mask),
        ))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Wec3x8 {
    pub x: f32x8,
    pub y: f32x8,
    pub z: f32x8,
}

impl Wec3x8 {
    #[inline]
    pub fn new(x: f32x8, y: f32x8, z: f32x8) -> Wec3x8 {
        Wec3x8 { x, y, z }
    }

    #[inline]
    pub fn broadcast(v: f32x8) -> Wec3x8 {
        Wec3x8::new(v, v, v)
    }

    #[inline]
    pub fn splat(v: Vec3) -> Wec3x8 {
        Wec3x8::new(
            f32x8::splat(v.x),
            f32x8::splat(v.y),
            f32x8::splat(v.z),   
        )
    }

    #[inline]
    pub fn zero() -> Wec3x8 {
        Wec3x8::broadcast(f32x8::splat(0.0))
    }

    #[inline]
    pub fn dot(&self, rhs: Wec3x8) -> f32x8 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn cross(&self, rhs: Wec3x8) -> Wec3x8 {
        Wec3x8::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    #[inline]
    pub fn merge(mask: mask32x8, tru: Wec3x8, fls: Wec3x8) -> Wec3x8 {
        Wec3x8::new(
            merge_packed(mask, tru.x, fls.x),
            merge_packed(mask, tru.y, fls.y),
            merge_packed(mask, tru.z, fls.z),
        )
    }

    #[inline]
    pub fn componentwise_min(&self) -> Vec3 {
        Vec3::new(
            self.x.horizontal_min(),
            self.y.horizontal_min(),
            self.z.horizontal_min()
        )
    }

    #[inline]
    pub fn componentwise_max(&self) -> Vec3 {
        Vec3::new(
            self.x.horizontal_max(),
            self.y.horizontal_max(),
            self.z.horizontal_max()
        )
    }
}

impl Add for Wec3x8 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Wec3x8) -> Self {
        Wec3x8::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Wec3x8 {
    #[inline]
    fn add_assign(&mut self, rhs: Wec3x8) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Wec3x8 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Wec3x8) -> Self {
        Wec3x8::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Wec3x8 {
    #[inline]
    fn sub_assign(&mut self, rhs: Wec3x8) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Wec3x8 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Wec3x8) -> Self {
        Wec3x8::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<Wec3x8> for f32x8 {
    type Output = Wec3x8;
    #[inline]
    fn mul(self, rhs: Wec3x8) -> Wec3x8 {
        Wec3x8::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Mul<f32x8> for Wec3x8 {
    type Output = Wec3x8;
    #[inline]
    fn mul(self, rhs: f32x8) -> Wec3x8 {
        Wec3x8::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign for Wec3x8 {
    #[inline]
    fn mul_assign(&mut self, rhs: Wec3x8) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f32x8> for Wec3x8 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32x8) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div for Wec3x8 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Wec3x8) -> Self {
        Wec3x8::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Div<f32x8> for Wec3x8 {
    type Output = Wec3x8;
    #[inline]
    fn div(self, rhs: f32x8) -> Wec3x8 {
        Wec3x8::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign for Wec3x8 {
    #[inline]
    fn div_assign(&mut self, rhs: Wec3x8) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl DivAssign<f32x8> for Wec3x8 {
    #[inline]
    fn div_assign(&mut self, rhs: f32x8) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Wec3x8 {
    type Output = Wec3x8;
    #[inline]
    fn neg(self) -> Wec3x8 {
        self * f32x8::splat(-1.0)
    }
}

impl Index<usize> for Wec3x8 {
    type Output = f32x8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl IndexMut<usize> for Wec3x8 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}
