use nalgebra_glm::{vec2, IVec2, Vec2};

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
}

pub struct Sampler {
    u_address_mode: AddressMode,
    v_address_mode: AddressMode,
    filter: Filter,
}

impl Sampler {
    pub fn new(u_address_mode: AddressMode, v_address_mode: AddressMode, filter: Filter) -> Self {
        Self {
            u_address_mode,
            v_address_mode,
            filter,
        }
    }

    pub fn sample(&self, image: &Image, uv: Vec2) -> Color {
        let rs = uv.component_mul(&vec2(image.width(), image.height()).cast());
        match self.filter {
            Filter::Nearest => {
                let ij = nalgebra_glm::floor(&rs).try_cast().unwrap();
                return self.sample_texel(image, ij);
            }
            Filter::Linear => {
                let rs = rs - vec2(0.5, 0.5);
                let a = nalgebra_glm::fract(&rs);
                let ij0 = nalgebra_glm::floor(&rs).try_cast().unwrap();
                let samples = [
                    (ij0 + vec2(0, 0), 1.0 - a.x, 1.0 - a.y),
                    (ij0 + vec2(1, 0), a.x, 1.0 - a.y),
                    (ij0 + vec2(0, 1), 1.0 - a.x, a.y),
                    (ij0 + vec2(1, 1), a.x, a.y),
                ];
                return samples
                    .map(|(ij, w_i, w_j)| w_i * w_j * self.sample_texel(image, ij))
                    .into_iter()
                    .sum();
            }
        }
    }

    fn sample_texel(&self, image: &Image, ij: IVec2) -> Color {
        let i = self.u_address_mode.convert(ij.x, image.width());
        let j = self.v_address_mode.convert(ij.y, image.height());
        return image.get_color((i, j));
    }
}
