use crate::{
    framebuffer::Framebuffer, rasterization::rasterize_solid_triangle, vertex::Vertex,
    viewport::Viewport,
};

pub struct RasterizationPipeline {
    viewport: Viewport,
}

impl RasterizationPipeline {
    pub fn new(viewport: Viewport) -> Self {
        Self { viewport }
    }

    pub fn draw_triangle<V: Vertex>(&self, framebuffer: &mut Framebuffer, vertices: &[V; 3]) {
        let coords = [0, 1, 2]
            .map(|i| *vertices[i].coords())
            .map(|c| self.viewport.ndc_to_framebuffer(c));
        rasterize_solid_triangle(&coords, |coords, uvw| {
            framebuffer.set_color(
                (coords.x as usize, coords.y as usize),
                Vertex::fragment_color(vertices, uvw),
            )
        });
    }
}
