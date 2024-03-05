use nalgebra_glm::{vec2, Vec2};

use crate::{color::Color, framebuffer::Framebuffer};

pub fn fill_triangle(framebuffer: &mut Framebuffer, [v0, v1, v2]: &[Vec2; 3], color: Color) {
    let min = v0.inf(&v1.inf(v2));
    let max = v0.sup(&v1.sup(v2));

    let signed_area = edge_function(v0, v1, v2);
    let u_bias = left_or_top_edge_bias(v1, v2);
    let v_bias = left_or_top_edge_bias(v2, v0);
    let w_bias = left_or_top_edge_bias(v0, v1);

    let mut y = min.y;
    while y <= max.y {
        let mut x = min.x;
        while x <= max.x {
            let p = vec2(x, y);
            let u = (edge_function(v1, v2, &p) + u_bias) / signed_area;
            let v = (edge_function(v2, v0, &p) + v_bias) / signed_area;
            let w = (edge_function(v0, v1, &p) + w_bias) / signed_area;

            if u >= 0.0 && v >= 0.0 && w >= 0.0 {
                framebuffer.set_color((x as usize, y as usize), color);
            }
            x += 1.0;
        }
        y += 1.0;
    }
}

#[inline]
fn edge_function(v0: &Vec2, v1: &Vec2, v2: &Vec2) -> f32 {
    Vec2::perp(&(v2 - v0), &(v1 - v0))
}

#[inline]
fn left_or_top_edge_bias(start: &Vec2, end: &Vec2) -> f32 {
    let edge = end - start;
    let is_left_edge = edge.y > 0.0;
    let is_top_edge = edge.y == 0.0 && edge.x < 0.0;
    if is_left_edge || is_top_edge {
        return 0.0;
    } else {
        return -1.0 / 16.0;
    }
}

#[cfg(test)]
mod test {
    use crate::color::{BLACK, WHITE};

    use super::*;

    #[test]
    pub fn half_pixel_center() {
        let mut framebuffer = Framebuffer::new(9, 9);
        framebuffer.clear(BLACK);

        fill_triangle(
            &mut framebuffer,
            &[vec2(1.25, 1.25), vec2(1.5, 1.75), vec2(1.75, 1.25)],
            WHITE,
        );

        for y in 0..9 {
            for x in 0..9 {
                if (x, y) == (1, 1) {
                    continue;
                }
                assert_eq!(framebuffer.get_color((x, y)), BLACK);
            }
        }
        assert_eq!(framebuffer.get_color((1, 1)), WHITE);
    }

    #[test]
    pub fn top_left_rule() {
        let mut framebuffer = Framebuffer::new(3, 3);
        framebuffer.clear(BLACK);

        fill_triangle(
            &mut framebuffer,
            &[vec2(0.5, 0.5), vec2(2.5, 2.5), vec2(2.5, 0.5)],
            WHITE,
        );

        assert_eq!(framebuffer.get_color((0, 0)), WHITE);
        assert_eq!(framebuffer.get_color((1, 0)), WHITE);
        assert_eq!(framebuffer.get_color((2, 0)), BLACK);
        assert_eq!(framebuffer.get_color((0, 1)), BLACK);
        assert_eq!(framebuffer.get_color((1, 1)), WHITE);
        assert_eq!(framebuffer.get_color((2, 1)), BLACK);
        assert_eq!(framebuffer.get_color((0, 2)), BLACK);
        assert_eq!(framebuffer.get_color((1, 2)), BLACK);
        assert_eq!(framebuffer.get_color((2, 2)), BLACK);
    }
}
