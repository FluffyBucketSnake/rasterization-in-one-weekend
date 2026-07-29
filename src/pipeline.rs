use std::{fmt::Debug, marker::PhantomData};

use nalgebra_glm::Vec4;

use crate::{
    clipping::clip_triangle,
    framebuffer::Framebuffer,
    math::bary_lerp,
    rasterization::rasterize_solid_triangle,
    shaders::{FragmentData, FragmentShader, VertexShader},
    triangulation::fan_triangulate,
    viewport::Viewport,
};

#[derive(Debug)]
pub struct RasterizationPipeline<
    U,
    VSIn: Copy,
    FSIn: FragmentData,
    VS: VertexShader<U, VSIn, Fragment = FSIn>,
    FS: FragmentShader<U, FSIn>,
> {
    vertex_shader: VS,
    fragment_shader: FS,
    viewport: Viewport,
    _uniforms: PhantomData<U>,
    _vs_input: PhantomData<VSIn>,
    _fs_input: PhantomData<FSIn>,
}

impl<
        U,
        VSIn: Copy,
        FSIn: FragmentData,
        VS: VertexShader<U, VSIn, Fragment = FSIn>,
        FS: FragmentShader<U, FSIn>,
    > RasterizationPipeline<U, VSIn, FSIn, VS, FS>
{
    pub fn new(vertex_shader: VS, fragment_shader: FS, viewport: Viewport) -> Self {
        Self {
            vertex_shader,
            fragment_shader,
            viewport,
            _uniforms: PhantomData,
            _vs_input: PhantomData,
            _fs_input: PhantomData,
        }
    }

    pub fn draw_triangle(&self, triangle: &[VSIn; 3], uniforms: &U, framebuffer: &mut Framebuffer) {
        let clip_space_triangle = triangle.map(|v| self.vertex_shader.vs(uniforms, &v));
        let clipped_polygon = clip_triangle(&clip_space_triangle);
        let clipped_triangles = fan_triangulate(&clipped_polygon);
        let primitive_count = clipped_triangles.len() / 3;
        for i in 0..primitive_count {
            let (mut p0, f0) = clipped_triangles[3 * i];
            let (mut p1, f1) = clipped_triangles[3 * i + 1];
            let (mut p2, f2) = clipped_triangles[3 * i + 2];

            Self::homogenize_coords(&mut p0);
            Self::homogenize_coords(&mut p1);
            Self::homogenize_coords(&mut p2);

            let screen_coords = [p0, p1, p2].map(|p| self.viewport.ndc_to_framebuffer(p.xy()));
            rasterize_solid_triangle(&screen_coords, |frag| {
                let screen_coords = [frag.coords.x, frag.coords.y];
                let p = bary_lerp(p0, p1, p2, frag.t);
                if framebuffer.test_and_set_depth_safe(screen_coords, p.z) {
                    let f = f0.bary_lerp(p0.w, &f1, p1.w, &f2, p2.w, frag.t);
                    let df_dx = f0.derivate(p0.w, &f1, p1.w, &f2, p2.w, frag.t, frag.dt_dx);
                    let df_dy = f0.derivate(p0.w, &f1, p1.w, &f2, p2.w, frag.t, frag.dt_dy);

                    framebuffer.set_color(
                        screen_coords,
                        self.fragment_shader
                            .fs(uniforms, &frag, &f, &[df_dx, df_dy]),
                    );
                }
            });
        }
    }

    pub fn draw_triangles(&self, vertices: &[VSIn], uniforms: &U, framebuffer: &mut Framebuffer) {
        let primitive_count = vertices.len() / 3;

        for i in 0..primitive_count {
            let triangle = [0, 1, 2].map(|j| vertices[3 * i + j]);
            self.draw_triangle(&triangle, uniforms, framebuffer);
        }
    }

    pub fn draw_triangles_indexed(
        &self,
        indices: &[usize],
        vertices: &[VSIn],
        uniforms: &U,
        framebuffer: &mut Framebuffer,
    ) {
        let primitive_count = indices.len() / 3;

        for i in 0..primitive_count {
            let triangle = [0, 1, 2]
                .map(|j| indices[3 * i + j])
                .map(|idx| vertices[idx]);
            self.draw_triangle(&triangle, uniforms, framebuffer);
        }
    }

    fn homogenize_coords(coords: &mut Vec4) {
        let w_inv = 1.0 / coords.w;
        *coords *= w_inv;
        coords.w = w_inv;
    }
}
