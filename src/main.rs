use minifb::{Key, KeyRepeat, Window, WindowOptions};
use nalgebra_glm::vec2;
use rasterization_in_a_weekend::{
    color::WHITE, framebuffer::Framebuffer, rasterization::fill_triangle,
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

    let vertices = [vec2(480.0, 180.0), vec2(160.0, 90.0), vec2(160.0, 270.0)];
    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        fill_triangle(&mut framebuffer, &vertices, WHITE);
        framebuffer.update_window(&mut window);
    }
}
