use nalgebra_glm::{vec2, IVec2, Vec2};
use num::Float;

use crate::{color::Color, image::Image};

pub enum AddressMode {
    Repeat,
    Clamp,
}

impl AddressMode {
    pub fn convert(&self, src: i32, size: usize) -> usize {
        (match self {
            AddressMode::Repeat => src % size as i32,
            AddressMode::Clamp => src.clamp(0, (size - 1) as i32),
        }) as usize
    }
}

pub enum Filter {
    Nearest,
    Linear,
    Anisotropic(i32),
}

pub struct Sampler {
    u_address_mode: AddressMode,
    v_address_mode: AddressMode,
    min_filter: Filter,
    mag_filter: Filter,
}

impl Sampler {
    pub fn new(
        u_address_mode: AddressMode,
        v_address_mode: AddressMode,
        min_filter: Filter,
        mag_filter: Filter,
    ) -> Self {
        Self {
            u_address_mode,
            v_address_mode,
            min_filter,
            mag_filter,
        }
    }

    pub fn sample(&self, image: &Image<u32>, uv: Vec2, duv_dx: Vec2, duv_dy: Vec2) -> Color {
        let image_scale = vec2(image.width(), image.height()).cast();
        let rs = uv.component_mul(&image_scale);
        let scale_factor = vec2(
            duv_dx.component_mul(&image_scale).norm(),
            duv_dy.component_mul(&image_scale).norm(),
        );
        if scale_factor.min() > 1.0 {
            match self.min_filter {
                Filter::Nearest => {
                    return self.nearest_sample(image, rs);
                }
                Filter::Linear => {
                    return self.linear_sample(image, rs);
                }
                Filter::Anisotropic(l) => {
                    return self.anisotropic_sample(image, rs, scale_factor.min().max(l as f32));
                }
            }
        } else {
            match self.mag_filter {
                Filter::Nearest => {
                    return self.nearest_sample(image, rs);
                }
                Filter::Linear | Filter::Anisotropic(_) => {
                    return self.linear_sample(image, rs);
                }
            }
        }
    }

    fn nearest_sample(&self, image: &Image<u32>, rs: Vec2) -> Color {
        let ij = nalgebra_glm::floor(&rs).try_cast().unwrap();
        return self.sample_texel(image, ij, 0);
    }

    fn linear_sample(&self, image: &Image<u32>, rs: Vec2) -> Color {
        let rs = rs - vec2(0.5, 0.5);
        let a = nalgebra_glm::fract(&rs);
        let ij_0 = nalgebra_glm::floor(&rs).try_cast().unwrap();
        let samples = [
            (ij_0 + vec2(0, 0), 1.0 - a.x, 1.0 - a.y),
            (ij_0 + vec2(1, 0), a.x, 1.0 - a.y),
            (ij_0 + vec2(0, 1), 1.0 - a.x, a.y),
            (ij_0 + vec2(1, 1), a.x, a.y),
        ];
        return samples
            .map(|(ij, w_i, w_j)| w_i * w_j * self.sample_texel(image, ij, 0))
            .into_iter()
            .sum();
    }

    fn anisotropic_sample(&self, image: &Image<u32>, rs: Vec2, lod: f32) -> Color {
        let rs = rs - vec2(0.5, 0.5);
        let a = nalgebra_glm::fract(&rs);
        let b = lod.fract();
        let ij0 = nalgebra_glm::floor(&rs).try_cast().unwrap();
        let lod_0 = lod.floor() as usize;
        let samples = [
            (ij0 + vec2(0, 0), lod_0, 1.0 - a.x, 1.0 - a.y, 1.0 - b),
            (ij0 + vec2(1, 0), lod_0, a.x, 1.0 - a.y, 1.0 - b),
            (ij0 + vec2(0, 1), lod_0, 1.0 - a.x, a.y, 1.0 - b),
            (ij0 + vec2(1, 1), lod_0, a.x, a.y, 1.0 - b),
            (ij0 + vec2(0, 0), lod_0 + 1, 1.0 - a.x, 1.0 - a.y, b),
            (ij0 + vec2(1, 0), lod_0 + 1, a.x, 1.0 - a.y, b),
            (ij0 + vec2(0, 1), lod_0 + 1, 1.0 - a.x, a.y, b),
            (ij0 + vec2(1, 1), lod_0 + 1, a.x, a.y, b),
        ];
        return samples
            .map(|(ij, lod, w_i, w_j, w_k)| w_i * w_j * w_k * self.sample_texel(image, ij, lod))
            .into_iter()
            .sum();
    }

    fn sample_texel(&self, image: &Image<u32>, mut ij: IVec2, lod: usize) -> Color {
        let lod = lod.min(image.mip_levels() - 1);
        let [width, height] = image.dimens_at_lod(lod);
        for _ in 0..lod {
            ij /= 2;
        }
        let i = self.u_address_mode.convert(ij.x, width);
        let j = self.v_address_mode.convert(ij.y, height);
        return image.get_color_at_lod([i, j], lod);
    }
}
