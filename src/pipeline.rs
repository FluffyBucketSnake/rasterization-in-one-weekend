use std::{fmt::Debug, marker::PhantomData};

use nalgebra_glm::Mat4;

use crate::{
    clipping::clip_triangle,
    framebuffer::Framebuffer,
    image::Image,
    rasterization::{rasterize_solid_triangle, Fragment},
    sampler::Sampler,
    shaders::VertexShader,
    triangulation::fan_triangulate,
    vertex::Vertex,
    viewport::Viewport,
};

#[derive(Debug)]
pub struct RasterizationPipeline<U, VSIn: Copy, VS: VertexShader<U, VSIn>> {
    vertex_shader: VS,
    viewport: Viewport,
    _uniforms: PhantomData<U>,
    _vs_input: PhantomData<VSIn>,
}

impl<U, VSIn: Copy, VS: VertexShader<U, VSIn>> RasterizationPipeline<U, VSIn, VS> {
    pub fn new(vertex_shader: VS, viewport: Viewport) -> Self {
        Self {
            vertex_shader,
            viewport,
            _uniforms: PhantomData,
            _vs_input: PhantomData,
        }
    }

    pub fn draw_triangle(
        &self,
        triangle: &[VSIn; 3],
        uniforms: &U,
        (image, sampler): (&Image<u32>, &Sampler),
        framebuffer: &mut Framebuffer,
    ) {
        let clip_space_triangle = triangle.map(|v| self.vertex_shader.vs(uniforms, &v));
        let clipped_polygon = clip_triangle(&clip_space_triangle);
        let clipped_triangles = fan_triangulate(&clipped_polygon);
        let primitive_count = clipped_triangles.len() / 3;
        for i in 0..primitive_count {
            let ndc_triangle = [0, 1, 2]
                .map(|j| clipped_triangles[3 * i + j])
                .map(|v| v.homogenize());
            let screen_coords = [0, 1, 2].map(|i| {
                self.viewport
                    .ndc_to_framebuffer(ndc_triangle[i].coords.xy())
            });
            let [v0, v1, v2] = ndc_triangle;
            rasterize_solid_triangle(
                &screen_coords,
                |Fragment {
                     coords,
                     t,
                     dt_dx,
                     dt_dy,
                 }| {
                    let screen_coords = [coords.x, coords.y];
                    let Vertex {
                        coords, uv, color, ..
                    } = v0.bary_lerp(&v1, &v2, t);
                    let duv_dx = v0.duv(&v1, &v2, t, dt_dx);
                    let duv_dy = v0.duv(&v1, &v2, t, dt_dy);
                    if framebuffer.test_and_set_depth_safe(screen_coords, coords.z) {
                        framebuffer.set_color(
                            screen_coords,
                            color.component_mul(&sampler.sample(image, uv, duv_dx, duv_dy)),
                            // color,
                        );
                    }
                },
            );
        }
    }

    pub fn draw_triangles(
        &self,
        vertices: &[VSIn],
        uniforms: &U,
        image_sampler: (&Image<u32>, &Sampler),
        framebuffer: &mut Framebuffer,
    ) {
        let primitive_count = vertices.len() / 3;

        for i in 0..primitive_count {
            let triangle = [0, 1, 2].map(|j| vertices[3 * i + j]);
            self.draw_triangle(&triangle, uniforms, image_sampler, framebuffer);
        }
    }

    pub fn draw_triangles_indexed(
        &self,
        indices: &[usize],
        vertices: &[VSIn],
        uniforms: &U,
        image_sampler: (&Image<u32>, &Sampler),
        framebuffer: &mut Framebuffer,
    ) {
        let primitive_count = indices.len() / 3;

        for i in 0..primitive_count {
            let triangle = [0, 1, 2]
                .map(|j| indices[3 * i + j])
                .map(|idx| vertices[idx]);
            self.draw_triangle(&triangle, uniforms, image_sampler, framebuffer);
        }
    }
}
