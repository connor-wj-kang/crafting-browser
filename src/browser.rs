extern crate sdl2;

use sdl2::{event::Event, video::Window};
use sdl2::{keyboard::Keycode, EventPump};
use skia_safe::{surfaces::raster, AlphaType, ColorType, ImageInfo, Surface};

pub static WIDTH: u32 = 1280;
pub static HEIGHT: u32 = 720;

struct Browser {
    sdl_window: Window,
    event_pump: EventPump,
    root_surface: Surface,
}

impl Browser {
    fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let sdl_window = video_subsystem
            .window("Browser", WIDTH, HEIGHT)
            .position_centered()
            .allow_highdpi()
            .resizable()
            .build()
            .unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let root_surface = raster(
            &ImageInfo::new(
                (WIDTH as i32, HEIGHT as i32),
                ColorType::RGBA8888,
                AlphaType::Unpremul,
                None,
            ),
            None,
            None,
        )
        .unwrap();

        Self {
            sdl_window,
            event_pump,
            root_surface,
        }
    }

    fn run(mut self) {
        loop {
            self.poll_event();
        }
    }

    fn poll_event(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break,
                _ => {}
            }
        }
    }
}
