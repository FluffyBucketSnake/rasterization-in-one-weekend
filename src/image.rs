use std::{
    ops::{Index, IndexMut},
    path::PathBuf,
};

use image::{Rgba, RgbaImage};

use crate::{
    color::{from_raw_color, to_raw_color, Color},
    types::{Coords2D, Dimens2D, Dimens3D},
};

pub struct Image<T> {
    mip_chain: Vec<Vec<T>>,
    width_0: usize,
    height_0: usize,
}

impl<T> Image<T> {
    pub fn from_raw_parts(mip_chain: Vec<Vec<T>>, [width_0, height_0]: Dimens2D) -> Self {
        for (l, buffer) in mip_chain.iter().enumerate() {
            assert_eq!(
                buffer_size_at_lod([width_0, height_0], l),
                buffer.len(),
                "buffer at mip level #{} has incompatible size",
                l,
            );
        }
        return Self {
            mip_chain,
            width_0,
            height_0,
        };
    }

    pub fn filled(value: T, dimensions: Dimens2D, mip_levels: usize) -> Self
    where
        T: Copy,
    {
        let mut mip_chain = Vec::with_capacity(mip_levels);
        for lod in 0..mip_levels {
            mip_chain.push(vec![value; buffer_size_at_lod(dimensions, lod)])
        }
        return Self::from_raw_parts(mip_chain, dimensions);
    }

    pub fn fill(&mut self, value: T)
    where
        T: Copy,
    {
        for buffer in &mut self.mip_chain {
            buffer.fill(value);
        }
    }

    pub fn buffer_at_lod(&self, lod: usize) -> &[T] {
        &self.mip_chain[lod][..]
    }

    pub fn width(&self) -> usize {
        self.width_0
    }

    pub fn height(&self) -> usize {
        self.height_0
    }

    pub fn dimens(&self) -> Dimens2D {
        [self.width_0, self.height_0]
    }

    pub fn width_at_lod(&self, lod: usize) -> usize {
        dimen_at_lod(self.width_0, lod)
    }

    pub fn height_at_lod(&self, lod: usize) -> usize {
        dimen_at_lod(self.height_0, lod)
    }

    pub fn contains(&self, [x, y]: Coords2D) -> bool {
        x < self.width_0 && y < self.height_0
    }

    pub fn contains_at_lod(&self, [x, y]: Coords2D, lod: usize) -> bool {
        x < self.width_at_lod(lod) && y < self.height_at_lod(lod)
    }

    pub fn mip_levels(&self) -> usize {
        self.mip_chain.len()
    }

    pub fn dimens_at_lod(&self, lod: usize) -> [usize; 2] {
        dimens_at_lod(self.dimens(), lod)
    }
}

impl Image<u32> {
    pub fn from_file(path: PathBuf, mip_levels: usize) -> image::ImageResult<Self> {
        assert!(mip_levels > 0, "needs at least 1 mip level");
        let image = image::open(path)?.to_rgba8();
        let width_0 = image.width() as usize;
        let height_0 = image.height() as usize;

        let mut width = width_0 as u32;
        let mut height = height_0 as u32;
        let mut buffers = Vec::with_capacity(mip_levels);
        for _ in 0..mip_levels {
            let resized_image = image::imageops::resize(
                &image,
                width,
                height,
                image::imageops::FilterType::Gaussian,
            );
            let buffer = resized_image
                .pixels()
                .map(|Rgba(c)| c.map(|c| c as u32))
                .map(|[r, g, b, a]| (a << 24) | (r << 16) | (g << 8) | b)
                .collect::<Vec<_>>();
            buffers.push(buffer);
            width = (width / 2).max(1);
            height = (height / 2).max(1);
        }
        return Ok(Self::from_raw_parts(buffers, [width_0, height_0]));
    }

