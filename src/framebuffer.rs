use minifb::Window;

use crate::{
    color::{to_raw_color, Color},
    image::Image,
    types::{Coords2D, Dimens2D},
};

pub struct Framebuffer {
    color_attachment: Image<u32>,
    depth_attachment: Image<f32>,
}

impl Framebuffer {
    pub fn new([width, height]: Dimens2D) -> Self {
        Self {
            color_attachment: Image::filled(0, [width, height], 1),
            depth_attachment: Image::filled(std::f32::INFINITY, [width, height], 1),
        }
    }

    pub fn clear(&mut self, color: Color, depth: f32) {
        self.color_attachment.fill(to_raw_color(color));
        self.depth_attachment.fill(depth);
    }

    pub fn test_and_set_depth_safe(&mut self, coords: Coords2D, depth: f32) -> bool {
        if !self.contains(coords) {
            return false;
        }
        return self.test_and_set_depth(coords, depth);
    }

    pub fn test_and_set_depth(&mut self, coords: Coords2D, depth: f32) -> bool {
        let target = &mut self.depth_attachment[coords];
        if depth < *target {
            *target = depth;
            return true;
        }
        return false;
    }

    pub fn set_color(&mut self, coords: Coords2D, color: Color) {
        self.color_attachment.set_color(coords, color);
    }

    pub fn set_color_safe(&mut self, coords: Coords2D, color: Color) {
        if !self.contains(coords) {
            return;
        }
        self.set_color(coords, color);
    }

    pub fn get_color(&self, coords: Coords2D) -> Color {
        self.color_attachment.get_color(coords)
    }

    pub fn update_window(&self, window: &mut Window) {
        window
            .update_with_buffer(
                &self.color_attachment.buffer_at_lod(0),
                self.width(),
                self.height(),
            )
            .unwrap();
    }

    pub fn width(&self) -> usize {
        self.color_attachment.width()
    }

    pub fn height(&self) -> usize {
        self.color_attachment.height()
    }

    pub fn contains(&self, coords: Coords2D) -> bool {
        self.color_attachment.contains(coords)
    }
}
