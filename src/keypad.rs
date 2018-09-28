use sdl2::keyboard::Keycode;

pub struct Keypad {}

impl Keypad {
    pub fn new() -> Self {
        Keypad {}
    }

    pub fn handle_input(&mut self, keycode: Keycode)
    {
        match keycode {
            Keycode::Num0 => {},
            Keycode::Num1 => {},
            Keycode::Num2 => {},
            Keycode::Num3 => {},
            Keycode::Num4 => {},
            Keycode::Num5 => {},
            Keycode::Num6 => {},
            Keycode::Num7 => {},
            Keycode::Num8 => {},
            Keycode::Num9 => {},
            Keycode::A => {},
            Keycode::B => {},
            Keycode::C => {},
            Keycode::D => {},
            Keycode::E => {},
            Keycode::F => {},
        }
    }
}