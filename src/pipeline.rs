use nalgebra_glm::{Mat4, Vec3, Vec4};

use crate::{
    clipping::clip_triangle, color::WHITE, framebuffer::Framebuffer,
    rasterization::rasterize_solid_triangle, triangulation::fan_triangulate, vertex::Vertex,
    viewport::Viewport,
};

#[derive(Debug)]
pub struct RasterizationPipeline {
    transform: Mat4,
    viewport: Viewport,
}

impl RasterizationPipeline {
    pub fn new(transform: Mat4, viewport: Viewport) -> Self {
        Self {
            transform,
            viewport,
        }
    }

    pub fn transform(&self) -> &Mat4 {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Mat4) {
        self.transform = transform;
    }

    pub fn draw_triangles<V: Vertex>(&self, framebuffer: &mut Framebuffer, vertices: &[V]) {
        let primitive_count = vertices.len() / 3;
        let mut clip_coords = Vec::new();
        let mut clip_colors = Vec::new();
        for i in 0..primitive_count {
            let coords = [0, 1, 2]
                .map(|j| vertices[3 * i + j].coords())
                .map(|c| self.transform * c);
            let colors = [0, 1, 2].map(|j| vertices[3 * i + j].color());
            let (coords, weights) = fan_triangulate(&clip_triangle(coords))
                .into_iter()
                .unzip::<Vec4, Vec3, Vec<_>, Vec<_>>();
            clip_coords.extend(coords);
            clip_colors.extend(
                weights
                    .into_iter()
                    .map(|w| w.x * colors[0] + w.y * colors[1] + w.z * colors[2]),
            );
        }

        let primitive_count = clip_coords.len() / 3;
        for j in 0..primitive_count {
            let ndc_coords = [0, 1, 2].map(|i| clip_coords[j * 3 + i]).map(|c| c / c.w);
            let screen_coords = ndc_coords
                .clone()
                .map(|c| self.viewport.ndc_to_framebuffer(c.xy()));
            let colors = [0, 1, 2].map(|i| clip_colors[j * 3 + i]);
            rasterize_solid_triangle(&screen_coords, |screen_coords, uvw| {
                let screen_coords = (screen_coords.x as usize, screen_coords.y as usize);
                let z = uvw.x * ndc_coords[0].z + uvw.y * ndc_coords[1].z + uvw.z * ndc_coords[2].z;
                if framebuffer.test_and_set_depth_safe(screen_coords, z) {
                    framebuffer.set_color(
                        screen_coords,
                        uvw.x * colors[0] + uvw.y * colors[1] + uvw.z * colors[2],
                        // 100.0 * (1.0 - z) * WHITE,
                    );
                }
            });
        }
    }
}
