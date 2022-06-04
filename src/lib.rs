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
    fn process_opcode(&mut self, opcode: u16) {
        let nnn = opcode & 0x0FFF;
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        let x = nibbles.1;
        let y = nibbles.2;
        let n = nibbles.3;
        let kk = opcode & 0x0FF;

        match nibbles {
            // CLS
            (0, 0, 0xE, 0) => {
                todo!("Clear display");
            }
            // RET
            (0, 0, 0xE, 0xE) => {}
            // JP addr
            (1, _, _, _) => {}
            // CALL addr
            (2, _, _, _) => {}
            // SE Vx, byte
            (3, _, _, _) => {}
            (4, _, _, _) => {}
            (5, _, _, 0) => {}
            (6, _, _, _) => {}
            (7, _, _, _) => {}
            (8, _, _, 0) => {}
            (8, _, _, 1) => {}
            (8, _, _, 2) => {}
            (8, _, _, 3) => {}
            (8, _, _, 4) => {}
            (8, _, _, 5) => {}
            (8, _, _, 6) => {}
            (8, _, _, 7) => {}
            (8, _, _, 0xE) => {}
            (9, _, _, 0) => {}
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
