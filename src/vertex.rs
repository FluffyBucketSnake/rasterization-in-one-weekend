use nalgebra_glm::{Mat4, Vec3, Vec4};

use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub coords: Vec4,
    pub color: Color,
}

impl Vertex {
    pub fn new(coords: Vec3, color: Color) -> Self {
        Self {
            coords: coords.push(1.0),
            color,
        }
    }

    pub fn transform(mut self, transform: &Mat4) -> Self {
        self.coords = transform * self.coords;
        self
    }

    pub fn lerp(&self, y: &Self, a: f32) -> Self {
        Self {
            coords: nalgebra_glm::lerp(&self.coords, &y.coords, a),
            color: nalgebra_glm::lerp(&self.color, &y.color, a),
        }
    }

    pub fn homogenize(mut self) -> Self {
        self.coords /= self.coords.w;
        self
    }
}
