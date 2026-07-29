use std::{f32::consts::PI, path::PathBuf};

use gltf::{
    accessor::{DataType, Dimensions},
    buffer::{self},
    Accessor, Semantic,
};
use nalgebra_glm::{vec2, Mat4, Vec2, Vec3, Vec4};

use crate::{
    color::{self, Color},
    image::Image,
    vertex::Vertex,
};

pub fn unit_triangle<V>(f: impl FnMut(Vec2) -> V) -> [V; 3] {
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
    let top = unit_quad(|coords| f(CubeSide::Top, transform * coords.push(0.5)));

    let transform = nalgebra_glm::rotate_y(&identity(), -PI / 2.0).fixed_resize::<3, 3>(0.0);
    let left = unit_quad(|coords| f(CubeSide::Left, transform * coords.push(0.5)));

    let transform = nalgebra_glm::rotate_x(&identity(), -PI / 2.0).fixed_resize::<3, 3>(0.0);
    let bottom = unit_quad(|coords| f(CubeSide::Bottom, transform * coords.push(0.5)));

    let transform = nalgebra_glm::rotate_y(&identity(), PI / 2.0).fixed_resize::<3, 3>(0.0);
    let right = unit_quad(|coords| f(CubeSide::Right, transform * coords.push(0.5)));

    let transform = nalgebra_glm::rotate_x(&identity(), PI).fixed_resize::<3, 3>(0.0);
    let forward = unit_quad(|coords| f(CubeSide::Forward, transform * coords.push(0.5)));

    let transform = identity();
    let backward = unit_quad(|coords| f(CubeSide::Backward, transform * coords.push(0.5)));

    let mut iter = top
        .into_iter()
        .chain(left)
        .chain(bottom)
        .chain(right)
        .chain(forward)
        .chain(backward);
    std::array::from_fn(|_| iter.next().unwrap())
}

pub struct Model<V> {
    pub vertices: Vec<V>,
    pub indices: Vec<usize>,
    pub textures: Vec<Image<u32>>,
}

