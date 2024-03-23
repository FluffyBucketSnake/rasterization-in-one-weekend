use std::ops::RangeBounds;

use minifb::Window;

use crate::color::{from_raw_color, to_raw_color, Color};

pub struct Framebuffer {
    color_attachment: Vec<u32>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            color_attachment: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.color_attachment.fill(to_raw_color(color));
    }

    pub fn set_color(&mut self, coords: (usize, usize), color: Color) {
        self.color_attachment[coords.1 * self.width + coords.0] = to_raw_color(color);
    }

    pub fn set_color_safe(&mut self, coords: (usize, usize), color: Color) {
        match coords {
            (x, y) if x < self.width && y < self.height => {
                self.set_color(coords, color);
            }
            _ => {}
        }
    }

    pub fn get_color(&self, coords: (usize, usize)) -> Color {
        from_raw_color(self.color_attachment[coords.1 * self.width + coords.0])
    }

    pub fn update_window(&self, window: &mut Window) {
        window
            .update_with_buffer(&self.color_attachment, self.width, self.height)
            .unwrap();
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
