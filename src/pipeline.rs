use std::marker::PhantomData;

use nalgebra_glm::{Mat4, Vec3, Vec4};

use crate::{
    clipping::clip_triangle,
    framebuffer::Framebuffer,
    rasterization::rasterize_solid_triangle,
    shaders::{Fragment, FragmentShader, VertexShader},
    triangulation::fan_triangulate,
    viewport::Viewport,
};

#[derive(Debug)]
pub struct RasterizationPipeline<I, F, U, VS, FS> {
    viewport: Viewport,
    vertex_shader: VS,
    fragment_shader: FS,
    _input: PhantomData<I>,
    _uniforms: PhantomData<U>,
    _fragment: PhantomData<F>,
}

impl<I, F, U, VS, FS> RasterizationPipeline<I, F, U, VS, FS>
where
    F: Fragment,
    VS: VertexShader<I, U, Output = F>,
    FS: FragmentShader<F, U>,
{
    pub fn new(viewport: Viewport, vertex_shader: VS, fragment_shader: FS) -> Self {
        Self {
            viewport,
            vertex_shader,
            fragment_shader,
            _input: Default::default(),
            _uniforms: Default::default(),
            _fragment: Default::default(),
        }
    }

    pub fn draw_triangles(&self, framebuffer: &mut Framebuffer, uniforms: &U, vertices: &[I]) {
        let input_triangles = vertices
            .chunks_exact(3)
            .map::<&[I; 3], _>(|c| c.try_into().unwrap());
        let (clip_coordinates, clip_fragments) = input_triangles
            .flat_map(|t| [0, 1, 2].map(|i| self.vertex_shader.vs(&t[i], &uniforms)))
            .unzip::<_, _, Vec<_>, Vec<_>>();
        let (ndc_coordinates, fragments) = clip_coordinates
            .chunks_exact(3)
            .map::<&[Vec4; 3], _>(|c| c.try_into().unwrap())
            .zip(
                clip_fragments
                    .chunks_exact(3)
                    .map::<&[F; 3], _>(|f| f.try_into().unwrap()),
            )
            .flat_map(|(coords, fragments)| {
                fan_triangulate(&clip_triangle(coords))
                    .into_iter()
                    .map(|(c, w)| (c / c.w, F::interpolate(fragments, w)))
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();
        let ndc_triangles = ndc_coordinates
            .chunks_exact(3)
            .map::<&[_; 3], _>(|c| c.try_into().unwrap())
            .zip(
                fragments
                    .chunks_exact(3)
                    .map::<&[_; 3], _>(|c| c.try_into().unwrap()),
            );

        for (ndc_coords, fragments) in ndc_triangles {
            let fb_coords = ndc_coords.map(|c| self.viewport.ndc_to_framebuffer(c.xy()));
            rasterize_solid_triangle(&fb_coords, |fb_coords, uvw| {
                let screen_coords = (fb_coords.x as usize, fb_coords.y as usize);
                let z = uvw.x * ndc_coords[0].z + uvw.y * ndc_coords[1].z + uvw.z * ndc_coords[2].z;
                if framebuffer.test_and_set_depth_safe(screen_coords, z) {
                    let fragment = F::interpolate(fragments, uvw);
                    framebuffer.set_color(
                        screen_coords,
                        self.fragment_shader.fs(&fragment, uniforms),
                        // 100.0 * (1.0 - z) * WHITE,
                    );
                }
            });
        }
    }
}
