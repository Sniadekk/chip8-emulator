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

    let mut events = sdl_context.event_pump().unwrap();
    emulator.load_game("\\games\\pong.rom".to_string());

    'running: loop {
        for event in events.poll_iter() {
            if let Event::KeyDown { keycode, .. } = event {
                emulator.keypad.handle_input(keycode.unwrap())
            };

            emulator.emulate_cycle();
            let ten_millis = time::Duration::from_millis(100);
            thread::sleep(ten_millis);
        }
    }
}
