use cpu::registers::RegisterU8;
use cpu::registers::RegisterU16;
use cpu::registers::RegisterPair;
use cpu::registers::Target;



#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RstAddress {
  Rst00 = 0x0000,
  Rst08 = 0x0008,
  Rst38 = 0x0038
}

pub enum Instruction {
  LdR1R2(RegisterU8, RegisterU8),
  LdRFromMemHL(RegisterU8),
  LdMemHLFromR(RegisterU8),
  Nop,
  Unimplemented,
  JpNN,
  Cpl,
  Rst(RstAddress),
  Inc(RegisterU8),
  Dec(RegisterU8),
  Xor(RegisterU8),
  LdNNn(Target),
  IncNn(Target)
}

pub struct Optable {
  pub optable: [Instruction; 256]
}

impl Optable {
  pub fn new() -> Self {
    let mut table: [Instruction; 256] = [const { Instruction::Unimplemented }; 256];

    table[0x00] = Instruction::Nop;
    table[0x01] = Instruction::LdNNn(Target::Pair(RegisterPair::BC));
    table[0x03] = Instruction::IncNn(Target::Pair(RegisterPair::BC));
    table[0x04] = Instruction::Inc(RegisterU8::B);
    table[0x05] = Instruction::Dec(RegisterU8::B);
    table[0x0C] = Instruction::Inc(RegisterU8::C);
    table[0x0D] = Instruction::Dec(RegisterU8::C);
    table[0x11] = Instruction::LdNNn(Target::Pair(RegisterPair::DE));
    table[0x13] = Instruction::IncNn(Target::Pair(RegisterPair::DE));
    table[0x14] = Instruction::Inc(RegisterU8::D);
    table[0x15] = Instruction::Dec(RegisterU8::D);
    table[0x1C] = Instruction::Inc(RegisterU8::E);
    table[0x1D] = Instruction::Dec(RegisterU8::E);
    table[0x21] = Instruction::LdNNn(Target::Pair(RegisterPair::HL));
    table[0x23] = Instruction::IncNn(Target::Pair(RegisterPair::HL));
    table[0x24] = Instruction::Inc(RegisterU8::H);
    table[0x25] = Instruction::Dec(RegisterU8::H);
    table[0x2C] = Instruction::Inc(RegisterU8::L);
    table[0x2D] = Instruction::Dec(RegisterU8::L);
    table[0x2F] = Instruction::Cpl;
    table[0x31] = Instruction::LdNNn(Target::Single(RegisterU16::SP));
    table[0x33] = Instruction::IncNn(Target::Single(RegisterU16::SP));
    table[0x3C] = Instruction::Inc(RegisterU8::A);
    table[0x3D] = Instruction::Dec(RegisterU8::A);
    table[0x40] = Instruction::LdR1R2(RegisterU8::B, RegisterU8::B);
    table[0x41] = Instruction::LdR1R2(RegisterU8::B, RegisterU8::C);
    table[0x42] = Instruction::LdR1R2(RegisterU8::B, RegisterU8::D);
    table[0x43] = Instruction::LdR1R2(RegisterU8::B, RegisterU8::E);
    table[0x44] = Instruction::LdR1R2(RegisterU8::B, RegisterU8::H);
    table[0x45] = Instruction::LdR1R2(RegisterU8::B, RegisterU8::L);
    table[0x46] = Instruction::LdRFromMemHL(RegisterU8::B);
    table[0x47] = Instruction::LdR1R2(RegisterU8::B, RegisterU8::A);
    table[0x48] = Instruction::LdR1R2(RegisterU8::C, RegisterU8::B);
    table[0x49] = Instruction::LdR1R2(RegisterU8::C, RegisterU8::C);
    table[0x4A] = Instruction::LdR1R2(RegisterU8::C, RegisterU8::D);
    table[0x4B] = Instruction::LdR1R2(RegisterU8::C, RegisterU8::E);
    table[0x4C] = Instruction::LdR1R2(RegisterU8::C, RegisterU8::H);
    table[0x4D] = Instruction::LdR1R2(RegisterU8::C, RegisterU8::L);
    table[0x4E] = Instruction::LdRFromMemHL(RegisterU8::C);
    table[0x4F] = Instruction::LdR1R2(RegisterU8::C, RegisterU8::A);
    table[0x50] = Instruction::LdR1R2(RegisterU8::D, RegisterU8::B);
    table[0x51] = Instruction::LdR1R2(RegisterU8::D, RegisterU8::C);
    table[0x52] = Instruction::LdR1R2(RegisterU8::D, RegisterU8::D);
    table[0x53] = Instruction::LdR1R2(RegisterU8::D, RegisterU8::E);
    table[0x54] = Instruction::LdR1R2(RegisterU8::D, RegisterU8::H);
    table[0x55] = Instruction::LdR1R2(RegisterU8::D, RegisterU8::L);
    table[0x56] = Instruction::LdRFromMemHL(RegisterU8::D);
    table[0x57] = Instruction::LdR1R2(RegisterU8::D, RegisterU8::A);
    table[0x58] = Instruction::LdR1R2(RegisterU8::E, RegisterU8::B);
    table[0x59] = Instruction::LdR1R2(RegisterU8::E, RegisterU8::C);
    table[0x5A] = Instruction::LdR1R2(RegisterU8::E, RegisterU8::D);
    table[0x5B] = Instruction::LdR1R2(RegisterU8::E, RegisterU8::E);
    table[0x5C] = Instruction::LdR1R2(RegisterU8::E, RegisterU8::H);
    table[0x5D] = Instruction::LdR1R2(RegisterU8::E, RegisterU8::L);
    table[0x5E] = Instruction::LdRFromMemHL(RegisterU8::E);
    table[0x5F] = Instruction::LdR1R2(RegisterU8::E, RegisterU8::A);
    table[0x60] = Instruction::LdR1R2(RegisterU8::H, RegisterU8::B);
    table[0x61] = Instruction::LdR1R2(RegisterU8::H, RegisterU8::C);
    table[0x62] = Instruction::LdR1R2(RegisterU8::H, RegisterU8::D);
    table[0x63] = Instruction::LdR1R2(RegisterU8::H, RegisterU8::E);
    table[0x64] = Instruction::LdR1R2(RegisterU8::H, RegisterU8::H);
    table[0x65] = Instruction::LdR1R2(RegisterU8::H, RegisterU8::L);
    table[0x66] = Instruction::LdRFromMemHL(RegisterU8::H);
    table[0x67] = Instruction::LdR1R2(RegisterU8::H, RegisterU8::A);
    table[0x68] = Instruction::LdR1R2(RegisterU8::L, RegisterU8::B);
    table[0x69] = Instruction::LdR1R2(RegisterU8::L, RegisterU8::C);
    table[0x6A] = Instruction::LdR1R2(RegisterU8::L, RegisterU8::D);
    table[0x6B] = Instruction::LdR1R2(RegisterU8::L, RegisterU8::E);
    table[0x6C] = Instruction::LdR1R2(RegisterU8::L, RegisterU8::H);
    table[0x6D] = Instruction::LdR1R2(RegisterU8::L, RegisterU8::L);
    table[0x6E] = Instruction::LdRFromMemHL(RegisterU8::L);
    table[0x6F] = Instruction::LdR1R2(RegisterU8::L, RegisterU8::A);
    table[0x70] = Instruction::LdMemHLFromR(RegisterU8::B);
    table[0x71] = Instruction::LdMemHLFromR(RegisterU8::C);
    table[0x72] = Instruction::LdMemHLFromR(RegisterU8::D);
    table[0x73] = Instruction::LdMemHLFromR(RegisterU8::E);
    table[0x74] = Instruction::LdMemHLFromR(RegisterU8::H);
    table[0x75] = Instruction::LdMemHLFromR(RegisterU8::L);
    table[0x77] = Instruction::LdMemHLFromR(RegisterU8::A);
    table[0x78] = Instruction::LdR1R2(RegisterU8::A, RegisterU8::B);
    table[0x79] = Instruction::LdR1R2(RegisterU8::A, RegisterU8::C);
    table[0x7A] = Instruction::LdR1R2(RegisterU8::A, RegisterU8::D);
    table[0x7B] = Instruction::LdR1R2(RegisterU8::A, RegisterU8::E);
    table[0x7C] = Instruction::LdR1R2(RegisterU8::A, RegisterU8::H);
    table[0x7D] = Instruction::LdR1R2(RegisterU8::A, RegisterU8::L);
    table[0x7E] = Instruction::LdRFromMemHL(RegisterU8::A);
    table[0x7F] = Instruction::LdR1R2(RegisterU8::A, RegisterU8::A);
    table[0xA8] = Instruction::Xor(RegisterU8::B);
    table[0xA9] = Instruction::Xor(RegisterU8::C);
    table[0xAA] = Instruction::Xor(RegisterU8::D);
    table[0xAB] = Instruction::Xor(RegisterU8::H);
    table[0xAC] = Instruction::Xor(RegisterU8::L);
    table[0xAF] = Instruction::Xor(RegisterU8::A);
    table[0xC3] = Instruction::JpNN;
    table[0xC7] = Instruction::Rst(RstAddress::Rst00);
    table[0xCF] = Instruction::Rst(RstAddress::Rst08);
    table[0xFF] = Instruction::Rst(RstAddress::Rst38);

    Self { optable: table }
  }
}
