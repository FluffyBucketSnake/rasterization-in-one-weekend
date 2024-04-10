use nalgebra_glm::{Mat4, Vec3, Vec4};

use crate::{
    color::Color,
    vertex::{BasicVertex2D, BasicVertex3D},
};

pub trait Fragment: Sized {
    fn interpolate(fragments: &[Self; 3], w: Vec3) -> Self;
}

pub trait VertexShader<In, U> {
    type Output: Fragment;

    fn vs(&self, input: &In, uniforms: &U) -> (Vec4, Self::Output);
}

pub trait FragmentShader<In, U> {
    fn fs(&self, fragment: &In, uniforms: &U) -> Color;
}

impl Fragment for Color {
    fn interpolate(fragments: &[Self; 3], w: Vec3) -> Self {
        return fragments[0] * w.x + fragments[1] * w.y + fragments[2] * w.z;
    }
}

pub struct BasicUniform {
    pub transform: Mat4,
}

pub struct BasicVertexShader;
pub struct BasicFragmentShader;

impl VertexShader<BasicVertex2D, BasicUniform> for BasicVertexShader {
    type Output = Color;

    fn vs(
        &self,
        &BasicVertex2D(coords, color): &BasicVertex2D,
        uniforms: &BasicUniform,
    ) -> (Vec4, Self::Output) {
        (uniforms.transform * coords.push(0.0).push(1.0), color)
    }
}

impl VertexShader<BasicVertex3D, BasicUniform> for BasicVertexShader {
    type Output = Color;

    fn vs(
        &self,
        &BasicVertex3D(coords, color): &BasicVertex3D,
        uniforms: &BasicUniform,
    ) -> (Vec4, Self::Output) {
        (uniforms.transform * coords.push(1.0), color)
    }
}

impl FragmentShader<Color, BasicUniform> for BasicFragmentShader {
    fn fs(&self, &fragment: &Color, _: &BasicUniform) -> Color {
        fragment
    }
}
