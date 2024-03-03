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
