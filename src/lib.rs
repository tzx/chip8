use rand::{prelude::ThreadRng, thread_rng, Rng, RngCore};

const MEM_SIZE: usize = 4096;
pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_WIDTH: usize = 64;
const NUM_V_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const PC_START: u16 = 0x200;

// Sprites are 5 bytes each, so we have 16x5=80 bytes
const HEX_SPRITE_SIZE: usize = 80;
const HEXIDECIMAL_SPRITES: [u8; HEX_SPRITE_SIZE] = [
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

// can't derive default for const generics:
// https://github.com/rust-lang/rust/pull/60466#discussion_r280989938
pub struct Chip8<R: RngCore> {
    memory: [u8; MEM_SIZE],
    v_regs: [u8; NUM_V_REGS], // general purpose registers
    i: u16,                   // I register (used to store memory addresses)
    dt: u8,                   // delay timer
    st: u8,                   // sound timer
    pc: u16,                  // program counter
    sp: u8,                   // stack pointer
    stack: [u16; STACK_SIZE],
    keyboard: u16,
    display: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
    rng: R,
}

impl Default for Chip8<ThreadRng> {
    fn default() -> Self {
        Chip8 {
            memory: [0; MEM_SIZE],
            v_regs: [0; NUM_V_REGS],
            i: 0,
            dt: 0,
            st: 0,
            pc: PC_START,
            sp: 0,
            stack: [0; STACK_SIZE],
            keyboard: 0,
            display: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
            rng: thread_rng(),
        }
    }
}

impl Chip8<ThreadRng> {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            ..Default::default()
        };
        chip8.memory[0..HEX_SPRITE_SIZE].copy_from_slice(&HEXIDECIMAL_SPRITES);

        chip8
    }
}

impl<R: RngCore> Chip8<R> {
    pub fn load_rom_data(&mut self, rom_data: &[u8]) {
        let start = PC_START as usize;
        let end = start + rom_data.len();
        self.memory[start..end].copy_from_slice(rom_data);
    }

    pub fn get_display(&self) -> &[[u8; SCREEN_WIDTH]] {
        &self.display
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch_opcode();
        self.process_opcode(opcode);
    }