pub fn load_gltf<V>(
    path: PathBuf,
    mip_levels: usize,
    into_vertex: impl Fn(Vec4, Vec2, Vec3, Option<Color>) -> V,
) -> gltf::Result<Model<V>> {
    fn process_node<V>(
        node: gltf::Node<'_>,
        parent_transform: Mat4,
        buffers: &Vec<buffer::Data>,
        into_vertex: &impl Fn(Vec4, Vec2, Vec3, Option<Color>) -> V,
        vertices_output: &mut Vec<V>,
        indexes_output: &mut Vec<usize>,
    ) {
        let transform = parent_transform * read_transform(node.transform());

        if let Some(mesh) = node.mesh() {
            for primitive in mesh.primitives() {
                let base_idx = indexes_output.len();
                let positions =
                    read_positions(primitive.get(&Semantic::Positions).unwrap(), buffers);
                let tex_coords =
                    read_tex_coords(primitive.get(&Semantic::TexCoords(0)).unwrap(), buffers);
                let normals = read_normals(primitive.get(&Semantic::Normals).unwrap(), buffers);
                assert_eq!(
                    positions.len(),
                    tex_coords.len(),
                    "positions and tex_coords are incompatible"
                );
                let vertex_count = positions.len();
                let colors =
                    read_colors(primitive.get(&Semantic::Colors(0)), buffers, vertex_count);
                assert_eq!(colors.len(), vertex_count, "color accessor is incompatible");
                let indexes = read_indexes(primitive.indices(), base_idx, vertex_count, buffers);
                vertices_output.extend(
                    positions
                        .into_iter()
                        .zip(tex_coords)
                        .zip(colors)
                        .zip(normals)
                        .map(|(((coords, uv), color), normal)| {
                            into_vertex(coords, uv, normal, color)
                        }),
                );
                indexes_output.extend(indexes);
            }
        }
        for child in node.children() {
            process_node(
                child,
                transform,
                buffers,
                into_vertex,
                vertices_output,
                indexes_output,
            );
        }
    }

    fn read_transform(transform: gltf::scene::Transform) -> Mat4 {
        let [[m11, m12, m13, m14], [m21, m22, m23, m24], [m31, m32, m33, m34], [m41, m42, m43, m44]] =
            transform.matrix();
        return Mat4::new(
            m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, m41, m42, m43, m44,
        );
    }

    fn read_positions(accessor: gltf::Accessor<'_>, buffers: &Vec<buffer::Data>) -> Vec<Vec4> {
        use gltf::accessor::{DataType, Dimensions};

        let view = accessor.view().expect("sparse accessors are unsupported");
        let buffer = &buffers[view.buffer().index()];
        let base = view.offset() + accessor.offset();
        let len = view.length();
        let data = &buffer[base..base + len];

        let mut output = Vec::with_capacity(accessor.count());
        match (accessor.dimensions(), accessor.data_type()) {
            (Dimensions::Vec3, DataType::F32) => {
                read_vec3_f32_view(&accessor, &view, data, &mut |x, y, z| {
                    output.push(Vec4::new(x, -y, z, 1.0))
                });
            }
            (Dimensions::Vec4, DataType::F32) => {
                read_vec4_f32_view(&accessor, &view, data, &mut |x, y, z, w| {
                    output.push(Vec4::new(x, -y, z, w))
                });
            }

            other => panic!("incompatible accessor: {:?}", other),
        }
        return output;
    }

    fn read_tex_coords(accessor: gltf::Accessor<'_>, buffers: &Vec<buffer::Data>) -> Vec<Vec2> {
        use gltf::accessor::{DataType, Dimensions};

        let view = accessor.view().expect("sparse accessors are unsupported");
        let buffer = &buffers[view.buffer().index()];
        let base = view.offset() + accessor.offset();
        let len = view.length();
        let data = &buffer[base..base + len];

        let mut output = Vec::with_capacity(accessor.count());
        match (accessor.dimensions(), accessor.data_type()) {
            (Dimensions::Vec2, DataType::F32) => {
                read_vec2_f32_view(&accessor, &view, data, &mut |x, y| {
                    output.push(Vec2::new(x, y))
                });
            }
            other => panic!("incompatible accessor: {:?}", other),
        }
        return output;
    }

    fn read_colors(
        accessor: Option<Accessor<'_>>,
        buffers: &Vec<buffer::Data>,
        expected_len: usize,
    ) -> Vec<Option<Color>> {
        let accessor = if let Some(accessor) = accessor {
            accessor
        } else {
            return vec![None; expected_len];
        };

        let view = accessor.view().expect("sparse accessors are unsupported");
        let buffer = &buffers[view.buffer().index()];
        let base = view.offset() + accessor.offset();
        let len = view.length();
        let data = &buffer[base..base + len];

        let mut output = Vec::with_capacity(accessor.count());
        match (
            accessor.dimensions(),
            accessor.data_type(),
            accessor.normalized(),
        ) {
            (Dimensions::Vec3, DataType::F32, _) => {
                read_vec3_f32_view(&accessor, &view, data, &mut |x, y, z| {
                    output.push(Some(Color::new(x, y, z)))
                });
            }
            (Dimensions::Vec4, DataType::F32, _) => {
                read_vec4_f32_view(&accessor, &view, data, &mut |x, y, z, _| {
                    output.push(Some(Color::new(x, y, z)))
                });
            }
            (Dimensions::Vec3, DataType::U8, _) => {
                read_vec3_u8_view(&accessor, &view, data, &mut |x, y, z| {
                    output.push(Some(Color::new(
                        x as f32 / 255.0,
                        y as f32 / 255.0,
                        z as f32 / 255.0,
                    )))
                });
            }
            (Dimensions::Vec4, DataType::U8, _) => {
                read_vec4_u8_view(&accessor, &view, data, &mut |x, y, z, _| {
                    output.push(Some(Color::new(
                        x as f32 / 255.0,
                        y as f32 / 255.0,
                        z as f32 / 255.0,
                    )))
                });
            }
            other => panic!("incompatible accessor: {:?}", other),
        }
        return output;
    }

    fn read_normals(accessor: Accessor<'_>, buffers: &Vec<buffer::Data>) -> Vec<Vec3> {
        let view = accessor.view().expect("sparse accessors are unsupported");
        let buffer = &buffers[view.buffer().index()];
        let base = view.offset() + accessor.offset();
        let len = view.length();
        let data = &buffer[base..base + len];

        let mut output = Vec::with_capacity(accessor.count());
        match (accessor.dimensions(), accessor.data_type()) {
            (Dimensions::Vec3, DataType::F32) => {
                read_vec3_f32_view(&accessor, &view, data, &mut |x, y, z| {
                    output.push(Vec3::new(x, -y, z))
                });
            }
            (Dimensions::Vec4, DataType::F32) => {
                read_vec4_f32_view(&accessor, &view, data, &mut |x, y, z, _| {
                    output.push(Vec3::new(x, -y, z))
                });
            }
            other => panic!("incompatible accessor: {:?}", other),
        }
        return output;
    }

    fn read_indexes(
        accessor: Option<Accessor<'_>>,
        base_idx: usize,
        vertex_count: usize,
        buffers: &[buffer::Data],
    ) -> Vec<usize> {
        let accessor = if let Some(accessor) = accessor {
            accessor
        } else {
            return (base_idx..base_idx + vertex_count).collect();
        };

        let view = accessor.view().expect("sparse accessors are unsupported");
        let buffer = &buffers[view.buffer().index()];
        let base = view.offset() + accessor.offset();
        let len = view.length();
        let data = &buffer[base..base + len];

        let mut output = Vec::with_capacity(accessor.count());
        match (accessor.dimensions(), accessor.data_type()) {
            (Dimensions::Scalar, DataType::U8) => {
                read_u8_view(&accessor, &view, data, &mut |x| {
                    output.push(x as usize + base_idx)
                });
            }
            (Dimensions::Scalar, DataType::U16) => {
                read_u16_view(&accessor, &view, data, &mut |x| {
                    output.push(x as usize + base_idx)
                });
            }
            (Dimensions::Scalar, DataType::U32) => {
                read_u32_view(&accessor, &view, data, &mut |x| {
                    output.push(x as usize + base_idx)
                });
            }
            other => panic!("incompatible accessor: {:?}", other),
        }
        return output;
    }

    fn process_textures(src_images: Vec<gltf::image::Data>, mip_levels: usize) -> Vec<Image<u32>> {
        let mut output = Vec::with_capacity(src_images.len());
        for src in src_images {
            let pixel_count = (src.width * src.height) as usize;
            let mut buffer = Vec::with_capacity(pixel_count);
            match src.format {
                gltf::image::Format::R8 => {
                    for b in src.pixels {
                        buffer.push(u32::from_le_bytes([b, b, b, 0xFF]));
                    }
                }
                gltf::image::Format::R8G8B8 => {
                    for i in 0..pixel_count {
                        let [r, g, b] = src.pixels[i * 3..i * 3 + 3].try_into().unwrap();
                        buffer.push(u32::from_le_bytes([b, g, r, 0xFF]));
                    }
                }
                gltf::image::Format::R8G8B8A8 => {
                    for i in 0..pixel_count {
                        let [r, g, b, a] = src.pixels[i * 4..i * 4 + 4].try_into().unwrap();
                        buffer.push(u32::from_le_bytes([b, g, r, a]));
                    }
                }
                format => panic!("unsupported image format: {:?}", format),
            }
            output.push(Image::from_base_image(
                buffer,
                [src.width as usize, src.height as usize],
                mip_levels,
            ));
        }
        return output;
    }

    let (document, buffers, images) = gltf::import(path)?;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for node in document.nodes() {
        process_node(
            node,
            Mat4::identity(),
            &buffers,
            &into_vertex,
            &mut vertices,
            &mut indices,
        );
    }
    let textures = process_textures(images, mip_levels);
    return Ok(Model {
        vertices,
        indices,
        textures,
    });
}

