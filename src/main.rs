pub mod chip8;
pub mod graphics;
pub mod keypad;
pub mod config;

use sdl2::event::Event;

extern crate rand;
extern crate sdl2;

use chip8::Chip8;
use std::{thread, time};
pub use graphics::Display;


fn main() {
    let mut emulator = Chip8::new();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 demo: Window", 800, 600)
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut events = sdl_context.event_pump().unwrap();
    emulator.load_game("\\games\\pong.rom".to_string());
    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode: Some(keycode), .. } => emulator.keypad.handle_input(keycode),
                _ => ()
            }
            emulator.emulate_cycle();
            let ten_millis = time::Duration::from_millis(16);
            thread::sleep(ten_millis);
        }
        canvas.clear();
        canvas.present();
    }
}
