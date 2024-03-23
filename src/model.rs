use std::f32::consts::PI;

use nalgebra_glm::{vec2, Vec2, Vec3};

pub fn unit_triangle<V>(mut f: impl FnMut(Vec2) -> V) -> [V; 3] {
    [vec2(0.0, -0.5), vec2(-0.5, 0.5), vec2(0.5, 0.5)].map(f)
}

pub fn unit_quad<V: Clone>(mut f: impl FnMut(Vec2) -> V) -> [V; 6] {
    let top_left = f(vec2(-0.5, -0.5));
    let bottom_left = f(vec2(-0.5, 0.5));
    let bottom_right = f(vec2(0.5, 0.5));
    let top_right = f(vec2(0.5, -0.5));
    [
        top_left,
        bottom_left.clone(),
        top_right.clone(),
        bottom_left,
        bottom_right,
        top_right,
    ]
}

#[repr(u8)]
pub enum CubeSide {
    Top,
    Left,
    Bottom,
    Right,
    Forward,
    Backward,
}

pub fn unit_cube<V: Clone>(mut f: impl FnMut(CubeSide, Vec3) -> V) -> [V; 36] {
    use nalgebra_glm::identity;

    let transform = nalgebra_glm::rotate_x(&identity(), PI / 2.0).fixed_resize::<3, 3>(0.0);
    let top = unit_quad(|coords| f(CubeSide::Top, transform * coords.push(-0.5)));

    let transform = nalgebra_glm::rotate_y(&identity(), -PI / 2.0).fixed_resize::<3, 3>(0.0);
    let left = unit_quad(|coords| f(CubeSide::Left, transform * coords.push(-0.5)));

    let transform = nalgebra_glm::rotate_x(&identity(), -PI / 2.0).fixed_resize::<3, 3>(0.0);
    let bottom = unit_quad(|coords| f(CubeSide::Bottom, transform * coords.push(-0.5)));

    let transform = nalgebra_glm::rotate_y(&identity(), PI / 2.0).fixed_resize::<3, 3>(0.0);
    let right = unit_quad(|coords| f(CubeSide::Right, transform * coords.push(-0.5)));

    let transform = nalgebra_glm::rotate_x(&identity(), PI).fixed_resize::<3, 3>(0.0);
    let forward = unit_quad(|coords| f(CubeSide::Forward, transform * coords.push(-0.5)));

    let backward = unit_quad(|coords| f(CubeSide::Forward, coords.push(-0.5)));

    let mut iter = top
        .into_iter()
        .chain(left)
        .chain(bottom)
        .chain(right)
        .chain(forward)
        .chain(backward);
    std::array::from_fn(|_| iter.next().unwrap())
}

#[cfg(test)]
mod tests {
    use nalgebra_glm::Vec3;

    use super::unit_cube;

    #[test]
    fn unit_cube_primitives_are_counterclockwise() {
        let expected_normals = [
            -Vec3::y_axis(),
            -Vec3::x_axis(),
            Vec3::y_axis(),
            Vec3::x_axis(),
            -Vec3::z_axis(),
            Vec3::z_axis(),
        ];

        let vertices = unit_cube(|_, i| i);

        assert_eq!(
            vertices
                .chunks(3)
                .enumerate()
                .map(|(i, t)| expected_normals[i / 2].dot(&(t[2] - t[0]).cross(&(t[1] - t[0]))))
                .collect::<Vec<_>>(),
            vec![1.0; 12]
        );
    }
}
