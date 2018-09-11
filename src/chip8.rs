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
            // index register
            i: 0,
            // program counter
            pc: 0x200,
            display: Display::new(),
            keypad: Keypad::new(),
            stack: [0; 16],
            // stack pointer
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

    fn load_to_memory(&mut self, file: &mut File) {
        for byte in file.bytes() {
            match byte {
                Ok(value) => {
                    self.memory[self.pc] = value;
                    self.pc += 1;
                }
                Err(e) => {
                    eprintln!("e = {:#?}", e);
                    self.pc = 0x200;
                }
            }
        }
        self.pc = 0x200;
    }

    fn fetch_op_code(&mut self) {
        self.op_code = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
    }

    fn decode_op_code(&mut self) {
        match self.op_code & 0xF000 {
            0x0000 => {
                self.op_0xxx();
            }
            0x1000 => {
                self.op_1xxx();
            }
            0x2000 => {
                self.op_2xxx();
            }
            0x3000 => {
                self.op_3xxx();
            }
            0x4000 => {
                self.op_4xxx();
            }
            0x5000 => {
                self.op_5xxx();
            }
            0x6000 => {
                self.op_6xxx();
            }
            0x7000 => {
                self.op_7xxx();
            }
            0x8000 => {
                self.op_8xxx();
            }
            0xA000 => {
                self.op_axxx()
            }
            0xB000 => {
                self.op_bxxx()
            }
            0xC000 => {
                self.op_cxxx()
            }
            0xD000 => {
                self.op_dxxx()
            }
            0xE000 => {
                self.op_exxx()
            }
            0xF000 => {
                self.op_fxxx()
            }
        }
    }

    pub fn emulate_cycle(&mut self) {
        self.fetch_op_code();
        self.decode_op_code();
    }

    pub fn op_0xxx(&mut self) {
        self.display.clear();
    }

    pub fn op_1xxx(&mut self) {
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = self.get_addr();
    }

    pub fn op_2xxx(&mut self) {
        self.sp += 1;
        self.stack[self.sp] = self.pc;
        self.pc = self.get_addr();
    }

    pub fn op_3xxx(&mut self) {}
    pub fn op_4xxx(&mut self) {}
    pub fn op_5xxx(&mut self) {}
    pub fn op_6xxx(&mut self) {}
    pub fn op_7xxx(&mut self) {}
    pub fn op_8xxx(&mut self) {}
    pub fn op_9xxx(&mut self) {}
    pub fn op_axxx(&mut self) {}
    pub fn op_bxxx(&mut self) {}
    pub fn op_cxxx(&mut self) {}
    pub fn op_dxxx(&mut self) {}
    pub fn op_exxx(&mut self) {}
    pub fn op_fxxx(&mut self) {}

    fn get_x(&self) -> u8 { ((self.opcode & 0x0F00) >> 8) as u8 }
    fn get_y(&self) -> u8 { ((self.opcode & 0x00F0) >> 4) as u8 }
    fn get_nibble(&self) -> u8 { (self.opcode & 0x000F) as u8 }
    fn get_byte(&self) -> u8 { (self.opcode & 0x00FF) as u8 }

    // A 12-bit value, the lowest 12 bits of the instruction
    fn get_addr(&self) -> u16 { (self.opcode & 0x0FFF) }
}
