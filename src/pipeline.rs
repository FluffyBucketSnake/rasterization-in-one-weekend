use nalgebra_glm::Mat4;

use crate::{
    framebuffer::Framebuffer, rasterization::rasterize_solid_triangle, vertex::Vertex,
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

    pub fn draw_triangle<V: Vertex>(&self, framebuffer: &mut Framebuffer, vertices: &[V; 3]) {
        let coords = [0, 1, 2]
            .map(|i| vertices[i].coords())
            .map(|c| self.transform * c)
            .map(|c| self.viewport.ndc_to_framebuffer(c.xy()));
        rasterize_solid_triangle(&coords, |coords, uvw| {
            framebuffer.set_color_safe(
                (coords.x as usize, coords.y as usize),
                Vertex::fragment_color(vertices, uvw),
            )
        });
    }
}