fn read_u8_view(
    accessor: &gltf::Accessor<'_>,
    view: &gltf::buffer::View<'_>,
    data: &[u8],
    output: &mut dyn FnMut(u8),
) {
    assert_eq!(accessor.dimensions(), Dimensions::Scalar);
    assert_eq!(accessor.data_type(), DataType::U8);
    let stride = view.stride().unwrap_or(1);

    for i in 0..accessor.count() {
        output(data[i * stride]);
    }
}

fn read_u16_view(
    accessor: &gltf::Accessor<'_>,
    view: &gltf::buffer::View<'_>,
    data: &[u8],
    output: &mut dyn FnMut(u16),
) {
    assert_eq!(accessor.dimensions(), Dimensions::Scalar);
    assert_eq!(accessor.data_type(), DataType::U16);
    let stride = view.stride().unwrap_or(2);

    for i in 0..accessor.count() {
        let p = i * stride;
        output(u16::from_le_bytes(data[p..p + 2].try_into().unwrap()));
    }
}

fn read_u32_view(
    accessor: &gltf::Accessor<'_>,
    view: &gltf::buffer::View<'_>,
    data: &[u8],
    output: &mut dyn FnMut(u32),
) {
    assert_eq!(accessor.dimensions(), Dimensions::Scalar);
    assert_eq!(accessor.data_type(), DataType::U32);
    let stride = view.stride().unwrap_or(4);

    for i in 0..accessor.count() {
        let p = i * stride;
        output(u32::from_le_bytes(data[p..p + 4].try_into().unwrap()));
    }
}

