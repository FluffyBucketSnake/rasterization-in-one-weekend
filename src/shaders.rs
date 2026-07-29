use std::fmt::Debug;

use nalgebra_glm::{dot, vec3, Mat3, Mat4, Vec2, Vec3};

use crate::{color::Color, vertex::Vertex};

pub trait VertexShader<U, VSIn>: Debug {
    fn vs(&self, uniforms: &U, input: &VSIn) -> Vertex;
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

#[derive(Debug, Default)]
pub struct BasicVertexShader;

impl VertexShader<BasicUniforms, BasicVertex> for BasicVertexShader {
    fn vs(&self, uniforms: &BasicUniforms, input: &BasicVertex) -> Vertex {
        let p = (uniforms.view * uniforms.world * input.coords.push(1.0)).xyz();
        let light_pos = (uniforms.view * uniforms.light.pos.push(1.0)).xyz();
        let vertex_to_light = light_pos - p;
        let light_distance = vertex_to_light.norm();

        let v = (-p).normalize();
        let l = (vertex_to_light).normalize();
        let n = (uniforms.normal_matrix * input.normal).normalize();
        let h = (l + v).normalize();

        let lambertian = dot(&l, &n).clamp(0.0, 1.0);
        let specular = dot(&h, &n)
            .clamp(0.0, 1.0)
            .powf(uniforms.material.specular_exp);
        let light_scale = uniforms.light.power / light_distance;

        let color = input.color.component_mul(
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

        Vertex {
            coords: uniforms.proj * uniforms.view * uniforms.world * input.coords.push(1.0),
            color,
            uv: input.uv,
        }
    }
}
