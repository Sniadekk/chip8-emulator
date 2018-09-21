use sdl2::keyboard::Keycode;

pub struct Keypad {}

impl Keypad {
    pub fn new() -> Self {
        Keypad {}
    }

    pub fn handle_input(&mut self, keycode: Keycode) {
        eprintln!("keycode = {:#?}", keycode);
    }
}