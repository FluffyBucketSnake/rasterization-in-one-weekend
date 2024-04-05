use std::ops::RangeBounds;

use minifb::Window;

use crate::color::{from_raw_color, to_raw_color, Color};

pub struct Framebuffer {
    color_attachment: Vec<u32>,
    depth_attachment: Vec<f32>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            color_attachment: vec![0; width * height],
            depth_attachment: vec![std::f32::INFINITY; width * height],
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: Color, depth: f32) {
        self.color_attachment.fill(to_raw_color(color));
        self.depth_attachment.fill(depth);
    }

    pub fn test_and_set_depth_safe(&mut self, coords: (usize, usize), depth: f32) -> bool {
        if !self.contains(coords) {
            return false;
        }
        return self.test_and_set_depth(coords, depth);
    }

    pub fn test_and_set_depth(&mut self, coords: (usize, usize), depth: f32) -> bool {
        let target = &mut self.depth_attachment[coords_to_index(coords, self.width)];
        if depth < *target {
            *target = depth;
            return true;
        }
        return false;
    }

    pub fn set_color(&mut self, coords: (usize, usize), color: Color) {
        self.color_attachment[coords_to_index(coords, self.width)] = to_raw_color(color);
    }

    pub fn set_color_safe(&mut self, coords: (usize, usize), color: Color) {
        if !self.contains(coords) {
            return;
        }
        self.set_color(coords, color);
    }

    pub fn get_color(&self, coords: (usize, usize)) -> Color {
        from_raw_color(self.color_attachment[coords_to_index(coords, self.width)])
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

    pub fn contains(&self, coords: (usize, usize)) -> bool {
        coords.0 < self.width && coords.1 < self.height
    }
}

fn coords_to_index(coords: (usize, usize), width: usize) -> usize {
    coords.1 * width + coords.0
}