    pub fn timer_tick(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // I don't want to deal with sound, but it beeps here
            }
            self.st -= 1;
        }
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        if pressed {
            self.keyboard |= 1 << idx;
        } else {
            self.keyboard &= !(1 << idx);
        }
    }

    fn fetch_opcode(&mut self) -> u16 {
        let higher_byte = self.memory[self.pc as usize] as u16;
        let lower_byte = self.memory[(self.pc + 1) as usize] as u16;
        let opcode = (higher_byte << 8) | lower_byte;
        // Increase the program counter by 2 since each instruction is 2 bytes
        self.pc += 2;
        opcode
    }

    fn process_opcode(&mut self, opcode: u16) {
        let nnn = opcode & 0x0FFF;
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        // x and y are used to index and they are converted from u8 so it should be safe
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3;
        let kk = (opcode & 0x0FF) as u8;

        match nibbles {
            // CLS
            (0, 0, 0xE, 0) => self.display = [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
            // RET
            (0, 0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            // JP addr
            (1, _, _, _) => self.pc = nnn,
            // CALL addr
            (2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn
            }
            // SE Vx, byte
            (3, _, _, _) => {
                self.pc += if self.v_regs[x] == kk { 2 } else { 0 };
            }
            (4, _, _, _) => {
                self.pc += if self.v_regs[x] != kk { 2 } else { 0 };
            }
            (5, _, _, 0) => {
                self.pc += if self.v_regs[x] == self.v_regs[y as usize] {
                    2
                } else {
                    0
                };
            }
            (6, _, _, _) => {
                self.v_regs[x] = kk;
            }
            (7, _, _, _) => {
                self.v_regs[x] = self.v_regs[x].wrapping_add(kk);
            }
            (8, _, _, 0) => {
                self.v_regs[x] = self.v_regs[y];
            }
            (8, _, _, 1) => {
                self.v_regs[x] |= self.v_regs[y];
            }
            (8, _, _, 2) => {
                self.v_regs[x] &= self.v_regs[y];
            }
            (8, _, _, 3) => {
                self.v_regs[x] ^= self.v_regs[y];
            }
            (8, _, _, 4) => {
                let (sum, overflow) = self.v_regs[x].overflowing_add(self.v_regs[y]);
                self.v_regs[x] = sum;
                self.v_regs[0xF] = overflow as u8;
            }
            (8, _, _, 5) => {
                let (difference, overflow) = self.v_regs[x].overflowing_sub(self.v_regs[y]);
                self.v_regs[x] = difference;
                self.v_regs[0xF] = !overflow as u8;
            }
            (8, _, _, 6) => {
                self.v_regs[0xF] = self.v_regs[x] & 0x1;
                self.v_regs[x] = self.v_regs[x] >> 1;
            }
            (8, _, _, 7) => {
                let (difference, overflow) = self.v_regs[y].overflowing_sub(self.v_regs[x]);
                self.v_regs[x] = difference;
                self.v_regs[0xF] = !overflow as u8;
            }
            (8, _, _, 0xE) => {
                self.v_regs[0xF] = self.v_regs[x] >> 7;
                self.v_regs[x] = self.v_regs[x] << 1;
            }
            (9, _, _, 0) => {
                self.pc += if self.v_regs[x] != self.v_regs[y] {
                    2
                } else {
                    0
                };
            }
            (0xA, _, _, _) => {
                self.i = nnn;
            }
            (0xB, _, _, _) => {
                self.pc = nnn + self.v_regs[0] as u16;
            }
            (0xC, _, _, _) => {
                self.v_regs[x] = self.rng.gen::<u8>() & kk;
            }
            (0xD, _, _, _) => {
                let orig_x = self.v_regs[x] as usize;
                let orig_y = self.v_regs[y] as usize;
                let sprite_data = &self.memory[(self.i as usize)..(self.i as usize + n as usize)];

                let num_rows = sprite_data.len();
                const NUM_COLS: usize = 8;
                let mut collided = false;

                for r in 0..num_rows {
                    let row = sprite_data[r];
                    for c in 0..NUM_COLS {
                        let new_pixel = row >> (7 - c) & 0x01;
                        if new_pixel == 1 {
                            let pos_x = (orig_x + c) % SCREEN_WIDTH;
                            let pos_y = (orig_y + r) % SCREEN_HEIGHT;
                            let old_pixel = self.display[pos_y][pos_x];
                            if old_pixel == 1 {
                                collided = true
                            }
                            self.display[pos_y][pos_x] = new_pixel ^ old_pixel;
                        }
                    }
                }
                self.v_regs[0xF] = collided as u8;
            }
            (0xE, _, 9, 0xE) => {
                self.pc += if (1 << self.v_regs[x]) & self.keyboard != 0 {
                    2
                } else {
                    0
                };
            }
            (0xE, _, 0xA, 1) => {
                self.pc += if (1 << self.v_regs[x]) & self.keyboard == 0 {
                    2
                } else {
                    0
                };
            }
            (0xF, _, 0, 7) => {
                self.v_regs[x] = self.dt;
            }
            (0xF, _, 0, 0xA) => {
                self.pc -= 2;
                for i in 0..0x10 {
                    if 1 << i & self.keyboard != 0 {
                        self.v_regs[x] = i as u8;
                        self.pc += 2;
                        break;
                    }
                }
            }
            (0xF, _, 1, 5) => {
                self.dt = self.v_regs[x];
            }
            (0xF, _, 1, 8) => {
                self.st = self.v_regs[x];
            }
            (0xF, _, 1, 0xE) => {
                self.i += self.v_regs[x] as u16;
            }
            (0xF, _, 2, 9) => {
                // Each of the sprites are 5 bytes long and they are stored starting at 0x000
                self.i = self.v_regs[x] as u16 * 5;
            }
            (0xF, _, 3, 3) => {
                let num = self.v_regs[x];
                self.memory[self.i as usize] = num / 100;
                self.memory[self.i as usize + 1] = (num / 10) % 10;
                self.memory[self.i as usize + 2] = num % 10;
            }
            (0xF, _, 5, 5) => {
                self.memory[(self.i as usize)..=(self.i as usize + x as usize)]
                    .copy_from_slice(&self.v_regs[0..=x]);
            }
            (0xF, _, 6, 5) => {
                self.v_regs[0..=x].copy_from_slice(
                    &self.memory[(self.i as usize)..=(self.i as usize + x as usize)],
                );
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Chip8;
    use crate::STACK_SIZE;

    #[test]
    fn opcode_ret() {
        let mut stack = [0; STACK_SIZE];
        stack[0] = 0x0333;
        let mut chip = Chip8 {
            stack,
            sp: 1,
            ..Default::default()
        };
        chip.process_opcode(0x00EE);
        assert_eq!(chip.sp, 0);
        assert_eq!(chip.pc, 0x333);
    }

    #[test]
    fn opcode_jp() {
        let mut chip = Chip8::new();
        chip.process_opcode(0x1727);
        assert_eq!(chip.pc, 0x727);
    }

    #[test]
    fn opcode_call() {
        let mut chip = Chip8::new();
        chip.process_opcode(0x2727);
        assert_eq!(chip.sp, 1);
        assert_eq!(chip.stack[0], 0x200);
        assert_eq!(chip.pc, 0x727);
    }
}
