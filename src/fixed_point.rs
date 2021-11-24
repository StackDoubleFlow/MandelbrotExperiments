use crate::{WIDTH, HEIGHT, MAX_ITERATION};

type Type = i32;
const FIXED_POINT: Type = 25;

fn to_fixed(num: f64) -> Type {
    (num * (2usize).pow(FIXED_POINT as u32) as f64) as Type
}

fn to_float(num: Type) -> f64 {
    num as f64 / (2usize).pow(FIXED_POINT as u32) as f64
}

fn mul(a: Type, b: Type) -> Type {
    // println!("{:032b} * {:032b} = {:064b}", a, b, a as isize * b as isize);
    ((a as isize * b as isize) >> FIXED_POINT) as Type
}

pub fn draw(frame: &mut [u8], offset_x: f64, offset_y: f64, scale: f64) {
    // dbg!(to_float(mul(to_fixed(-1.0), to_fixed(-0.2))));
    // println!("{:032b}", mul(to_fixed(1.0), to_fixed(1.0)));
    // return;

    // dbg!(offset_x, offset_y, scale);
    let offset_x = to_fixed(offset_x);
    let offset_y = to_fixed(offset_y);
    let scale = to_fixed(scale);
    // println!("{:032b}", offset_y);
    for px in 0..WIDTH {
        for py in 0..HEIGHT {
            let x0 = mul((px as Type) << FIXED_POINT, scale) + offset_x;
            let y0 = mul((py as Type) << FIXED_POINT, scale) + offset_y;

            let mut x = to_fixed(0.0);
            let mut y = to_fixed(0.0);
            let mut x2 = to_fixed(0.0);
            let mut y2 = to_fixed(0.0);

            let mut iteration = 0;

            while x2 + y2 <= to_fixed(4.0) && iteration < MAX_ITERATION {
                y = (mul(x, y) << 1) + y0;
                x = x2 - y2 + x0;
                x2 = mul(x, x);
                y2 = mul(y, y);
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
