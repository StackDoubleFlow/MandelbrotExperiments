
use crate::{WIDTH, HEIGHT, MAX_ITERATION};

pub fn draw(frame: &mut [u8], offset_x: f64, offset_y: f64, scale: f64) {
    for px in 0..WIDTH {
        for py in 0..HEIGHT {
            let x0 = px as f64 * scale + offset_x;
            let y0 = py as f64 * scale + offset_y;

            let mut x = 0.0;
            let mut y = 0.0;
            let mut x2 = 0.0;
            let mut y2 = 0.0;

            let mut iteration = 0;

            while x2 + y2 <= 4.0 && iteration < MAX_ITERATION {
                y = 2.0 * x * y + y0;
                x = x2 - y2 + x0;
                x2 = x * x;
                y2 = y * y;
                iteration += 1;
            }

            let quotient = iteration as f64 / MAX_ITERATION as f64;

            let pixel = if quotient > 0.5 {
                [255, 255, 255, 255]
            } else {
                [0, 0, 0, 255]
            };

            let i = ((py * WIDTH) + px) * 4;
            frame[i as usize..i as usize + 4].copy_from_slice(&pixel);
        }
    }
}