fn read_vec4_u8_view(
    accessor: &gltf::Accessor<'_>,
    view: &gltf::buffer::View<'_>,
    data: &[u8],
    output: &mut dyn FnMut(u8, u8, u8, u8),
) {
    assert_eq!(accessor.dimensions(), Dimensions::Vec4);
    assert_eq!(accessor.data_type(), DataType::U8);
    let stride = view.stride().unwrap_or(4);

    for i in 0..accessor.count() {
        let p = i * stride;
        let s = &data[p..p + 4];

        output(s[0], s[1], s[2], s[3]);
    }
}

fn read_vec3_u8_view(
    accessor: &gltf::Accessor<'_>,
    view: &gltf::buffer::View<'_>,
    data: &[u8],
    output: &mut dyn FnMut(u8, u8, u8),
) {
    assert_eq!(accessor.dimensions(), Dimensions::Vec3);
    assert_eq!(accessor.data_type(), DataType::U8);
    let stride = view.stride().unwrap_or(3);

    for i in 0..accessor.count() {
        let p = i * stride;
        let s = &data[p..p + 3];

        output(s[0], s[1], s[2]);
    }
}

fn read_vec4_f32_view(
    accessor: &gltf::Accessor<'_>,
    view: &gltf::buffer::View<'_>,
    data: &[u8],
    output: &mut dyn FnMut(f32, f32, f32, f32),
) {
    assert_eq!(accessor.dimensions(), Dimensions::Vec4);
    assert_eq!(accessor.data_type(), DataType::F32);
    let stride = view.stride().unwrap_or(16);

    for i in 0..accessor.count() {
        let p = i * stride;
        let s = &data[p..p + 16];

        output(
            f32::from_le_bytes(s[0..4].try_into().unwrap()),
            f32::from_le_bytes(s[4..8].try_into().unwrap()),
            f32::from_le_bytes(s[8..12].try_into().unwrap()),
            f32::from_le_bytes(s[12..16].try_into().unwrap()),
        );
    }
}

fn read_vec3_f32_view(
    accessor: &gltf::Accessor<'_>,
    view: &gltf::buffer::View<'_>,
    data: &[u8],
    output: &mut dyn FnMut(f32, f32, f32),
) {
    assert_eq!(accessor.dimensions(), Dimensions::Vec3);
    assert_eq!(accessor.data_type(), DataType::F32);
    let stride = view.stride().unwrap_or(12);

    for i in 0..accessor.count() {
        let p = i * stride;
        let s = &data[p..p + 12];

        output(
            f32::from_le_bytes(s[0..4].try_into().unwrap()),
            f32::from_le_bytes(s[4..8].try_into().unwrap()),
            f32::from_le_bytes(s[8..12].try_into().unwrap()),
        );
    }
}

fn read_vec2_f32_view(
    accessor: &gltf::Accessor<'_>,
    view: &gltf::buffer::View<'_>,
    data: &[u8],
    output: &mut dyn FnMut(f32, f32),
) {
    assert_eq!(accessor.dimensions(), Dimensions::Vec2);
    assert_eq!(accessor.data_type(), DataType::F32);
    let stride = view.stride().unwrap_or(8);

    for i in 0..accessor.count() {
        let p = i * stride;
        let s = &data[p..p + 8];

        output(
            f32::from_le_bytes(s[0..4].try_into().unwrap()),
            f32::from_le_bytes(s[4..8].try_into().unwrap()),
        );
    }
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
