use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nalgebra_glm::vec2;
use rasterization_in_a_weekend::rasterization::rasterize_solid_triangle;

pub fn triangle_rasterization_benchmarks(c: &mut Criterion) {
    let vertices = [vec2(480.0, 180.0), vec2(160.0, 90.0), vec2(160.0, 270.0)];
    c.bench_function("barycentric", |b| {
        b.iter(|| rasterize_solid_triangle(black_box(&vertices), black_box(|_| {})))
    });
}

criterion_group!(benches, triangle_rasterization_benchmarks);
criterion_main!(benches);
