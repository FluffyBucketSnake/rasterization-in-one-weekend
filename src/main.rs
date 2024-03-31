use std::f32::consts::PI;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use nalgebra_glm::vec3;
use rasterization_in_a_weekend::{
    color::{BLACK, BLUE, GREEN, RED, WHITE},
    framebuffer::Framebuffer,
    model::unit_cube,
    pipeline::RasterizationPipeline,
    vertex::BasicVertex3D,
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
    window.limit_update_rate(Some(std::time::Duration::from_micros(1000 / 60)));

    let rotation = PI / 150.0;
    let mut angle = 0.0;

    let viewport = Viewport::full(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    let projection =
        nalgebra_glm::perspective_fov_zo(PI / 3.0, viewport.width, viewport.height, 0.001, 50.0);
    let view = nalgebra_glm::look_at_rh(
        &vec3(0.0, 0.0, 0.0),
        &vec3(0.0, 0.0, 1.0),
        &vec3(0.0, -1.0, 0.0),
    );
    let proj_view = projection * view;
    let default_world = nalgebra_glm::scale(
        &nalgebra_glm::translate(&nalgebra_glm::identity(), &vec3(0.0, 0.0, 50.0)),
        &vec3(5.0, 5.0, 5.0),
    );
    let transform = proj_view * default_world;
    let mut pipeline = RasterizationPipeline::new(transform, viewport);

    let mut colors = std::iter::repeat([RED, GREEN, BLUE, WHITE]).flatten();
    let vertices = unit_cube(|_, c| BasicVertex3D::new(c, colors.next().unwrap()));

    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        framebuffer.clear(BLACK);
        angle = angle % (2.0 * PI);
        let world = nalgebra_glm::rotate_y(&default_world, angle);
        let transform = proj_view * world;
        pipeline.set_transform(transform);
        pipeline.draw_triangles(&mut framebuffer, &vertices);
        framebuffer.update_window(&mut window);
        angle += rotation;
    }
}
