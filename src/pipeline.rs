use nalgebra_glm::Mat4;

use crate::{
    clipping::clip_triangle,
    framebuffer::Framebuffer,
    image::Image,
    rasterization::{rasterize_solid_triangle, Fragment},
    sampler::Sampler,
    triangulation::fan_triangulate,
    vertex::Vertex,
    viewport::Viewport,
};

#[derive(Debug)]
pub struct RasterizationPipeline {
    viewport: Viewport,
}

impl RasterizationPipeline {
    pub fn new(viewport: Viewport) -> Self {
        Self { viewport }
    }

    pub fn draw_triangles(
        &self,
        framebuffer: &mut Framebuffer,
        transform: &Mat4,
        (image, sampler): (&Image, &Sampler),
        vertices: &[Vertex],
    ) {
        let primitive_count = vertices.len() / 3;
        for i in 0..primitive_count {
            let triangle = [0, 1, 2]
                .map(|j| vertices[3 * i + j])
                .map(|v| v.transform(transform));
            let clipped_polygon = clip_triangle(&triangle);
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
                        let screen_coords = (coords.x as usize, coords.y as usize);
                        let Vertex { coords, uv, .. } = v0.bary_lerp(&v1, &v2, t);
                        let duv_dx = v0.duv(&v1, &v2, t, dt_dx);
                        let duv_dy = v0.duv(&v1, &v2, t, dt_dy);
                        if framebuffer.test_and_set_depth_safe(screen_coords, coords.z) {
                            framebuffer.set_color(
                                screen_coords,
                                sampler.sample(image, uv, duv_dx, duv_dy),
                            );
                        }
                    },
                );
            }
        }
    }
}
