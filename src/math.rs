use std::ops::{Div, Mul};

use nalgebra_glm::Vec3;
use simba::scalar::{ClosedAdd, ClosedMul};

#[inline]
pub fn bary_lerp<T>(v0: T, v1: T, v2: T, t: Vec3) -> T
where
    f32: Mul<T, Output = T>,
    T: ClosedAdd,
{
    t.x * v0 + t.y * v1 + t.z * v2
}

#[inline]
pub fn bary_lerp_perp<T>(v0: T, w0: f32, v1: T, w1: f32, v2: T, w2: f32, t: Vec3, w_t: f32) -> T
where
    f32: Mul<T, Output = T> + ClosedMul,
    T: ClosedAdd + Div<f32, Output = T>,
{
    bary_lerp(w0 * v0, w1 * v1, w2 * v2, t) / w_t
}
