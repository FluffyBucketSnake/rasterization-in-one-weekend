use std::f32::consts::PI;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use nalgebra_glm::{vec2, vec3, Vec2};
use rasterization_in_a_weekend::{
    color::{BLACK, BLUE, GREEN, RED, WHITE},
    framebuffer::Framebuffer,
    image::Image,
    model::unit_cube,
    pipeline::RasterizationPipeline,
    vertex::Vertex,
    viewport::Viewport,
};

const WINDOW_TITLE: &str = "Rasterization in One Weekend";
const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 360;

fn main() {
    let mut framebuffer = Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut window = Window::new(
        WINDOW_TITLE,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();
    window.set_target_fps(60);

    let image = Image::from_file("textures/simple.png".into()).unwrap();
    let viewport = Viewport::full(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    let projection = nalgebra_glm::perspective_fov_rh_zo(
        PI / 3.0,
        viewport.width,
        viewport.height,
        0.01,
        1000.0,
    );
    let view = nalgebra_glm::look_at_rh(
        &vec3(0.0, 0.0, 0.0),
        &vec3(0.0, 0.0, 1.0),
        &vec3(0.0, -1.0, 0.0),
    );
    let proj_view = projection * view;
    let default_world = nalgebra_glm::scale(
        &nalgebra_glm::translate(&nalgebra_glm::identity(), &vec3(0.0, 0.0, 10.0)),
        &vec3(2.0, 2.0, 2.0),
    );
    let pipeline = RasterizationPipeline::new(viewport);
    let mut colors = std::iter::repeat([RED, GREEN, BLUE, WHITE]).flatten();
    let mut uv = std::iter::repeat([
        vec2(0.0, 0.0),
        vec2(0.0, 1.0),
        vec2(1.0, 1.0),
        vec2(1.0, 0.0),
    ])
    .flatten();
    let vertices = unit_cube(|_, c| {
        Vertex::new(
            c - vec3(0.0, 2.0, 0.0),
            colors.next().unwrap(),
            uv.next().unwrap(),
        )
    })
    .into_iter()
    .chain(unit_cube(|_, c| {
        Vertex::new(
            c + vec3(0.0, 2.0, 0.0),
            colors.next().unwrap(),
            uv.next().unwrap(),
        )
    }))
    .collect::<Vec<_>>();

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
        let transform = proj_view * world;
        pipeline.draw_triangles(&mut framebuffer, &transform, &image, &vertices);
        framebuffer.update_window(&mut window);
        frame += 1;
    }
}
