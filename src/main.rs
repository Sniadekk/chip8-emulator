pub mod chip8;
pub mod graphics;
pub mod keypad;
pub mod config;

extern crate rand;

use chip8::Chip8;
use std::{thread, time};
pub use graphics::Display;


fn main() {
    let mut emulator = Chip8::new();
    emulator.load_game("\\games\\pong.rom".to_string());
    loop {
        emulator.emulate_cycle();
//        if emulator.display.should_draw {
//            emulator.display.draw();
//            emulator.display.should_draw = false;
//        }
        let ten_millis = time::Duration::from_millis(100);
        thread::sleep(ten_millis);
    }
}
