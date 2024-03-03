use minifb::{Key, KeyRepeat, Window, WindowOptions};

const WINDOW_TITLE: &str = "Rasterization in One Weekend";
const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    let mut window = Window::new(
        WINDOW_TITLE,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_micros(1000 / 60)));

    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        for y in 0..WINDOW_HEIGHT {
            for x in 0..WINDOW_WIDTH {
                let i = y * WINDOW_WIDTH + x;
                let on_left_half = x / (WINDOW_WIDTH / 2) == 0;
                let on_upper_half = y / (WINDOW_HEIGHT / 2) == 0;
                buffer[i] = if on_left_half ^ on_upper_half {
                    0x00FFFFFF
                } else {
                    0x00000000
                };
            }
        }

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}
