mod ops;

use crate::cpu::ops::exec_opcode;
use pixels::Pixels;
use std::fs;
use std::path::PathBuf;
use std::u8;

// The memory address the program counter starts at
const START_ADDRESS: u16 = 0x200;

// The starting address in memory where the fonts are stored
const FONT_SET_START: usize = 0x50;

// The bytes for each given alphanumeric
const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const GRAPHICS_ROWS: usize = 32;
const GRAPHICS_COLUMNS: usize = 64;

const NUM_REGISTERS: usize = 16;
// In bytes
const KILOBYTE: usize = 1024;
const MEM_SIZE: usize = 4 * KILOBYTE;
const CALL_STACK_SIZE: usize = 16;

// Emulates the memory and the cpu
pub struct Cpu {
    // Represents the registers V0 .. VF
    registers: [u8; NUM_REGISTERS],
    //  Represents the memory of the system, in bytes
    memory: [u8; MEM_SIZE],
    // Register typically used to store adresses
    index_register: u16,
    // Represents the program counter
    pc: u16,
    // Represents the call stack
    stack: [u16; CALL_STACK_SIZE],
    // Represents the stack pointer
    sp: u8,
    // ??
    delay_timer: u8,
    // ??
    sound_timer: u8,
    // Represents the pixels on the screen
    graphics: [[bool; GRAPHICS_COLUMNS]; GRAPHICS_ROWS],
}

impl Cpu {
    /* Returns a new Cpu struct */
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            registers: [0; NUM_REGISTERS],
            memory: [0; MEM_SIZE],
            index_register: 0,
            pc: START_ADDRESS,
            stack: [0; CALL_STACK_SIZE],
            sp: 0,
            delay_timer: u8::MAX,
            sound_timer: u8::MAX,
            graphics: [[false; GRAPHICS_COLUMNS]; GRAPHICS_ROWS],
        };
        for (i, &data) in FONT_SET.iter().enumerate() {
            cpu.memory[FONT_SET_START + i] = data;
        }
        cpu
    }

    /// Loads the rom stored at puth buf into memory, starting at START_ADDRESS
    ///
    ///	`self` - The Cpu object to load the rom into
    ///	`path` - The path which we should load the rom from
    pub fn load_rom(&mut self, path: &PathBuf) {
        let rom_data = fs::read(path).expect("Something went wrong with the file, try again");
        for (i, &data) in rom_data.iter().enumerate() {
            self.memory[START_ADDRESS as usize + i] = data;
        }
    }

    /// "Cycles" the Cpu. executing the next instruction and decrementing the proper timers
    ///
    /// `self` - The Cpu object which we should cycle
    pub fn cycle(&mut self) {
        let opcode =
            ((self.memory[self.pc as usize] << 4) + (self.memory[(self.pc + 1) as usize])) as u16;
        exec_opcode(self, opcode);

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn draw(&self, screen: &mut [u8]) {
        for on in self.graphics.iter().zip() {
            let color = if c.alive {
                [0, 0xff, 0xff, 0xff]
            } else {
                [0, 0, c.heat, 0xff]
            };
            pix.copy_from_slice(&color);
        }
    }
}
