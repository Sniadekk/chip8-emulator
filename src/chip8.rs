use graphics::Display;
use keypad::Keypad;
use config::FONTSET;

pub struct Chip8 {
    op_code: u16,
    memory: [u8; 4096],
    register: [u8; 16],
    // stack pointer
    stack: [u16; 16],
    sp: u16,
    // index counter
    i: u16,
    // program counter
    pc: u16,
    pub display: Display,
    pub keypad: Keypad,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip: Chip8 = Chip8 {
            op_code: 0,
            memory: [0; 4096],
            register: [0; 16],
            i: 0,
            pc: 0x200,
            display: Display::new(),
            keypad: Keypad::new(),
            stack: [0; 16],
            sp: 0,
        };

        for x in 0..80 {
            chip.memory[x] = FONTSET[x];
        }
        chip
    }
    fn fetch_op_code(&mut self) {}
    fn decode_op_code(&self) {}
    fn execute_op_code(&self) {}

    pub fn emulate_cycle(&self) -> () {}
}
