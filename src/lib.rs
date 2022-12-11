pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {}

const ZERO: [u8; 5] = [0xF0, 0x90, 0x90, 0x90, 0xF0];
const ONE: [u8; 5] = [0x20, 0x60, 0x20, 0x20, 0x70];
const TWO: [u8; 5] = [0xF0, 0x10, 0xF0, 0x80, 0xF0];
const THREE: [u8; 5] = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
const FOUR: [u8; 5] = [0x90, 0x90, 0xF0, 0x10, 0x10];
const FIVE: [u8; 5] = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
const SIX: [u8; 5] = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
const SEVEN: [u8; 5] = [0xF0, 0x10, 0x20, 0x40, 0x40];
const EIGHT: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0xF0];
const NINE: [u8; 5] = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
const A: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0x90];
const B: [u8; 5] = [0xE0, 0x90, 0xE0, 0x90, 0xE0];
const C: [u8; 5] = [0xF0, 0x80, 0x80, 0x80, 0xF0];
const D: [u8; 5] = [0xE0, 0x90, 0x90, 0x90, 0xE0];
const E: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
const F: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0x80];

const FONT: [[u8; 5]; 16] = [
    ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, A, B, C, D, E, F,
];

pub struct Chip8 {
    ram: [u8; 4096],
    registers: [u8; 16],
    display_buffer: [bool; 2048],
    delay_timer: Timer,
    sound_timer: Timer,
    program_counter: u16,
    stack: [u16; 16],
    index_register: u16,
}

impl Chip8 {
    pub fn load(path: &str) -> Result<Chip8> {
        todo!()
    }

    pub fn run(&mut self) -> ! {
        loop {
            // fetch
            let instruction = ((self.ram[self.program_counter as usize] as u16) << 8)
                + self.ram[self.program_counter as usize + 1] as u16;

            let op_code = (instruction >> 12) as u8;
            let x = n_nibbles(instruction, 1, 2) as usize;
            let y = n_nibbles(instruction, 1, 2) as usize;
            let n = n_nibbles(instruction, 1, 0) as u8;
            let nn = n_nibbles(instruction, 2, 0) as u8;
            let nnn = n_nibbles(instruction, 3, 0);

            // decode
            let op: Op = match op_code {
                0x0 => match n_nibbles(instruction, 3, 0) {
                    0x0E0 => Op::ClearScreen,
                    0x0EE => Op::Pop,
                    x => panic!("Unknown X: {x:#04x} for op code: 0x00"),
                },
                0x1 => Op::Jump(n_nibbles(instruction, 3, 0)),
                0x2 => Op::CallSubroutine(n_nibbles(instruction, 3, 0)),
                0x3 => Op::SkipEqual(x as usize, nn),
                0x4 => Op::SkipNotEqual(x as usize, nn),
                0x5 => Op::SkipRegisterEqual(x as usize, y as usize),
                0x6 => Op::Set(x as usize, nn),
                0x7 => Op::Add(x as usize, nn),
                0x8 => match n_nibbles(instruction, 1, 0) {
                    0x0 => Op::SetRegister(x as usize, y as usize),
                    0x1 => Op::OrRegister(x as usize, y as usize),
                    0x2 => Op::AndRegister(x as usize, y as usize),
                    0x3 => Op::XorRegister(x as usize, y as usize),
                    0x4 => Op::AddRegister(x as usize, y as usize),
                    0x5 => Op::SubtractYFromX(x as usize, y as usize),
                    0x6 => Op::ShiftRight(x as usize, y as usize),
                    0xE => Op::ShiftLeft(x as usize, y as usize),
                    0x7 => Op::SubtractXFromY(x as usize, y as usize),
                    x => panic!("{x:#04x} is not a logical or arithmetic instruction."),
                },
                0xA => Op::SetIndex(nnn),
                0xB => Op::JumpWithOffset(nnn),
                0xC => Op::Random(x, nn),
                0xD => Op::Display(x, y, n),
                0xE => match nn {
                    0x9E => Op::SkipIfPressed(x),
                    0xA1 => Op::SkipIfNotPressed(x),
                    x => panic!("{x:#04x} is not a recognised skip if key."),
                },
                0xF => match nn {
                    0x07 => Op::SetRegisterDelayTimer(x),
                    0x15 => Op::SetDelayTimer(x),
                    0x18 => Op::SetSoundTimer(x),
                    0x1E => Op::AddToIndex(x),
                    0x0A => Op::GetKey(x),
                    0x29 => Op::FontCharacter(x),
                    0x33 => Op::DecimalConversion(x),
                    0x55 => Op::Store(x),
                    0x65 => Op::Load(x),
                    x => panic!("{x:#04x} is not a recognised timer instruction."),
                },
                _ => todo!(),
            };

            // execute
            op.execute(self);
        }
    }
}

