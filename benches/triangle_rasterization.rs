use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nalgebra_glm::{vec2, Vec2};
use rasterization_in_a_weekend::{
    color::{Color, WHITE},
    framebuffer::Framebuffer,
    rasterization::fill_triangle,
    vertex::Vertex,
};

#[derive(Debug, Clone)]
struct DummyVertex(Vec2);

impl Vertex for DummyVertex {
    #[inline]
    fn fragment_color(_: &[Self; 3], _: f32, _: f32) -> Color {
        WHITE
    }

    #[inline]
    fn coords(&self) -> &Vec2 {
        &self.0
    }
}

fn vertex(x: f32, y: f32) -> DummyVertex {
    DummyVertex(vec2(x, y))
}

pub fn triangle_rasterization_benchmarks(c: &mut Criterion) {
    let vertices = [
        vertex(480.0, 180.0),
        vertex(160.0, 90.0),
        vertex(160.0, 270.0),
    ];
    c.bench_function("barycentric", |b| {
        b.iter(|| fill_triangle(&mut Framebuffer::new(640, 360), black_box(&vertices)))
    });
}

criterion_group!(benches, triangle_rasterization_benchmarks);
criterion_main!(benches);
