#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fltk::{app, prelude::*, window::Window};
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use std::{thread, time::Duration};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;
const CIRCLE_RADIUS: i16 = 64;
const SLEEP: u64 = 16;

/// Representation of the application state. In this example, a circle will bounce around the screen.
struct World {
    circle_x: i16,
    circle_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let app = app::App::default();
    let mut win = Window::default()
        .with_size(WIDTH as i32, HEIGHT as i32)
        .with_label("Hello Pixels");
    win.end();
    win.show();

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &win);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut world = World::new();

    while app.wait() {
        // Update internal state
        world.update();
        // Draw the current frame
        world.draw(pixels.get_frame());
        if pixels
            .render()
            .map_err(|e| error!("pixels.render() failed: {}", e))
            .is_err()
        {
            app.quit();
        }
        win.redraw();
        // Calls to redraw in the event loop require an explicit sleep
        thread::sleep(Duration::from_millis(SLEEP));
    }

    Ok(())
}

impl World {
    /// Create a new `World` instance that can draw a moving circle.
    fn new() -> Self {
        Self {
            circle_x: 300,
            circle_y: 200,
            velocity_x: 5,
            velocity_y: 5,
        }
    }

    /// Update the `World` internal state; bounce the circle around the screen.
    fn update(&mut self) {
        if self.circle_x - CIRCLE_RADIUS <= 0 || self.circle_x + CIRCLE_RADIUS > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.circle_y - CIRCLE_RADIUS <= 0 || self.circle_y + CIRCLE_RADIUS > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.circle_x += self.velocity_x;
        self.circle_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;
            let length = {
                let x = (x - self.circle_x) as f64;
                let y = (y - self.circle_y) as f64;

                x.powf(2.0) + y.powf(2.0)
            };
            let inside_the_circle = length < (CIRCLE_RADIUS as f64).powi(2);

            let rgba = if inside_the_circle {
                [0xac, 0x00, 0xe6, 0xff]
            } else {
                [0x26, 0x00, 0x33, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
