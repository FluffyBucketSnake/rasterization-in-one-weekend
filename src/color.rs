#![allow(unused)]

use nalgebra_glm::Vec3;

pub type Color = Vec3;

pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);
pub const RED: Color = Color::new(1.0, 0.0, 0.0);
pub const GREEN: Color = Color::new(0.0, 1.0, 0.0);
pub const BLUE: Color = Color::new(0.0, 0.0, 1.0);

pub fn to_raw_color(color: Color) -> u32 {
    let r = (color.x.clamp(0.0, 1.0) * 255.999) as u32;
    let g = (color.y.clamp(0.0, 1.0) * 255.999) as u32;
    let b = (color.z.clamp(0.0, 1.0) * 255.999) as u32;
    (r << 16) | (g << 8) | b
}

pub fn from_raw_color(raw: u32) -> Color {
    let r = ((raw >> 16) & 0xFF) as f32 / 255.0;
    let g = ((raw >> 8) & 0xFF) as f32 / 255.0;
    let b = (raw & 0xFF) as f32 / 255.0;
    Color::new(r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn color_to_raw() {
        assert_eq!(to_raw_color(BLACK), 0x00000000);
        assert_eq!(to_raw_color(WHITE), 0x00FFFFFF);
        assert_eq!(to_raw_color(RED), 0x00FF0000);
        assert_eq!(to_raw_color(GREEN), 0x0000FF00);
        assert_eq!(to_raw_color(BLUE), 0x000000FF);
    }

    #[test]
    pub fn raw_to_color() {
        assert_eq!(from_raw_color(0x00000000), BLACK);
        assert_eq!(from_raw_color(0x00FFFFFF), WHITE);
        assert_eq!(from_raw_color(0x00FF0000), RED);
        assert_eq!(from_raw_color(0x0000FF00), GREEN);
        assert_eq!(from_raw_color(0x000000FF), BLUE);
    }
}
