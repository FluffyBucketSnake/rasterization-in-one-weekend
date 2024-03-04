use nalgebra_glm::{vec2, Vec2};

use crate::{color::Color, framebuffer::Framebuffer};

pub fn fill_triangle(framebuffer: &mut Framebuffer, [v0, v1, v2]: &[Vec2; 3], color: Color) {
    let x_min = v0.x.min(v1.x.min(v2.x)) as usize;
    let x_max = v0.x.max(v1.x.max(v2.x)) as usize;
    let y_min = v0.y.min(v1.y.min(v2.x)) as usize;
    let y_max = v0.y.max(v1.x.max(v2.y)) as usize;
    for y in y_min..y_max {
        for x in x_min..x_max {
            let p = vec2(x as f32, y as f32);
            let signed_area = edge_function(v0, v1, v2);
            let abp = edge_function(v0, v1, &p);
            let bcp = edge_function(v1, v2, &p);
            let cap = edge_function(v2, v0, &p);

            let u = abp / signed_area;
            let v = bcp / signed_area;
            let w = cap / signed_area;

            if u >= 0.0 && v >= 0.0 && w >= 0.0 {
                framebuffer.set_color((x, y), color);
            }
        }
    }
}

#[inline]
fn edge_function(v0: &Vec2, v1: &Vec2, v2: &Vec2) -> f32 {
    Vec2::perp(&(v1 - v0), &(v2 - v0))
}
