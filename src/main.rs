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

    // let vertices = [vec2(320.0, 90.0), vec2(160.0, 270.0), vec2(480.0, 270.0)];
    // let vertices = [vec2(160.0, 90.0), vec2(480.0, 90.0), vec2(320.0, 270.0)];
    let vertices = [vec2(480.0, 180.0), vec2(160.0, 90.0), vec2(160.0, 270.0)];
    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        fill_triangle(
            &mut framebuffer,
            [vertices[0], vertices[1], vertices[2]],
            WHITE,
        );

        framebuffer.update_window(&mut window);
    }
}

fn fill_triangle(framebuffer: &mut Framebuffer, mut vertices: [Vec2; 3], color: Color) {
    vertices.sort_unstable_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    if vertices[1].y == vertices[2].y {
        if vertices[1].x > vertices[2].x {
            vertices.swap(1, 2);
        }
        fill_flat_bottom_triangle(framebuffer, vertices, color);
    } else if vertices[0].y == vertices[1].y {
        if vertices[0].x > vertices[1].x {
            vertices.swap(0, 1);
        }
        fill_flat_top_triangle(framebuffer, vertices, color);
    } else {
        let [v0, v1, v2] = vertices;
        let alpha = (v1.y - v0.y) / (v2.y - v0.y);
        let v_mid = Vec2::lerp(&v0, &v2, alpha);
        if v1.x < v_mid.x {
            fill_flat_bottom_triangle(framebuffer, [v0, v1, v_mid], color);
            fill_flat_top_triangle(framebuffer, [v1, v_mid, v2], color);
        } else {
            fill_flat_bottom_triangle(framebuffer, [v0, v_mid, v1], color);
            fill_flat_top_triangle(framebuffer, [v_mid, v1, v2], color);
        }
    }
}

fn fill_flat_top_triangle(framebuffer: &mut Framebuffer, [v0, v1, v2]: [Vec2; 3], color: Color) {
    let slope_0 = (v2.x - v0.x) / (v2.y - v0.y);
    let slope_1 = (v2.x - v1.x) / (v2.y - v1.y);

    fill_flat_triangle(framebuffer, [v0, v1, v2], v1.x, slope_0, slope_1, color);
}

fn fill_flat_bottom_triangle(framebuffer: &mut Framebuffer, [v0, v1, v2]: [Vec2; 3], color: Color) {
    let slope_0 = (v1.x - v0.x) / (v1.y - v0.y);
    let slope_1 = (v2.x - v0.x) / (v2.y - v0.y);

    fill_flat_triangle(framebuffer, [v0, v1, v2], v0.x, slope_0, slope_1, color);
}

#[inline]
fn fill_flat_triangle(
    framebuffer: &mut Framebuffer,
    [v0, v1, v2]: [Vec2; 3],
    mut interpolant_1: f32,
    slope_0: f32,
    slope_1: f32,
    color: Color,
) {
    let mut interpolant_0 = v0.x;

    let y_start = v0.y as usize;
    let y_end = v2.y as usize;

    for y in y_start..y_end {
        let start_x = interpolant_0 as usize;
        let end_x = interpolant_1 as usize;

        for x in start_x..end_x {
            framebuffer.set_color((x, y), color);
        }

        interpolant_0 += slope_0;
        interpolant_1 += slope_1;
    }
}
