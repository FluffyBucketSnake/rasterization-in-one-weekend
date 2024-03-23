use crate::{framebuffer::Framebuffer, rasterization::rasterize_solid_triangle, vertex::Vertex};

pub struct RasterizationPipeline;

impl RasterizationPipeline {
    pub fn draw_triangle<V: Vertex>(&self, framebuffer: &mut Framebuffer, vertices: &[V; 3]) {
        let coords = [0, 1, 2].map(|i| *vertices[i].coords());
        rasterize_solid_triangle(&coords, |coords, uvw| {
            framebuffer.set_color(
                (coords.x as usize, coords.y as usize),
                Vertex::fragment_color(vertices, uvw),
            )
        });
    }
}
