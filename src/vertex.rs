use nalgebra_glm::{Vec2, Vec3, Vec4};

use crate::color::Color;

pub trait Vertex: Sized {
    fn color(&self) -> Color;
    fn coords(&self) -> Vec4;
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
    fn color(&self) -> Color {
        self.color
    }

    #[inline]
    fn coords(&self) -> Vec4 {
        self.coords.push(0.0).push(1.0)
    }
}

#[derive(Debug, Clone)]
pub struct BasicVertex3D {
    pub coords: Vec3,
    pub color: Color,
}

impl BasicVertex3D {
    pub fn new(coords: Vec3, color: Color) -> Self {
        Self { coords, color }
    }
}

impl Vertex for BasicVertex3D {
    #[inline]
    fn color(&self) -> Color {
        self.color
    }

    #[inline]
    fn coords(&self) -> Vec4 {
        self.coords.push(1.0)
    }
}
