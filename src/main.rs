mod color;
mod framebuffer;

use color::{Color, WHITE};
use framebuffer::Framebuffer;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use nalgebra_glm::{vec2, Vec2};

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

    let vertices = [vec2(320.0, 90.0), vec2(160.0, 270.0), vec2(480.0, 270.0)];
    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        draw_line(&mut framebuffer, vertices[0], vertices[1], WHITE);
        draw_line(&mut framebuffer, vertices[1], vertices[2], WHITE);
        draw_line(&mut framebuffer, vertices[2], vertices[0], WHITE);

        framebuffer.update_window(&mut window);
    }
}

fn draw_line(framebuffer: &mut Framebuffer, start: Vec2, end: Vec2, color: Color) {
    let diff = end - start;
    let step = diff.abs().max();
    let derivative = diff / step;
    let step = step as usize;
    let mut current = start;
    let mut i = 0;
    while i <= step {
        framebuffer.set_color((current.x as usize, current.y as usize), color);
        current += derivative;
        i += 1;
    }
}
