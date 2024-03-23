use nalgebra_glm::Vec2;

#[derive(Debug)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Viewport {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn full(width: f32, height: f32) -> Self {
        Self::new(0.0, 0.0, width, height)
    }

    pub fn ndc_to_framebuffer(&self, src: Vec2) -> Vec2 {
        Vec2::new(
            (src.x + 1.0) * self.width / 2.0 + self.x,
            (src.y + 1.0) * self.height / 2.0 + self.y,
        )
    }
}

#[cfg(test)]
mod tests {
    use nalgebra_glm::vec2;

    use super::*;

    #[test]
    pub fn ndc_to_framebuffer() {
        let viewport = Viewport::new(160.0, 80.0, 640.0, 480.0);
        assert_eq!(
            viewport.ndc_to_framebuffer(vec2(-1.0, -1.0)),
            vec2(160.0, 80.0)
        );
        assert_eq!(
            viewport.ndc_to_framebuffer(vec2(0.0, 0.0)),
            vec2(480.0, 320.0)
        );
        assert_eq!(
            viewport.ndc_to_framebuffer(vec2(1.0, 1.0)),
            vec2(800.0, 560.0)
        );
    }
}
