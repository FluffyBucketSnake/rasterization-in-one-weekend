use std::path::PathBuf;

use image::Rgba;

use crate::color::{from_raw_color, Color};

pub type Coords2D = (usize, usize);

pub struct Image {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn from_buffer(buffer: Vec<u32>, width: usize, height: usize) -> Self {
        assert_eq!(width * height, buffer.len());
        Self {
            buffer,
            width,
            height,
        }
    }

    pub fn from_file(path: PathBuf) -> image::ImageResult<Self> {
        let image = image::open(path)?.to_rgba8();
        let width = image.width() as usize;
        let height = image.height() as usize;
        Ok(Self::from_buffer(
            image
                .pixels()
                .map(|Rgba(c)| c.map(|c| c as u32))
                .map(|[r, g, b, a]| (a << 24) | (r << 16) | (g << 8) | b)
                .collect(),
            width,
            height,
        ))
    }

    pub fn get_color(&self, coords: Coords2D) -> Color {
        assert!(self.contains(coords));
        from_raw_color(self.buffer[map_coords_to_index(coords, self.width)])
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn contains(&self, coords: (usize, usize)) -> bool {
        coords.0 < self.width && coords.1 < self.height
    }
}

pub fn map_coords_to_index(coords: Coords2D, width: usize) -> usize {
    coords.1 * width + coords.0
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

        let image = Image::from_file(image_path).unwrap();

        assert_eq!(image.get_color((0, 0)), WHITE);
        assert_eq!(image.get_color((1, 0)), RED);
        assert_eq!(image.get_color((0, 1)), GREEN);
        assert_eq!(image.get_color((1, 1)), BLUE);
    }
}
