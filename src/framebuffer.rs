use minifb::Window;

use crate::color::{to_raw_color, Color};

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

    pub fn update_window(&self, window: &mut Window) {
        window
            .update_with_buffer(&self.color_attachment, self.width, self.height)
            .unwrap();
    }
}
