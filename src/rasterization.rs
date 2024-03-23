use nalgebra_glm::{vec2, vec3, Vec2, Vec3};

pub fn rasterize_solid_triangle(vertices: &[Vec2; 3], mut f: impl FnMut(Vec2, Vec3)) {
    let [c0, c1, c2] = *vertices;

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
                f(vec2(x.trunc(), y.trunc()), vec3(u, v, w) / signed_area)
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
    use super::*;

    #[test]
    pub fn half_pixel_center() {
        let mut fragments = Vec::new();

        rasterize_solid_triangle(
            &[vec2(1.25, 1.25), vec2(1.5, 1.75), vec2(1.75, 1.25)],
            |coords, _| fragments.push(coords),
        );

        assert_eq!(fragments, [vec2(1.0, 1.0)]);
    }

    #[test]
    pub fn top_left_rule() {
        let mut fragments = Vec::new();

        rasterize_solid_triangle(
            &[vec2(0.5, 0.5), vec2(2.5, 2.5), vec2(2.5, 0.5)],
            |coords, _| fragments.push(coords),
        );

        assert_eq!(fragments, [vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0)]);
    }
}
