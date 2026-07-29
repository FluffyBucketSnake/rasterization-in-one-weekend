use std::fmt::Debug;

use nalgebra_glm::{dot, lerp, Mat3, Mat4, Vec2, Vec3, Vec4};

use crate::{
    color::{self, Color},
    image::Image,
    math::{bary_lerp, bary_lerp_perp},
    rasterization::Fragment,
    sampler::Sampler,
};

pub trait FragmentData: Copy {
    type Derivative: Copy;

    fn lerp(&self, other: &Self, a: f32) -> Self;
    fn bary_lerp(&self, w0: f32, f1: &Self, w1: f32, f2: &Self, w2: f32, t: Vec3) -> Self;
    fn derivate(
        &self,
        w0: f32,
        f1: &Self,
        w1: f32,
        f2: &Self,
        w2: f32,
        t: Vec3,
        dt: Vec3,
    ) -> Self::Derivative;
}

pub trait VertexShader<U, V>: Debug {
    type Fragment: FragmentData;

    fn vs(&self, uniforms: &U, vertex: &V) -> (Vec4, Self::Fragment);
}

pub trait FragmentShader<U, F: FragmentData>: Debug {
    fn fs(&self, uniforms: &U, fragment: &Fragment, data: &F, d_data: &[F::Derivative; 2])
        -> Color;
}

pub struct Environment {
    pub ambient_color: Color,
}

pub struct PointLight {
    pub pos: Vec3,
    pub color: Color,
    pub power: f32,
}

pub struct PhongMaterial {
    pub diffuse_color: Color,
    pub specular_color: Color,
    pub specular_exp: f32,
    pub texture: (Image<u32>, Sampler),
}

pub struct BasicUniforms {
    pub world: Mat4,
    pub view: Mat4,
    pub proj: Mat4,
    pub normal_matrix: Mat3,
    pub light: PointLight,
    pub material: PhongMaterial,
    pub env: Environment,
}

#[derive(Debug, Clone, Copy)]
pub struct BasicVertex {
    pub coords: Vec3,
    pub color: Color,
    pub uv: Vec2,
    pub normal: Vec3,
}

#[derive(Debug, Clone, Copy)]
pub struct BasicFragmentData {
    pub view_coords: Vec3,
    pub color: Color,
    pub uv: Vec2,
    pub normal: Vec3,
}

impl FragmentData for BasicFragmentData {
    type Derivative = DBasicFragmentData;

    fn lerp(&self, other: &Self, a: f32) -> Self {
        Self {
            view_coords: lerp(&self.view_coords, &other.view_coords, a),
            color: lerp(&self.color, &other.color, a),
            normal: lerp(&self.normal, &other.normal, a),
            uv: lerp(&self.uv, &other.uv, a),
        }
    }

    fn bary_lerp(&self, w0: f32, f1: &Self, w1: f32, f2: &Self, w2: f32, t: Vec3) -> Self {
        let f0 = self;
        let w_t = bary_lerp(w0, w1, w2, t);
        Self {
            view_coords: bary_lerp(f0.view_coords, f1.view_coords, f2.view_coords, t),
            normal: bary_lerp(f0.normal, f1.normal, f2.normal, t),
            color: bary_lerp_perp(f0.color, w0, f1.color, w1, f2.color, w2, t, w_t),
            uv: bary_lerp_perp(f0.uv, w0, f1.uv, w1, f2.uv, w2, t, w_t),
        }
    }

    fn derivate(
        &self,
        w0: f32,
        f1: &Self,
        w1: f32,
        f2: &Self,
        w2: f32,
        t: Vec3,
        dt: Vec3,
    ) -> Self::Derivative {
        let f0 = self;
        let w_t0 = bary_lerp(w0, w1, w2, t);
        let w_t1 = bary_lerp(w0, w1, w2, t + dt);

        DBasicFragmentData {
            uv: bary_lerp_perp(f0.uv, w0, f1.uv, w1, f2.uv, w2, t + dt, w_t1)
                - bary_lerp_perp(f0.uv, w0, f1.uv, w1, f2.uv, w2, t, w_t0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DBasicFragmentData {
    pub uv: Vec2,
}

#[derive(Debug, Default)]
pub struct BasicVertexShader;

impl VertexShader<BasicUniforms, BasicVertex> for BasicVertexShader {
    type Fragment = BasicFragmentData;

    fn vs(&self, uniforms: &BasicUniforms, input: &BasicVertex) -> (Vec4, Self::Fragment) {
        let view_coords = uniforms.view * uniforms.world * input.coords.push(1.0);

        (
            uniforms.proj * view_coords,
            BasicFragmentData {
                view_coords: view_coords.xyz(),
                normal: uniforms.normal_matrix * input.normal.normalize(),
                color: input.color,
                uv: input.uv,
            },
        )
    }
}

#[derive(Debug, Default)]
pub struct BasicFragmentShader;

impl FragmentShader<BasicUniforms, BasicFragmentData> for BasicFragmentShader {
    fn fs(
        &self,
        uniforms: &BasicUniforms,
        _: &Fragment,
        data: &BasicFragmentData,
        d_data: &[DBasicFragmentData; 2],
    ) -> Color {
        let p = data.view_coords;
        let light_pos = (uniforms.view * uniforms.light.pos.push(1.0)).xyz();
        let vertex_to_light = light_pos - p;
        let light_distance = vertex_to_light.norm();

        let v = (-p).normalize();
        let l = (vertex_to_light).normalize();
        let n = data.normal.normalize();
        let h = (l + v).normalize();

        let lambertian = dot(&l, &n).clamp(0.0, 1.0);
        let specular = dot(&h, &n)
            .clamp(0.0, 1.0)
            .powf(uniforms.material.specular_exp);
        let light_scale = uniforms.light.power / light_distance;

        let (texture, sampler) = &uniforms.material.texture;
        let sample = sampler.sample(texture, data.uv, d_data[0].uv, d_data[1].uv);
        let albedo = data.color.component_mul(&sample);
        return albedo.component_mul(
            &(uniforms.env.ambient_color
                + uniforms
                    .material
                    .diffuse_color
                    .component_mul(&uniforms.light.color)
                    * lambertian
                    * light_scale
                + uniforms
                    .material
                    .specular_color
                    .component_mul(&uniforms.light.color)
                    * specular
                    * light_scale),
        );
    }
}
