use std::fs;
use std::path::PathBuf;
use std::u8;
use rand::Rng;


const START_ADDRESS: u16 = 0x200;

const FONT_SET_START:usize = 0x50;

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

pub struct Cpu {
	registers: [u8; 16],
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

	pub fn load_rom(&mut self, path: &PathBuf) {
		let rom_data = fs::read(path).expect("Something went wrong with the file, try again");
		for (i, &data) in rom_data.iter().enumerate() {
			self.memory[START_ADDRESS as usize + i] = data;
		}
	}

	pub fn process_opcode(&mut self, op_code: u16) {
		match get_nibbles(op_code) {
			[0, 0, 0xe, 0] => cls(self),
			[0, 0, 0xe, 0xe] => ret(self),
			[1, n1, n2,n3] => jp_addr(self, (n1 << 2 + n2 << 1 + n3) as u16),
			[2, n1, n2, n3] => call_addr(self, (n1 << 2 + n2 << 1 + n3) as u16),
			_ => (),
		}
	}


}

type Nibbles = [u8; 4];

const NIBBLE_ONE_MASK: u16 = 0xf000;
const NIBBLE_TWO_MASK: u16 = 0x0f00;
const NIBBLE_THREE_MASK: u16 = 0x00f0;
const NIBBLE_FOUR_MASK: u16 = 0xf00f;


fn get_nibbles(op_code: u16) -> Nibbles {
	[
        ((op_code & NIBBLE_ONE_MASK) >> 12).try_into().unwrap(),
        ((op_code & NIBBLE_TWO_MASK) >> 8).try_into().unwrap(),
        ((op_code & NIBBLE_THREE_MASK) >> 4).try_into().unwrap(),
       ((op_code & NIBBLE_FOUR_MASK)).try_into().unwrap()
    ]
}

fn cls(cpu: &mut Cpu) {
	cpu.graphics.iter_mut().for_each(|x| x.iter_mut().for_each(|x| *x = 0));
}

fn ret(cpu: &mut Cpu) {
	cpu.sp -= 1;
}

fn jp_addr(cpu: &mut Cpu, addr: u16) {
	cpu.pc = addr;
}

fn call_addr(cpu: &mut Cpu, addr: u16) {
	cpu.stack[cpu.sp as usize] = cpu.pc; 
	cpu.sp += 1; 
	cpu.pc = addr; 
}