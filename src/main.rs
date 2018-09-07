pub mod chip8;
pub mod graphics;
pub mod keypad;
pub mod config;

use chip8::Chip8;
pub use graphics::Display;


fn main() {
    let mut emulator = Chip8::new();
    loop {
        emulator.emulate_cycle();
        if emulator.display.should_draw {
            emulator.display.draw();
            emulator.display.should_draw = false;
        }
    }
}
