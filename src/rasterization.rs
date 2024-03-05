use nalgebra_glm::{vec2, Vec2};

use crate::{framebuffer::Framebuffer, vertex::Vertex};

pub fn fill_triangle<V: Vertex>(framebuffer: &mut Framebuffer, vertices: &[V; 3]) {
    let mut coords = vertices.iter().map(|v| v.coords()).copied();
    let c0 = coords.next().unwrap();
    let c1 = coords.next().unwrap();
    let c2 = coords.next().unwrap();

    let min = c0.inf(&c1.inf(&c2));
    let max = c0.sup(&c1.sup(&c2));

    let signed_area = edge_function(c0, c1, c2);
    let u_bias = left_or_top_edge_bias(c1, c2);
    let v_bias = left_or_top_edge_bias(c2, c0);
    let w_bias = left_or_top_edge_bias(c0, c1);

    let mut y = min.y;
    while y <= max.y {
        let mut x = min.x;
        while x <= max.x {
            let p = vec2(x, y);
            let u = edge_function(c1, c2, p) + u_bias;
            let v = edge_function(c2, c0, p) + v_bias;
            let w = edge_function(c0, c1, p) + w_bias;

            if u >= 0.0 && v >= 0.0 && w >= 0.0 {
                framebuffer.set_color(
                    (x as usize, y as usize),
                    V::fragment_color(vertices, u / signed_area, v / signed_area),
                );
            }
            x += 1.0;
        }
        y += 1.0;
    }
}

#[inline]
fn edge_function(v0: Vec2, v1: Vec2, v2: Vec2) -> f32 {
    Vec2::perp(&(v2 - v0), &(v1 - v0))
}

#[inline]
fn left_or_top_edge_bias(start: Vec2, end: Vec2) -> f32 {
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

    #[derive(Debug, Clone)]
    struct DummyVertex(Vec2);

    impl Vertex for DummyVertex {
        fn fragment_color(_: &[Self; 3], _: f32, _: f32) -> crate::color::Color {
            WHITE
        }

        fn coords(&self) -> &Vec2 {
            &self.0
        }
    }

    fn vertex(x: f32, y: f32) -> DummyVertex {
        DummyVertex(vec2(x, y))
    }

    #[test]
    pub fn half_pixel_center() {
        let mut framebuffer = Framebuffer::new(9, 9);
        framebuffer.clear(BLACK);

        fill_triangle(
            &mut framebuffer,
            &[vertex(1.25, 1.25), vertex(1.5, 1.75), vertex(1.75, 1.25)],
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
            &[vertex(0.5, 0.5), vertex(2.5, 2.5), vertex(2.5, 0.5)],
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
