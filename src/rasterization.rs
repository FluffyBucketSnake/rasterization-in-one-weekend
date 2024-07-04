use nalgebra_glm::{ceil, floor, vec2, vec3, RealNumber, TVec2, Vec2, Vec3};
use simba::scalar::FixedI28F4;

type FVec2 = TVec2<FixedI28F4>;

const EPSILON: FixedI28F4 = FixedI28F4::from_bits(0x01);

#[derive(Debug, Clone, Copy)]
pub struct Fragment {
    pub coords: TVec2<usize>,
    pub t: Vec3,
    pub dt_dx: Vec3,
    pub dt_dy: Vec3,
}

pub fn rasterize_solid_triangle(vertices: &[Vec2; 3], mut f: impl FnMut(Fragment)) {
    let [c0, c1, c2] = vertices.clone().map(vec2_to_fvec2);

    let min = floor(&c0.inf(&c1.inf(&c2)));
    let max = ceil(&c0.sup(&c1.sup(&c2)));

    let signed_area = edge_function(c0, c1, c2).0.to_num::<f32>();

    if signed_area < 0.0 {
        return;
    }

    let half = FixedI28F4::from_bits(0b1000);
    let offset = vec2(half, half);

    let w_bias = vec3(
        left_or_top_edge_bias(c1, c2, EPSILON),
        left_or_top_edge_bias(c2, c0, EPSILON),
        left_or_top_edge_bias(c0, c1, EPSILON),
    );
    let w_0 = vec3(
        edge_function(c1, c2, min + offset),
        edge_function(c2, c0, min + offset),
        edge_function(c0, c1, min + offset),
    ) + w_bias;
    let dw_dx = vec3(c2.y - c1.y, c0.y - c2.y, c1.y - c0.y);
    let dw_dy = vec3(c1.x - c2.x, c2.x - c0.x, c0.x - c1.x);

    let dw_dx_f32 = dw_dx.map(|c| c.0.to_num()) / signed_area;
    let dw_dy_f32 = dw_dy.map(|c| c.0.to_num()) / signed_area;

    let mut w_y = w_0;
    let mut y = min.y;
    while y <= max.y {
        let mut w = w_y;
        let mut x = min.x;
        while x <= max.x {
            if w.x >= num::zero() && w.y >= num::zero() && w.z >= num::zero() {
                f(Fragment {
                    coords: vec2(x, y).map(|c| c.0.to_num()),
                    t: w.map(|c| c.0.to_num::<f32>()) / signed_area,
                    dt_dx: dw_dx_f32,
                    dt_dy: dw_dy_f32,
                })
            }
            w += dw_dx;
            x += num::one();
        }
        w_y += dw_dy;
        y += num::one();
    }
}

#[inline]
fn vec2_to_fvec2(src: Vec2) -> FVec2 {
    src.map(|c| FixedI28F4::from_num(c))
}

#[inline]
fn edge_function<T: RealNumber>(v0: TVec2<T>, v1: TVec2<T>, v2: TVec2<T>) -> T {
    TVec2::perp(&(v2 - v0), &(v1 - v0))
}

#[inline]
fn left_or_top_edge_bias<T: RealNumber>(start: TVec2<T>, end: TVec2<T>, epsilon: T) -> T {
    let edge = end - start;
    let is_left_edge = edge.y > num::zero();
    let is_top_edge = edge.y == num::zero() && edge.x < num::zero();
    if is_left_edge || is_top_edge {
        return num::zero();
    } else {
        return -epsilon;
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
            |Fragment { coords, .. }| fragments.push(coords),
        );

        assert_eq!(fragments, [vec2(1, 1)]);
    }

    #[test]
    pub fn top_left_rule() {
        let mut fragments = Vec::new();

        rasterize_solid_triangle(
            &[vec2(0.5, 0.5), vec2(2.5, 2.5), vec2(2.5, 0.5)],
            |Fragment { coords, .. }| fragments.push(coords),
        );

        assert_eq!(fragments, [vec2(0, 0), vec2(1, 0), vec2(1, 1)]);
    }
}
