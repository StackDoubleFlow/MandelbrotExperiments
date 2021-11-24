mod normal;
mod fixed_point;

use fixed_point::draw;

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 32;
const HEIGHT: u32 = 32;
const MAX_ITERATION: usize = 20;
const PAN_SENSITIVITY: f64 = 2.0;
const ZOOM_SENSITIVITY: f64 = 0.02;

fn main() {
    let event_loop = EventLoop::new();

    let size = LogicalSize::new(WIDTH, HEIGHT);
    let window = WindowBuilder::new()
        .with_title("mandelbrot")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_max_inner_size(size)
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    let mut input = WinitInputHelper::new();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let (dimensions, offsets) = if WIDTH as f64 / HEIGHT as f64 >= 3.5 / 2.0 {
        let width = 2.0 * WIDTH as f64 / HEIGHT as f64;
        ((width, 2.0), ((-2.5 + (3.5 - width)) / 2.0, -1.0))
    } else {
        let height = 3.5 * HEIGHT as f64 / WIDTH as f64;
        ((3.5, height), (-2.5, (-1.0 + (1.0 - height)) / 2.0))
    };

    let scale = Box::leak(Box::new(dimensions.0 / WIDTH as f64));
    let offset_x = Box::leak(Box::new(offsets.0));
    let offset_y = Box::leak(Box::new(offsets.1));
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            draw(pixels.get_frame(), *offset_x, *offset_y, *scale);
            if pixels
                .render()
                .map_err(|e| panic!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_held(VirtualKeyCode::W) {
                *offset_y += -PAN_SENSITIVITY * *scale;
            }
            if input.key_held(VirtualKeyCode::A) {
                *offset_x += -PAN_SENSITIVITY * *scale;
            }
            if input.key_held(VirtualKeyCode::S) {
                *offset_y += PAN_SENSITIVITY * *scale;
            }
            if input.key_held(VirtualKeyCode::D) {
                *offset_x += PAN_SENSITIVITY * *scale;
            }

            let old_x = (WIDTH as f64 / 2.0) * *scale + *offset_x;
            let old_y = (HEIGHT as f64 / 2.0) * *scale + *offset_y;
            if input.key_held(VirtualKeyCode::Up) {
                *scale *= 1.0 - ZOOM_SENSITIVITY;
            }
            if input.key_held(VirtualKeyCode::Down) {
                *scale *= ZOOM_SENSITIVITY + 1.0;
            }
            let new_x = (WIDTH as f64 / 2.0) * *scale + *offset_x;
            let new_y = (HEIGHT as f64 / 2.0) * *scale + *offset_y;
            *offset_x += old_x - new_x;
            *offset_y += old_y - new_y;

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}


