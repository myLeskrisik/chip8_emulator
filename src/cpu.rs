mod ops;

use std::fs;
use std::path::PathBuf;
use std::u8;
use pixels::Pixels;
use ops::exec_opcode;


// The memory address the program counter starts at
const START_ADDRESS: u16 = 0x200;

// The starting address in memory where the fonts are stored
const FONT_SET_START:usize = 0x50;

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
	0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

const GRAPHICS_ROWS: usize = 32;
const GRAPHICS_COLUMNS: usize = 64;

// Emulates the memory and the cpu
pub struct Cpu {
	// Represents the registers V0 .. VF
	registers: [u8; 16],
	//  Represents the memory of the system
	memory: [u8; 4096],
	// Register typically used to store adresses
	index_register: u16,
	// Represents the program counter
	pc: u16,
	// Represents the call stack
	stack: [u16; 16],
	// Represents the stack pointer
	sp: u8,
	// ??
	delay_timer: u8,
	// ??
	sound_timer: u8,
	// Represents the pixels on the screen
	graphics: [[bool; GRAPHICS_COLUMNS]; GRAPHICS_ROWS],
	pixels: Pixels,

}

impl Cpu {
	/* Returns a new Cpu struct */
	pub fn new (pixels: Pixels) -> Cpu {
		let mut cpu = Cpu {
			registers: [0; 16],
			memory: [0; 4096],
			index_register: 0,
			pc: START_ADDRESS,
			stack: [0; 16],
			sp: 0,
			delay_timer: u8::MAX,
			sound_timer: u8::MAX,
			graphics: [[false; GRAPHICS_COLUMNS]; GRAPHICS_ROWS],
			pixels: pixels
		};
		for (i, &data) in FONT_SET.iter().enumerate() {
			cpu.memory[FONT_SET_START + i] = data;
		}
		cpu
	}

	/* 	Loads the rom stored at puth buf into memory, starting at START_ADDRESS
	 *
	 *	 #Arguments
	 *	`self` - The Cpu object to load the rom into
	 *	`path` - The path which we should load the rom from
	 */
	pub fn load_rom (&mut self, path: &PathBuf) {
		let rom_data = fs::read(path).expect("Something went wrong with the file, try again");
		for (i, &data) in rom_data.iter().enumerate() {
			self.memory[START_ADDRESS as usize + i] = data;
		}
	}

	/* 	"Cycles" the Cpu. executing the next instruction and decrementing the proper timers
	 *
	 *	 #Arguments
	 *	`self` - The Cpu object which we should cycle
	 */
	pub fn cycle (&mut self) {
		let opcode = ((self.memory[self.pc as usize] << 4) + (self.memory[(self.pc + 1) as usize])) as u16;
		exec_opcode(self, opcode);

		if self.delay_timer > 0 {
			self.delay_timer -= 1;
		}

		if self.sound_timer > 0 {
			self.sound_timer -= 1;
		}
	}
}