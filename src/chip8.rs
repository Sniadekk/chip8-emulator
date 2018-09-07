use graphics::Display;
use keypad::Keypad;
use config::FONTSET;

use std::fs::File;
use std::env::current_dir;
use std::io::prelude::*;


pub struct Chip8 {
    op_code: u16,
    memory: [u8; 4096],
    register: [u8; 16],
    // stack pointer
    stack: [u16; 16],
    sp: usize,
    // index counter
    i: usize,
    // program counter
    pc: usize,
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

    pub fn load_game(&mut self, game: String) {
        let mut current_directory = current_dir().unwrap().display().to_string();
        current_directory.push_str(&game);
        let mut reader = File::open(&current_directory).unwrap();
        self.load_to_memory(&mut reader);
    }
    // TODO: finish saving game to the memory
    fn load_to_memory(&mut self, file: &mut File) {
        for byte in file.bytes(){
            match byte {
                Ok(value) => {
                    self.memory[self.pc] = value;
                    self.pc += 1;
                    self.load_to_memory(file);
                }
                Err(e) => {
                    self.pc = 0x200;
                }
            }
        }
    }

    fn fetch_op_code(&mut self) {
        self.op_code = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
    }

    fn decode_op_code(&self) {}
    fn execute_op_code(&self) {}

    pub fn emulate_cycle(&mut self) {
        self.fetch_op_code();
    }
}
