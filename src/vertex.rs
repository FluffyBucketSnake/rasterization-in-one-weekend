use nalgebra_glm::{Vec2, Vec3};

use crate::color::Color;

#[derive(Debug, Clone)]
pub struct BasicVertex2D(pub Vec2, pub Color);

#[derive(Debug, Clone)]
pub struct BasicVertex3D(pub Vec3, pub Color);
