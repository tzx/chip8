const MEM_SIZE: usize = 4096;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_WIDTH: usize = 64;

struct Chip8 {
    memory: [u8; MEM_SIZE],
    v_regs: [u8; 16],        // general purpose registers
    dt: u8,                  // delay timer
    st: u8,                  // sound timer
    pc: u16,                 // program counter
    sp: u8,                  // stack pointer
    stack: [u16; 16],
    keyboard: u16,
    display: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}
