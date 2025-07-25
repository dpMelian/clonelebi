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
  Nop,
  Unimplemented,
  JpNN,
  Cpl,
  Rst(RstAddress),
  Inc(RegisterU8),
  Xor(RegisterU8),
  LdNNn(Target)
}

pub struct Optable {
  pub optable: [Instruction; 256]
}

impl Optable {
  pub fn new() -> Self {
    let mut table: [Instruction; 256] = [const { Instruction::Unimplemented }; 256];

    table[0x00] = Instruction::Nop;
    table[0x01] = Instruction::LdNNn(Target::Pair(RegisterPair::BC));
    table[0x04] = Instruction::Inc(RegisterU8::B);
    table[0x0C] = Instruction::Inc(RegisterU8::C);
    table[0x11] = Instruction::LdNNn(Target::Pair(RegisterPair::DE));
    table[0x14] = Instruction::Inc(RegisterU8::D);
    table[0x1C] = Instruction::Inc(RegisterU8::E);
    table[0x21] = Instruction::LdNNn(Target::Pair(RegisterPair::HL));
    table[0x24] = Instruction::Inc(RegisterU8::H);
    table[0x2C] = Instruction::Inc(RegisterU8::L);
    table[0x2F] = Instruction::Cpl;
    table[0x31] = Instruction::LdNNn(Target::Single(RegisterU16::SP));
    table[0x3C] = Instruction::Inc(RegisterU8::A);
    table[0x40] = Instruction::LdR1R2(RegisterU8::B, RegisterU8::B);
    table[0x41] = Instruction::LdR1R2(RegisterU8::B, RegisterU8::C);
    table[0x42] = Instruction::LdR1R2(RegisterU8::B, RegisterU8::D);
    table[0x43] = Instruction::LdR1R2(RegisterU8::B, RegisterU8::E);
    table[0x44] = Instruction::LdR1R2(RegisterU8::B, RegisterU8::H);
    table[0x45] = Instruction::LdR1R2(RegisterU8::B, RegisterU8::L);
    table[0x48] = Instruction::LdR1R2(RegisterU8::C, RegisterU8::B);
    table[0x49] = Instruction::LdR1R2(RegisterU8::C, RegisterU8::C);
    table[0x4A] = Instruction::LdR1R2(RegisterU8::C, RegisterU8::D);
    table[0x4B] = Instruction::LdR1R2(RegisterU8::C, RegisterU8::E);
    table[0x4C] = Instruction::LdR1R2(RegisterU8::C, RegisterU8::H);
    table[0x5B] = Instruction::LdR1R2(RegisterU8::E, RegisterU8::E);
    table[0x5C] = Instruction::LdR1R2(RegisterU8::E, RegisterU8::H);
    table[0x60] = Instruction::LdR1R2(RegisterU8::H, RegisterU8::B);
    table[0x78] = Instruction::LdR1R2(RegisterU8::A, RegisterU8::B);
    table[0x79] = Instruction::LdR1R2(RegisterU8::A, RegisterU8::C);
    table[0x7A] = Instruction::LdR1R2(RegisterU8::A, RegisterU8::D);
    table[0x7B] = Instruction::LdR1R2(RegisterU8::A, RegisterU8::E);
    table[0x7C] = Instruction::LdR1R2(RegisterU8::A, RegisterU8::H);
    table[0x7D] = Instruction::LdR1R2(RegisterU8::A, RegisterU8::L);
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
