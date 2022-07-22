use std::fs;
use std::path::PathBuf;
use std::u8;
use rand::Rng;

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
	0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

// Emulates the memory and the cpu
pub struct Cpu {
	// Represents the registers v0 .. v15
	registers: [u8; 16],
	// 
	memory: [u8; 4096],
	index_register: u16,
	pc: u16,
	stack: [u16; 16],
	sp: u8,
	delay_timer: u8,
	sound_timer: u8,
	graphics: [[u8; 64]; 32],
}

impl Cpu {
	// Returns a new Cpu struct
	pub fn new() -> Cpu {
		let mut cpu = Cpu {
			registers: [0; 16],
			memory: [0; 4096],
			index_register: 0,
			pc: START_ADDRESS,
			stack: [0; 16],
			sp: 0,
			delay_timer: u8::MAX,
			sound_timer: u8::MAX,
			graphics: [[0; 64]; 32]
		};
		for (i, &data) in FONT_SET.iter().enumerate() {
			cpu.memory[FONT_SET_START + i] = data;
		}
		cpu
	}

	// Loads the rom stored at puth buf into memory, starting at START_ADDRESS
	pub fn load_rom(&mut self, path: &PathBuf) {
		let rom_data = fs::read(path).expect("Something went wrong with the file, try again");
		for (i, &data) in rom_data.iter().enumerate() {
			self.memory[START_ADDRESS as usize + i] = data;
		}
	}

	// Executes the op code passed in the parameter op_code
	pub fn exec_opcode(&mut self, op_code: u16) {
		match get_nibbles(op_code) {
			[0, 0, 0xe, 0] => cls(self),
			[0, 0, 0xe, 0xe] => ret(self),
			[1, n1, n2,n3] => jp_addr(self, (n1 << 2 | n2 << 1 | n3) as u16),
			[2, n1, n2, n3] => call_addr(self, (n3 | n1 << 2 | n2 << 1) as u16),
			[3, x, k1, k2] => se_vx(self, x as usize, k2 | k1 << 1),
			_ => (),
		}
	}

	pub fn cycle(&mut self) {
		let opcode = ((self.memory[self.pc as usize] << 4) + (self.memory[(self.pc + 1) as usize])) as u16;
		self.exec_opcode(opcode);

		if self.delay_timer > 0 {
			self.delay_timer -= 1;
		}

		if self.sound_timer > 0 {
			self.sound_timer -= 1;
		}
	}
}

// Represents each half byte (a nibble) in a 2-byte number
type Nibbles = [u8; 4];

// Masks out the first nibble from a 2-byte numbers
const NIBBLE_ONE_MASK: u16 = 0xf000;
// Masks out the second nibble from a 2-byte numbers
const NIBBLE_TWO_MASK: u16 = 0x0f00;
// Masks out the third nibble from a 2-byte numbers
const NIBBLE_THREE_MASK: u16 = 0x00f0;
// Masks out the fourth nibble from a 2-byte numbers
const NIBBLE_FOUR_MASK: u16 = 0xf00f;

// Returns the Nibbles (half a byte each) from a 2 byte number
fn get_nibbles(op_code: u16) -> Nibbles {
	[
        ((op_code & NIBBLE_ONE_MASK) >> 12).try_into().unwrap(),
        ((op_code & NIBBLE_TWO_MASK) >> 8).try_into().unwrap(),
        ((op_code & NIBBLE_THREE_MASK) >> 4).try_into().unwrap(),
       ((op_code & NIBBLE_FOUR_MASK)).try_into().unwrap()
    ]
}

// Clears the screen
fn cls(cpu: &mut Cpu) {
	cpu.graphics.iter_mut().for_each(|x| x.iter_mut().for_each(|x| *x = 0));
}

// Returns from a subroutine
fn ret(cpu: &mut Cpu) {
	cpu.sp -= 1;
}

// Jumps to the address, addr
fn jp_addr(cpu: &mut Cpu, addr: u16) {
	cpu.pc = addr;
}

// Calls the subroutine at address, addr
fn call_addr(cpu: &mut Cpu, addr: u16) {
	cpu.stack[cpu.sp as usize] = cpu.pc; 
	cpu.sp += 1; 
	cpu.pc = addr; 
}

// Skips the next instruction, if the data in the register x is equal to kk
fn se_vx(cpu: &mut Cpu, x: usize, kk: u8) {
	if cpu.registers[x] == kk {
		cpu.pc += 2;
	}
}