    pub fn from_base_image(
        base: Vec<u32>,
        [width_0, height_0]: Dimens2D,
        mip_levels: usize,
    ) -> Self {
        assert!(mip_levels > 0, "needs at least 1 mip level");

        let mut width = width_0 as u32;
        let mut height = height_0 as u32;

        let base_image_view = RgbaImage::from_fn(width, height, |x, y| {
            let [b, g, r, a] = base[y as usize * width_0 + x as usize].to_le_bytes();
            Rgba([r, g, b, a])
        });
        let mut buffers = Vec::with_capacity(mip_levels);
        buffers.push(base.clone());
        for _ in 1..mip_levels {
            width = (width / 2).max(1);
            height = (height / 2).max(1);
            let resized_image = image::imageops::resize(
                &base_image_view,
                width,
                height,
                image::imageops::FilterType::Gaussian,
            );
            let buffer = resized_image
                .pixels()
                .map(|Rgba(c)| c.map(|c| c as u32))
                .map(|[r, g, b, a]| (a << 24) | (r << 16) | (g << 8) | b)
                .collect::<Vec<_>>();
            buffers.push(buffer);
        }
        return Self::from_raw_parts(buffers, [width_0, height_0]);
    }

    pub fn get_color(&self, coords: Coords2D) -> Color {
        from_raw_color(self[coords])
    }

    pub fn get_color_at_lod(&self, coords: Coords2D, lod: usize) -> Color {
        from_raw_color(self[(coords, lod)])
    }

    pub fn set_color(&mut self, coords: Coords2D, color: Color) {
        self[coords] = to_raw_color(color);
    }
}

impl<T> Index<(Coords2D, usize)> for Image<T> {
    type Output = T;

    fn index(&self, (coords, lod): (Coords2D, usize)) -> &Self::Output {
        let dimens = self.dimens_at_lod(lod);
        &self.mip_chain[lod][coords_to_idx(coords, dimens)]
    }
}

impl<T> Index<Coords2D> for Image<T> {
    type Output = T;

    fn index(&self, coords: Coords2D) -> &Self::Output {
        &self[(coords, 0)]
    }
}

impl<T> IndexMut<(Coords2D, usize)> for Image<T> {
    fn index_mut(&mut self, (coords, lod): (Coords2D, usize)) -> &mut Self::Output {
        let dimens = self.dimens_at_lod(lod);
        &mut self.mip_chain[lod][coords_to_idx(coords, dimens)]
    }
}

impl<T> IndexMut<Coords2D> for Image<T> {
    fn index_mut(&mut self, coords: Coords2D) -> &mut Self::Output {
        &mut self[(coords, 0)]
    }
}

fn coords_to_idx([x, y]: Coords2D, [width, _]: Dimens2D) -> usize {
    return y * width + x;
}

fn dimen_at_lod(dimen_0: usize, lod: usize) -> usize {
    let mut dimen = dimen_0;
    for _ in 0..lod {
        dimen = (dimen / 2).max(1);
    }
    return dimen;
}

fn dimens_at_lod([width_0, height_0]: Dimens2D, lod: usize) -> Dimens2D {
    return [dimen_at_lod(width_0, lod), dimen_at_lod(height_0, lod)];
}

fn buffer_size_at_lod(dimens_0: Dimens2D, lod: usize) -> usize {
    let [width, height] = dimens_at_lod(dimens_0, lod);
    return width * height;
}

fn estimate_total_image_size([width_0, height_0, mip_levels]: Dimens3D) -> usize {
    let mut total_size = 0;
    let mut width = width_0;
    let mut height = height_0;

    for _ in 0..mip_levels {
        total_size += width + height;
        width = (width / 2).max(1);
        height = (height / 2).max(1);
    }
    return total_size;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use crate::color::{BLUE, GREEN, RED, WHITE};

    #[test]
    fn load_simple_image() {
        let mut image_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        image_path.push("textures/simple.png");

        let image = Image::from_file(image_path, 2).unwrap();

        assert_eq!(image.get_color([0, 0]), WHITE);
        assert_eq!(image.get_color([1, 0]), RED);
        assert_eq!(image.get_color([0, 1]), GREEN);
        assert_eq!(image.get_color([1, 1]), BLUE);
    }
}
