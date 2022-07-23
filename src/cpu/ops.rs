use rand::Rng;
use crate::cpu::GRAPHICS_COLUMNS;
use crate::cpu::GRAPHICS_ROWS;
use crate::Cpu;

/* 	Executes the op code passed 
 *
 *	#Arguments
 *	`cpu` - The Cpu to execute the operation on
 *	`op_code` - The opcode to execute
 */
pub fn exec_opcode (cpu: &mut Cpu, op_code: u16) {
	match get_nibbles(op_code) {
		[0, 0, 0xe, 0] => cls(cpu),
		[0, 0, 0xe, 0xe] => ret(cpu),
		[1, n1, n2,n3] => jp_addr(cpu,  nibbles_to_u16(n1, n2, n3)),
		[2, n1, n2, n3] => call_addr(cpu, nibbles_to_u16(n1, n2, n3)),
		[3, x, n1, n2] => se_vx_byte(cpu, x as usize, nibbles_to_u8(n1,n2)),
		[4,x,n1,n2] => sne_vx_byte(cpu, x as usize, nibbles_to_u8(n1, n2)),
		[5, x, y, 0] => se_vx_vy(cpu, x as usize, y as usize),
		[6, x, n1, n2] => ld_vx_byte(cpu, x as usize, nibbles_to_u8(n1, n2)),
		[7, x, n1, n2] => add_vx_byte(cpu, x as usize, nibbles_to_u8(n1, n2)),
		[8, x, y, 0] => ld_vx_vy(cpu, x as usize, y as usize),
		[8, x, y, 1] => or_vx_vy(cpu, x as usize, y as usize),
		[8, x, y, 2] => and_vx_vy(cpu, x as usize, y as usize),
		[8, x, y, 3] => xor_vx_vy(cpu, x as usize, y as usize),
		[8, x, y, 4] => add_vx_vy(cpu, x as usize, y as usize),
		[8, x, y, 5] => sub_vx_vy(cpu, x as usize, y as usize),
		[8, x, _, 6] => shr_vx_vy(cpu, x as usize),
		[8, x, y, 7] => subn_vx_vy(cpu, x as usize, y as usize),
		[8, x, _, _e] => shl_vx_vy(cpu, x as usize),
		[9, x, y, 0] => sne_vx_vy(cpu, x as usize, y as usize),
		[0xa, n1, n2, n3] => ld_i_addr(cpu, nibbles_to_u16(n1, n2, n3)),
		[0xb, n1, n2, n3] => jp_v0_addr(cpu, nibbles_to_u16(n1, n2, n3)),
		[0xc, x, n1, n2] => rnd_vx_byte(cpu, x as usize, nibbles_to_u8(n1, n2)),
		[0xd, x, y, n] => drw_vx_vy_nibble(cpu, x as usize, y as usize, n as usize),
		_ => (),
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

/* 	Returns the Nibbles (half a byte each) from a 2 byte number 
 *
 *	 #Arguments
 *	`op_code` - The two byte value representing an instruction
 */
fn get_nibbles (op_code: u16) -> Nibbles {
	[
    ((op_code & NIBBLE_ONE_MASK) >> 12).try_into().unwrap(),
    ((op_code & NIBBLE_TWO_MASK) >> 8).try_into().unwrap(),
    ((op_code & NIBBLE_THREE_MASK) >> 4).try_into().unwrap(),
    ((op_code & NIBBLE_FOUR_MASK)).try_into().unwrap()
  ]
}

fn nibbles_to_u8 (n1: u8, n2: u8) -> u8 {
	assert!(n1.leading_zeros() >= 4);
	assert!(n2.leading_zeros() >= 4);
	n1 << 4 | n2
}

fn nibbles_to_u16 (n1: u8, n2: u8, n3: u8) -> u16 {
	assert!(n1.leading_zeros() >= 4);
	assert!(n2.leading_zeros() >= 4);
	assert!(n3.leading_zeros() >= 4);
	(n1 as u16) << 8 | (n2 as u16) << 4 | (n3 as u16)
}

// Represents the register which is used to store flags about operations
const FLAG_REGISTER: usize = 0xf;

/* 	Clears the screen
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 */
fn cls (cpu: &mut Cpu) {
	cpu.graphics.iter_mut().for_each(|x| x.iter_mut().for_each(|x| *x = false));
}

/* 	Returns from a subroutine
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 */
fn ret (cpu: &mut Cpu) {
	cpu.sp -= 1;
}

/* 	Jumps to the address, addr
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`addr` - The address we should jump to
 */
fn jp_addr (cpu: &mut Cpu, addr: u16) {
	cpu.pc = addr;
}

/* 	Calls the subroutine at address, addr
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`addr` - The address where we should call the subroutine
 */
fn call_addr (cpu: &mut Cpu, addr: u16) {
	cpu.stack[cpu.sp as usize] = cpu.pc; 
	cpu.sp += 1; 
	cpu.pc = addr; 
}

/* 	Skips the next instruction, if the data in register x is equal to kk
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register we check equality for
 *	`kk` - The value we check equality for
 */
fn se_vx_byte (cpu: &mut Cpu, x: usize, kk: u8) {
	if cpu.registers[x] == kk { cpu.pc += 2; }
}

/* 	Skips the next instruction, if the data in register x is not equal to kk
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register to check for inequality
 *	`kk` - The value to check for inequality
 */
fn sne_vx_byte (cpu: &mut Cpu, x: usize, kk: u8) {
	if cpu.registers[x] != kk { cpu.pc += 2; }
}

/* 	Skips the next instruction, if the data in register x is equal to the data in register y
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register to check for inequality
 *	`y` - The other register to check for inequality
 */
fn se_vx_vy (cpu: &mut Cpu, x: usize, y: usize) {
	if cpu.registers[x] == cpu.registers[y] { cpu.pc += 2; }
}

/* 	Sets the data in register x to kk
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register which will be set
 *	`kk` - The data which should be set
 */
fn ld_vx_byte (cpu: &mut Cpu, x: usize, kk:u8) {
	cpu.registers[x] = kk;
}

/* 	Adds kk to the value that is currently in register x, then sets register x to the result
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 */
fn add_vx_byte (cpu: &mut Cpu, x: usize, kk: u8) {
	cpu.registers[x] += kk;
}

/* 	Sets the value of register y to the value of register x
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register whose data will be set to other register
 *	`y` - The register which will have its data set
 *
 */
fn ld_vx_vy (cpu: &mut Cpu, x: usize, y: usize) {
	cpu.registers[y] = cpu.registers[x];
}

/* 	Calculates the OR of the value of register x and the value of register y, 
 *	then stores the result in register x
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register whose data will be set, OR'ed with the other register
 *	`y` - The other register which is OR'ed
 */
fn or_vx_vy (cpu: &mut Cpu, x: usize, y: usize) {
	cpu.registers[x] |= cpu.registers[y];
}

/* 	Calculates the AND of the value of register x and the value of register y, 
 * 	then stores the result in register x
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register which will be set, AND'ed with the other register
 *	`y` - The other register which will be AND'ed
 */
fn and_vx_vy (cpu: &mut Cpu, x: usize, y: usize) {
	cpu.registers[x] &= cpu.registers[y];
}

/* 	Calculates the Exclursive OR (XOR) of the value of register x and the value of register y, 
 * 	then stores the result in register x
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register which will be set, XOR'ed with the other register
 *	`y` - The other register which will be XOR'ed
 */
fn xor_vx_vy (cpu: &mut Cpu, x: usize, y: usize) {
	cpu.registers[x] ^= cpu.registers[y];
}

/* 	Calculates the sum of registers x and y then puts that value into register x. If that value
 * 	is greater than 255, then VF is set to 1, 0 otherwise
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register which will be set, added to the other register
 *	`y` - The other register which will be added
 */
fn add_vx_vy (cpu: &mut Cpu, x: usize, y: usize) {
	match cpu.registers[x].checked_add(cpu.registers[y]) {
		Some (z) => { cpu.registers[x] = z; cpu.registers[FLAG_REGISTER] = 0; },
		None => { cpu.registers[x] = u8::MAX; cpu.registers[FLAG_REGISTER] = 1; },
	};
}

/* 	Calculates the difference between registers x and y then puts that value into regsiter x. If that value
 * 	would result in overflow then VF is set to 0, 1 otherwise
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register which will be set, the first operand of the subtraction
 *	`y` - The register which is the second operand of the subtraction
 */
fn sub_vx_vy (cpu: &mut Cpu, x: usize, y: usize) {
	match cpu.registers[x].checked_sub(cpu.registers[y]) {
		Some (z) => { cpu.registers[x] = z; cpu.registers[FLAG_REGISTER] = 1; },
		None =>{ cpu.registers[x] = 0; cpu.registers[FLAG_REGISTER] = 0;}
	}
}

/* Shifts the bits in register vx right once, if the LSB is 1, VF is set to 1, 0 otherwise
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register which will be bit-shifted right
 */
fn shr_vx_vy (cpu: &mut Cpu, x: usize) {
	if cpu.registers[x] % 2 == 0 {
		cpu.registers[FLAG_REGISTER] = 0;
	} else {
		cpu.registers[FLAG_REGISTER] = 0;
	}
	cpu.registers[x] >>= 1;
}

/* 	Calculates the difference between registers y and x then puts that value into regsiter x. If that value
 * 	would result in overflow, then VF is set to 0, 1 otherwise
 *
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register which will be set, the second operand of the subtraction
 *	`y` - The register which is the first operand of the subtraction
 */
fn subn_vx_vy (cpu: &mut Cpu, x: usize, y: usize) {
	match cpu.registers[y].checked_sub(cpu.registers[x]) {
		Some (z) => { cpu.registers[x] = z; cpu.registers[FLAG_REGISTER] = 1; },
		None =>{ cpu.registers[x] = 0; cpu.registers[FLAG_REGISTER] = 0;}
	}
}

/* 	Shifts the bits in register vx left once, if the MSB is 1, VF is set to 1, 0 otherwise
 *
 *	#Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register whose data which will be shifted left
 */
fn shl_vx_vy (cpu: &mut Cpu, x: usize) {
	if cpu.registers[x] >> 7 == 1 {
		cpu.registers[FLAG_REGISTER] = 1;
	} else {
		cpu.registers[FLAG_REGISTER] = 0;
	}
	cpu.registers[x] <<= 1;
}

/* 	Skips the next instruction if the value of register x is not equal to register y
 *
 *	#Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The first register which will be checked for inequality
 *	`y` - The second register which will be checked for inequality
 */
fn sne_vx_vy (cpu: &mut Cpu, x: usize, y: usize) {
	if cpu.registers[x] != cpu.registers[y] {
		cpu.pc += 2;
	}
}

/* 	Sets the value of the index register to the address passed
 * 
 *	 #Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`addr` - The address which should be placed in the index register
 */
 fn ld_i_addr (cpu: &mut Cpu, addr: u16) {
 	cpu.index_register = addr;
 }

 /*	Jumps to the location: addr + V0
  *
  *	 #Arguments
  *	`cpu` - The Cpu which we should execute this instruction on
  * `addr` - The address which, summed with register 0's value, will be jumped to
  */
  fn jp_v0_addr (cpu: &mut Cpu, addr: u16) {
  	cpu.pc = (cpu.registers[0] as u16) + addr;
  }

/* Generates a random byte then AND's it with the byte passed, then sets register x to it
 * 
 *	#Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register which will be set
 * 	`byte` - The byte which will be AND'ed with the generated random number
 */
  fn rnd_vx_byte (cpu: &mut Cpu, x: usize, byte: u8) {

  	cpu.registers[x] = rand::thread_rng().gen::<u8>() & byte;
  }

/*	Draws the nibble size sprite at (register x, register y) starting at the value in
 * index_register
 *
 *	#Arguments
 *	`cpu` - The Cpu which we should execute this instruction on
 *	`x` - The register which we should get the sprites x coordinate from
 *	`y` - The register which we should get the sprites y coordinate from
 *	`nibble` - The number of bytes to be read from memory for the sprite
 */
  fn drw_vx_vy_nibble (cpu: &mut Cpu, x: usize, y: usize, nibble: usize) {
  	let starting_addr = cpu.index_register as usize;
  	let x = cpu.registers[x];
  	let mut cur_y = cpu.registers[y] as usize;
  	let mut pixels_changed = false;

  	for i in 0 .. nibble {
  		let cur_byte = cpu.memory[starting_addr + i];
  		let mut cur_x = x as usize; 

  		for j in 0 .. u8::BITS as u8 {
  			let pixel_on = get_ith_bit(j, cur_byte).unwrap() == 1;
  			let cur_pixel_val = cpu.graphics[cur_y % GRAPHICS_ROWS][cur_x % GRAPHICS_COLUMNS];

  			if cur_pixel_val && !pixel_on {
  				cpu.registers[FLAG_REGISTER] = 1;
  				pixels_changed = true
  			} else if !pixels_changed {
  				cpu.registers[FLAG_REGISTER] = 0;
  			}

  			cpu.graphics[cur_y % GRAPHICS_ROWS][cur_x % GRAPHICS_COLUMNS] = cur_pixel_val ^ pixel_on;
  			cur_x += 1;

  		}
  		cur_y += 1;
  	}
  }

/* 	Gets the bit_num'th bit from the byte passed and returns Some(bit), if bit_num is in the range 0..7,
 * 	None is returned otherwise
 *
 *	#Arguments
 *	`bit_num` - The bit we want extracted from the byte, must be 0..7
 *	`byte` - The byte we want to extract the bit_num'th bit from
 */
  fn get_ith_bit (bit_num: u8, byte: u8) -> Option<u8> {
  	if bit_num > 7 { return None; }
  	let single_bit_mask = 0b1000_0000 >> bit_num;
  	let res = byte & single_bit_mask;
  	assert!(res == 1 || res == 0, "The result was neither 1 or 0");
  	Some(res)
  }
