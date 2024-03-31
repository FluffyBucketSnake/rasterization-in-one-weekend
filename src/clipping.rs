use nalgebra_glm::{lerp, Vec3, Vec4};

pub fn clip_triangle(input_vertices: [Vec4; 3]) -> Vec<(Vec4, Vec3)> {
    const PLANES: [(f32, Vec4); 6] = [
        (1.0, Vec4::new(1.0, 0.0, 0.0, 0.0)),
        (1.0, Vec4::new(-1.0, 0.0, 0.0, 0.0)),
        (1.0, Vec4::new(0.0, 1.0, 0.0, 0.0)),
        (1.0, Vec4::new(0.0, -1.0, 0.0, 0.0)),
        (0.0, Vec4::new(0.0, 0.0, 1.0, 0.0)),
        (1.0, Vec4::new(0.0, 0.0, -1.0, 0.0)),
    ];

    const WEIGHTS: [Vec3; 3] = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];

    let mut buffer_vertices = Vec::with_capacity(3 + PLANES.len());
    buffer_vertices.extend(input_vertices.into_iter().zip(WEIGHTS));
    let mut input_vertices = Vec::new();
    for (d, normal) in PLANES {
        input_vertices.clone_from(&buffer_vertices);
        buffer_vertices.clear();

        if input_vertices.len() == 0 {
            return vec![];
        }

        let mut j = input_vertices.len() - 1;
        for i in 0..input_vertices.len() {
            let (coords_i, weights_i) = input_vertices[i];
            let (coords_j, weights_j) = input_vertices[j];
            let hcoords_i = coords_i / coords_i.w;
            let hcoords_j = coords_j / coords_j.w;

            let distance_i = hcoords_i.dot(&normal) + d;
            let distance_j = hcoords_j.dot(&normal) + d;

            let alpha = -distance_j / (hcoords_i - hcoords_j).dot(&normal);
            let intersection = (
                lerp(&coords_j, &coords_i, alpha),
                lerp(&weights_j, &weights_i, alpha),
            );

            if distance_i >= 0.0 {
                if distance_j < 0.0 {
                    buffer_vertices.push(intersection);
                }
                buffer_vertices.push((coords_i, weights_i));
            } else if distance_j >= 0.0 {
                buffer_vertices.push(intersection);
            }
            j = i;
        }
    }
    return buffer_vertices;
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra_glm::{vec3, vec4};

    #[test]
    fn should_not_clip_primitives_fully_inside() {
        let triangles = [
            [
                vec4(0.0, -1.0, 0.0, 1.0),
                vec4(1.0, 1.0, 0.0, 1.0),
                vec4(-1.0, 1.0, 0.0, 1.0),
            ],
            [
                vec4(0.0, -2.0, 0.0, 2.0),
                vec4(2.0, 2.0, 0.0, 2.0),
                vec4(-2.0, 2.0, 0.0, 2.0),
            ],
        ];

        const EXPECTED_WEIGHTS: [Vec3; 3] = [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ];

        for triangle in triangles {
            let (coords, weights) = clip_triangle(triangle)
                .into_iter()
                .unzip::<Vec4, Vec3, Vec<_>, Vec<_>>();

            assert_eq!(coords, triangle, "Triangle {:?} was clipped", triangle);
            assert_eq!(
                weights, EXPECTED_WEIGHTS,
                "Triangle {:?} has wrong weights",
                weights
            );
        }
    }

    #[test]
    fn should_clip_primitives_with_one_vertex_outside() {
        let test_cases = [
            (
                [
                    vec4(0.0, -1.0, 0.5, 1.0),
                    vec4(0.0, 0.0, -0.5, 1.0),
                    vec4(0.0, 1.0, 0.5, 1.0),
                ],
                [
                    (vec4(0.0, -1.0, 0.5, 1.0), vec3(1.0, 0.0, 0.0)),
                    (vec4(0.0, -0.5, 0.0, 1.0), vec3(0.5, 0.5, 0.0)),
                    (vec4(0.0, 0.5, 0.0, 1.0), vec3(0.0, 0.5, 0.5)),
                    (vec4(0.0, 1.0, 0.5, 1.0), vec3(0.0, 0.0, 1.0)),
                ],
            ),
            (
                [
                    vec4(0.0, -1.0, 0.5, 1.0),
                    vec4(0.0, 0.0, 1.5, 1.0),
                    vec4(0.0, 1.0, 0.5, 1.0),
                ],
                [
                    (vec4(0.0, -1.0, 0.5, 1.0), vec3(1.0, 0.0, 0.0)),
                    (vec4(0.0, -0.5, 1.0, 1.0), vec3(0.5, 0.5, 0.0)),
                    (vec4(0.0, 0.5, 1.0, 1.0), vec3(0.0, 0.5, 0.5)),
                    (vec4(0.0, 1.0, 0.5, 1.0), vec3(0.0, 0.0, 1.0)),
                ],
            ),
            // [
            //     vec4(-0.5, -1.0, 0.0, 1.0),
            //     vec4(-1.5, 0.0, 0.0, 1.0),
            //     vec4(-0.5, 1.0, 0.0, 1.0),
            // ],
            // [
            //     vec4(-1.0, -1.0, 0.0, 1.0),
            //     vec4(1.5, 0.0, 0.0, 1.0),
            //     vec4(0.5, 1.0, 0.0, 1.0),
            // ],
        ];

        for (input, expected_output) in test_cases {
            assert_eq!(
                clip_triangle(input),
                expected_output,
                "Wrong clipping for triangle {:?}",
                input
            );
        }
    }
}
