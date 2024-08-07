use std::ops::{Div, Mul};

use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};
use simba::scalar::{ClosedAdd, ClosedMul};

use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub coords: Vec4,
    pub color: Color,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(coords: Vec3, color: Color, uv: Vec2) -> Self {
        Self {
            coords: coords.push(1.0),
            color,
            uv,
        }
    }

    pub fn transform(mut self, transform: &Mat4) -> Self {
        self.coords = transform * self.coords;
        self
    }

    pub fn lerp(&self, y: &Self, a: f32) -> Self {
        use nalgebra_glm::lerp;
        Self {
            coords: lerp(&self.coords, &y.coords, a),
            color: lerp(&self.color, &y.color, a),
            uv: lerp(&self.uv, &y.uv, a),
        }
    }

    pub fn bary_lerp(&self, v1: &Self, v2: &Self, t: Vec3) -> Self {
        let v0 = self;
        let coords = bary_lerp(v0.coords, v1.coords, v2.coords, t);
        let w0 = v0.coords.w;
        let w1 = v1.coords.w;
        let w2 = v2.coords.w;
        let w_t = coords.w;
        Self {
            coords,
            color: bary_lerp_perp(v0.color, w0, v1.color, w1, v2.color, w2, t, w_t),
            uv: bary_lerp_perp(v0.uv, w0, v1.uv, w1, v2.uv, w2, t, w_t),
        }
    }

    pub fn duv(&self, v1: &Self, v2: &Self, t: Vec3, dt: Vec3) -> Vec2 {
        let v0 = self;
        let w0 = v0.coords.w;
        let w1 = v1.coords.w;
        let w2 = v2.coords.w;
        let w_t0 = bary_lerp(v0.coords.w, v1.coords.w, v2.coords.w, t);
        let w_t1 = bary_lerp(v0.coords.w, v1.coords.w, v2.coords.w, t + dt);
        return bary_lerp_perp(v0.uv, w0, v1.uv, w1, v2.uv, w2, t + dt, w_t1)
            - bary_lerp_perp(v0.uv, w0, v1.uv, w1, v2.uv, w2, t, w_t0);
    }

    pub fn homogenize(mut self) -> Self {
        let w_inv = 1.0 / self.coords.w;
        self.coords *= w_inv;
        self.coords.w = w_inv;
        self
    }
}

#[inline]
fn bary_lerp<T>(v0: T, v1: T, v2: T, t: Vec3) -> T
where
    f32: Mul<T, Output = T>,
    T: ClosedAdd,
{
    t.x * v0 + t.y * v1 + t.z * v2
}

#[inline]
fn bary_lerp_perp<T>(v0: T, w0: f32, v1: T, w1: f32, v2: T, w2: f32, t: Vec3, w_t: f32) -> T
where
    f32: Mul<T, Output = T> + ClosedMul,
    T: ClosedAdd + Div<f32, Output = T>,
{
    bary_lerp(w0 * v0, w1 * v1, w2 * v2, t) / w_t
}