enum Op {
    ClearScreen,
    Pop,
    Jump(u16),
    CallSubroutine(u16),
    SkipEqual(usize, u8),
    SkipNotEqual(usize, u8),
    SkipRegisterEqual(usize, usize),
    Set(usize, u8),
    SetRegister(usize, usize),
    Add(usize, u8),
    OrRegister(usize, usize),
    AndRegister(usize, usize),
    XorRegister(usize, usize),
    AddRegister(usize, usize),
    SubtractYFromX(usize, usize),
    SubtractXFromY(usize, usize),
    ShiftRight(usize, usize),
    ShiftLeft(usize, usize),
    SetIndex(u16),
    JumpWithOffset(u16),
    Random(usize, u8),
    Display(usize, usize, u8),
    SkipIfPressed(usize),
    SkipIfNotPressed(usize),
    SetRegisterDelayTimer(usize),
    SetDelayTimer(usize),
    SetSoundTimer(usize),
    AddToIndex(usize),
    GetKey(usize),
    FontCharacter(usize),
    DecimalConversion(usize),
    Store(usize),
    Load(usize),
}

impl Op {
    fn execute(&self, chip8: &mut Chip8) {
        match self {
            Op::ClearScreen => {
                for i in 0..chip8.display_buffer.len() {
                    chip8.display_buffer[i] = false
                }
            }
            Op::Pop => todo!(),
            Op::Jump(nnn) => chip8.program_counter = *nnn,
            Op::CallSubroutine(_) => todo!(),
            Op::SkipEqual(_, _) => todo!(),
            Op::SkipNotEqual(_, _) => todo!(),
            Op::SkipRegisterEqual(_, _) => todo!(),
            Op::Set(x, nn) => chip8.registers[*x] = *nn,
            Op::SetRegister(x, y) => chip8.registers[*x] = chip8.registers[*y],
            Op::Add(_, _) => todo!(),
            Op::OrRegister(x, y) => chip8.registers[*x] |= chip8.registers[*y],
            Op::AndRegister(x, y) => chip8.registers[*x] &= chip8.registers[*y],
            Op::XorRegister(x, y) => chip8.registers[*x] ^= chip8.registers[*y],
            Op::AddRegister(x, y) => todo!(),
            Op::SubtractYFromX(_, _) => todo!(),
            Op::SubtractXFromY(_, _) => todo!(),
            Op::ShiftRight(_, _) => todo!(),
            Op::ShiftLeft(_, _) => todo!(),
            Op::SetIndex(nnn) => chip8.index_register = *nnn,
            Op::JumpWithOffset(_) => todo!(),
            Op::Random(_, _) => todo!(),
            Op::Display(_, _, _) => todo!(),
            Op::SkipIfPressed(_) => todo!(),
            Op::SkipIfNotPressed(_) => todo!(),
            Op::SetRegisterDelayTimer(_) => todo!(),
            Op::SetDelayTimer(_) => todo!(),
            Op::SetSoundTimer(_) => todo!(),
            Op::AddToIndex(_) => todo!(),
            Op::GetKey(_) => todo!(),
            Op::FontCharacter(_) => todo!(),
            Op::DecimalConversion(_) => todo!(),
            Op::Store(_) => todo!(),
            Op::Load(_) => todo!(),
        }
    }
}

struct Timer(u8);

#[inline]
fn n_nibbles(value: u16, n: usize, offset: usize) -> u16 {
    value & ((0xFFFF >> (16 - n * 4)) << offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_nibbles_12bits_no_offset_test() {
        let v = 0x00E0;

        assert_eq!(0x0E0, n_nibbles(v, 3, 0));
    }

    #[test]
    fn n_nibbles_4bits_no_offset_test() {
        let v = 0xAEFA;
        assert_eq!(0xA, n_nibbles(v, 1, 0));
    }
}
