const MEM_SIZE: usize = 4096;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_WIDTH: usize = 64;
const NUM_V_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const PC_START: u16 = 0x200;

// can't derive default for const generics:
// https://github.com/rust-lang/rust/pull/60466#discussion_r280989938
struct Chip8 {
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
}

impl Default for Chip8 {
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
        }
    }
}

impl Chip8 {
    pub fn new() -> Self {
        // TODO: need to load program or some shit
        Chip8 { ..Default::default() }
    }

    pub fn process_opcode(&mut self, opcode: u16) {
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
            (0, 0, 0xE, 0) => {
                todo!("Clear display");
            }
            // RET
            (0, 0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            // JP addr
            (1, _, _, _) => {
                self.pc = nnn
            }
            // CALL addr
            (2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn
            }
            // SE Vx, byte
            (3, _, _, _) => {
                self.pc += if self.v_regs[x] == kk {2} else {0};
            }
            (4, _, _, _) => {
                self.pc += if self.v_regs[x] != kk {2} else {0};
            }
            (5, _, _, 0) => {
                self.pc += if self.v_regs[x] == self.v_regs[y as usize] {2} else {0};
            }
            (6, _, _, _) => {
                self.v_regs[x] = kk;
            }
            (7, _, _, _) => {
                self.v_regs[x] += kk;
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
                self.pc += if self.v_regs[x] != self.v_regs[y] {2} else {0};
            }
            (0xA, _, _, _) => {}
            (0xB, _, _, _) => {}
            (0xC, _, _, _) => {}
            (0xD, _, _, _) => {}
            (0xE, _, 9, 0xE) => {}
            (0xE, _, 0xA, 1) => {}
            (0xF, _, 0, 7) => {}
            (0xF, _, 0, 0xA) => {}
            (0xF, _, 1, 5) => {}
            (0xF, _, 1, 8) => {}
            (0xF, _, 1, 0xE) => {}
            (0xF, _, 2, 9) => {}
            (0xF, _, 3, 3) => {}
            (0xF, _, 5, 5) => {}
            (0xF, _, 6, 5) => {}
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::STACK_SIZE;
    use super::Chip8;

    #[test]
    fn opcode_ret() {
        let mut stack = [0; STACK_SIZE];
        stack[0] = 0x0333;
        let mut chip = Chip8 { stack, sp: 1, ..Default::default() };
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
