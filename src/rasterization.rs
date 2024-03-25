use nalgebra_glm::{ceil, floor, vec2, vec3, RealNumber, TVec2, Vec2, Vec3};
use simba::scalar::FixedI28F4;

type FVec2 = TVec2<FixedI28F4>;

const EPSILON: FixedI28F4 = FixedI28F4::from_bits(0x01);

pub fn rasterize_solid_triangle(vertices: &[Vec2; 3], mut f: impl FnMut(Vec2, Vec3)) {
    let [c0, c1, c2] = vertices.clone().map(vec2_to_fvec2);

    let min = floor(&c0.inf(&c1.inf(&c2)));
    let max = ceil(&c0.sup(&c1.sup(&c2)));

    let signed_area = edge_function(c0, c1, c2).0.to_num::<f32>();

    if signed_area < 0.0 {
        return;
    }

    let u_bias = left_or_top_edge_bias(c1, c2, EPSILON);
    let v_bias = left_or_top_edge_bias(c2, c0, EPSILON);
    let w_bias = left_or_top_edge_bias(c0, c1, EPSILON);
    let half = FixedI28F4::from_bits(0b1000);

    let mut y = min.y;
    while y <= max.y {
        let mut x = min.x;
        while x <= max.x {
            let p = vec2(x + half, y + half);
            let u = edge_function(c1, c2, p) + u_bias;
            let v = edge_function(c2, c0, p) + v_bias;
            let w = edge_function(c0, c1, p) + w_bias;

            if u >= num::zero() && v >= num::zero() && w >= num::zero() {
                f(
                    vec2(x, y).map(|c| c.0.to_num()),
                    vec3(u, v, w).map(|c| c.0.to_num::<f32>()) / signed_area,
                )
            }
            x += num::one();
        }
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
