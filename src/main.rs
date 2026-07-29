use std::f32::consts::PI;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use nalgebra_glm::{identity, vec3};
use rasterization_in_a_weekend::{
    color::{self, BLACK},
    framebuffer::Framebuffer,
    model::load_gltf,
    pipeline::RasterizationPipeline,
    sampler::{AddressMode, Filter, Sampler},
    shaders::{
        BasicUniforms, BasicVertex, BasicVertexShader, Environment, PhongMaterial, PointLight,
    },
    viewport::Viewport,
};

const WINDOW_TITLE: &str = "Rasterization in One Weekend";
const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 360;

fn main() -> anyhow::Result<()> {
    let mut framebuffer = Framebuffer::new([WINDOW_WIDTH, WINDOW_HEIGHT]);
    let mut window = Window::new(
        WINDOW_TITLE,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();
    window.set_target_fps(60);

    // Assets
    let model = load_gltf("models/Ball.glb".into(), 4, |coords, uv, normal, color| {
        BasicVertex {
            coords: coords.xyz() / coords.w,
            uv,
            color: color.unwrap_or(color::WHITE),
            normal,
        }
    })?;

    // Pipeline
    let sampler = Sampler::new(
        AddressMode::Clamp,
        AddressMode::Clamp,
        Filter::Anisotropic(4),
        Filter::Linear,
    );
    let vertex_shader = BasicVertexShader::default();
    let viewport = Viewport::full(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    let pipeline = RasterizationPipeline::new(vertex_shader, viewport);

    // Uniforms
    let eye = vec3(0.0, 0.0, 0.0);
    let projection = nalgebra_glm::perspective_fov_rh_zo(
        PI / 3.0,
        viewport.width,
        viewport.height,
        0.01,
        1000.0,
    );
    let view = nalgebra_glm::look_at_rh(&eye, &vec3(0.0, 0.0, 1.0), &vec3(0.0, -1.0, 0.0));
    let default_world = nalgebra_glm::scale(
        &nalgebra_glm::translate(&nalgebra_glm::identity(), &vec3(0.0, 0.0, 10.0)),
        &vec3(2.0, 2.0, 2.0),
    );
    let mut uniforms = BasicUniforms {
        world: default_world,
        view: view,
        proj: projection,
        normal_matrix: identity(),
        light: PointLight {
            pos: vec3(-5.0, 5.0, 0.0),
            color: color::WHITE,
            power: 10.0,
        },
        material: PhongMaterial {
            specular_exp: 32.0,
            diffuse_color: color::WHITE,
            specular_color: color::WHITE,
        },
        env: Environment {
            ambient_color: color::BLACK,
        },
    };

    let amplitude = 1.0;
    let speed = PI / 60.0;
    let rotation = PI / 150.0;
    let mut frame = 0;
    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        framebuffer.clear(BLACK, std::f32::INFINITY);
        let f32_frame = frame as f32;
        let angle = f32_frame * rotation;
        let z_delta = amplitude * f32::cos(f32_frame * speed);

        let world = nalgebra_glm::rotate_x(
            &nalgebra_glm::translate(&default_world, &vec3(0.0, 0.0, z_delta)),
            angle,
        );
        uniforms.normal_matrix =
            nalgebra_glm::inverse_transpose(nalgebra_glm::mat4_to_mat3(&(view * world)));
        uniforms.world = world;

        pipeline.draw_triangles_indexed(
            &model.indices,
            &model.vertices,
            &uniforms,
            (&model.textures[0], &sampler),
            &mut framebuffer,
        );

        framebuffer.update_window(&mut window);
        frame += 1;
    }
    return Ok(());
}
