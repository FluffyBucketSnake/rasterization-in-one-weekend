use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};

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

    pub fn bary_lerp(&self, v1: &Self, v2: &Self, uvw: Vec3) -> Self {
        let v0 = self;
        Self {
            coords: uvw.x * v0.coords + uvw.y * v1.coords + uvw.z * v2.coords,
            color: uvw.x * v0.color + uvw.y * v1.color + uvw.z * v2.color,
            uv: uvw.x * v0.uv + uvw.y * v1.uv + uvw.z * v2.uv,
        }
    }

    pub fn homogenize(mut self) -> Self {
        self.coords /= self.coords.w;
        self
    }
}
