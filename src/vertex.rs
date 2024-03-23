use nalgebra_glm::{Vec2, Vec3};

use crate::color::Color;

pub trait Vertex: Sized {
    fn fragment_color(vertices: &[Self; 3], uvw: Vec3) -> Color;

    fn coords(&self) -> &Vec2;
}

#[derive(Debug, Clone)]
pub struct BasicVertex2D {
    pub coords: Vec2,
    pub color: Color,
}

impl BasicVertex2D {
    pub fn new(coords: Vec2, color: Color) -> Self {
        Self { coords, color }
    }
}

impl Vertex for BasicVertex2D {
    #[inline]
    fn fragment_color([v0, v1, v2]: &[Self; 3], uvw: Vec3) -> Color {
        uvw.x * v0.color + uvw.y * v1.color + uvw.z * v2.color
    }

    #[inline]
    fn coords(&self) -> &Vec2 {
        &self.coords
    }
}
