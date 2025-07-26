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
  Cpl,
  Dec(RegisterU8),
  Di,
  Inc(RegisterU8),
  IncNn(Target),
  Invalid,
  JpNN,
  LdMemHLFromR(RegisterU8),
  LdNnN(RegisterU8),
  LdNNn(Target),
  LdR1R2(RegisterU8, RegisterU8),
  LdRFromMemHL(RegisterU8),
  Nop,
  Rst(RstAddress),
  Unimplemented,
  Xor(RegisterU8)
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
    table[0x06] = Instruction::LdNnN(RegisterU8::B);
    table[0x0C] = Instruction::Inc(RegisterU8::C);
    table[0x0D] = Instruction::Dec(RegisterU8::C);
    table[0x0E] = Instruction::LdNnN(RegisterU8::C);
    table[0x11] = Instruction::LdNNn(Target::Pair(RegisterPair::DE));
    table[0x13] = Instruction::IncNn(Target::Pair(RegisterPair::DE));
    table[0x14] = Instruction::Inc(RegisterU8::D);
    table[0x15] = Instruction::Dec(RegisterU8::D);
    table[0x16] = Instruction::LdNnN(RegisterU8::D);
    table[0x1C] = Instruction::Inc(RegisterU8::E);
    table[0x1D] = Instruction::Dec(RegisterU8::E);
    table[0x1E] = Instruction::LdNnN(RegisterU8::E);
    table[0x21] = Instruction::LdNNn(Target::Pair(RegisterPair::HL));
    table[0x23] = Instruction::IncNn(Target::Pair(RegisterPair::HL));
    table[0x24] = Instruction::Inc(RegisterU8::H);
    table[0x25] = Instruction::Dec(RegisterU8::H);
    table[0x26] = Instruction::LdNnN(RegisterU8::H);
    table[0x2C] = Instruction::Inc(RegisterU8::L);
    table[0x2D] = Instruction::Dec(RegisterU8::L);
    table[0x2E] = Instruction::LdNnN(RegisterU8::L);
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
    table[0xD3] = Instruction::Invalid;
    table[0xDB] = Instruction::Invalid;
    table[0xDD] = Instruction::Invalid;
    table[0xE3] = Instruction::Invalid;
    table[0xE4] = Instruction::Invalid;
    table[0xEB] = Instruction::Invalid;
    table[0xEC] = Instruction::Invalid;
    table[0xED] = Instruction::Invalid;
    table[0xF3] = Instruction::Di;
    table[0xF4] = Instruction::Invalid;
    table[0xFC] = Instruction::Invalid;
    table[0xFD] = Instruction::Invalid;
    table[0xFF] = Instruction::Rst(RstAddress::Rst38);

    Self { optable: table }
  }
}

pub struct CycleTable {
  pub cycle_table: [u64; 256]
}

impl CycleTable {
  pub fn new() -> Self {
    let mut table: [u64; 256] = [0; 256];

    table[0x00] = 4;
    table[0x01] = 12;
    table[0x02] = 8;
    table[0x03] = 8;
    table[0x04] = 4;
    table[0x05] = 4;
    table[0x06] = 8;
    table[0x07] = 4;
    table[0x08] = 20;
    table[0x09] = 8;
    table[0x0A] = 8;
    table[0x0B] = 8;
    table[0x0C] = 4;
    table[0x0D] = 4;
    table[0x0E] = 8;
    table[0x0F] = 4;

    table[0x10] = 4;
    table[0x11] = 12;
    table[0x12] = 8;
    table[0x13] = 8;
    table[0x14] = 4;
    table[0x15] = 4;
    table[0x16] = 8;
    table[0x17] = 4;
    table[0x18] = 12;
    table[0x19] = 8;
    table[0x1A] = 8;
    table[0x1B] = 8;
    table[0x1C] = 4;
    table[0x1D] = 4;
    table[0x1E] = 8;
    table[0x1F] = 4;

    // table[0x20] = ;
    table[0x21] = 12;
    table[0x22] = 8;
    table[0x23] = 8;
    table[0x24] = 4;
    table[0x25] = 4;
    table[0x26] = 8;
    table[0x27] = 4;
    // table[0x28] = ;
    table[0x29] = 8;
    table[0x2A] = 8;
    table[0x2B] = 8;
    table[0x2C] = 4;
    table[0x2D] = 4;
    table[0x2E] = 8;
    table[0x2F] = 4;

    // table[0x30] = ;
    table[0x31] = 12;
    table[0x32] = 8;
    table[0x33] = 8;
    table[0x34] = 12;
    table[0x35] = 12;
    table[0x36] = 12;
    table[0x37] = 4;
    // table[0x38] = ;
    table[0x39] = 8;
    table[0x3A] = 8;
    table[0x3B] = 8;
    table[0x3C] = 4;
    table[0x3D] = 4;
    table[0x3E] = 8;
    table[0x3F] = 4;

    table[0x40] = 4;
    table[0x41] = 4;
    table[0x42] = 4;
    table[0x43] = 4;
    table[0x44] = 4;
    table[0x45] = 4;
    table[0x46] = 8;
    table[0x47] = 4;
    table[0x48] = 4;
    table[0x49] = 4;
    table[0x4A] = 4;
    table[0x4B] = 4;
    table[0x4C] = 4;
    table[0x4D] = 4;
    table[0x4E] = 8;
    table[0x4F] = 4;

    table[0x50] = 4;
    table[0x51] = 4;
    table[0x52] = 4;
    table[0x53] = 4;
    table[0x54] = 4;
    table[0x55] = 4;
    table[0x56] = 8;
    table[0x57] = 4;
    table[0x58] = 4;
    table[0x59] = 4;
    table[0x5A] = 4;
    table[0x5B] = 4;
    table[0x5C] = 4;
    table[0x5D] = 4;
    table[0x5E] = 8;
    table[0x5F] = 4;

    table[0x60] = 4;
    table[0x61] = 4;
    table[0x62] = 4;
    table[0x63] = 4;
    table[0x64] = 4;
    table[0x65] = 4;
    table[0x66] = 8;
    table[0x67] = 4;
    table[0x68] = 4;
    table[0x69] = 4;
    table[0x6A] = 4;
    table[0x6B] = 4;
    table[0x6C] = 4;
    table[0x6D] = 4;
    table[0x6E] = 8;
    table[0x6F] = 4;

    table[0x70] = 8;
    table[0x71] = 8;
    table[0x72] = 8;
    table[0x73] = 8;
    table[0x74] = 8;
    table[0x75] = 8;
    table[0x76] = 4;
    table[0x77] = 8;
    table[0x78] = 4;
    table[0x79] = 4;
    table[0x7A] = 4;
    table[0x7B] = 4;
    table[0x7C] = 4;
    table[0x7D] = 4;
    table[0x7E] = 8;
    table[0x7F] = 4;

    table[0x80] = 4;
    table[0x81] = 4;
    table[0x82] = 4;
    table[0x83] = 4;
    table[0x84] = 4;
    table[0x85] = 4;
    table[0x86] = 8;
    table[0x87] = 4;
    table[0x88] = 4;
    table[0x89] = 4;
    table[0x8A] = 4;
    table[0x8B] = 4;
    table[0x8C] = 4;
    table[0x8D] = 4;
    table[0x8E] = 8;
    table[0x8F] = 4;

    table[0x90] = 4;
    table[0x91] = 4;
    table[0x92] = 4;
    table[0x93] = 4;
    table[0x94] = 4;
    table[0x95] = 4;
    table[0x96] = 8;
    table[0x97] = 4;
    table[0x98] = 4;
    table[0x99] = 4;
    table[0x9A] = 4;
    table[0x9B] = 4;
    table[0x9C] = 4;
    table[0x9D] = 4;
    table[0x9E] = 8;
    table[0x9F] = 4;

    table[0xA0] = 4;
    table[0xA1] = 4;
    table[0xA2] = 4;
    table[0xA3] = 4;
    table[0xA4] = 4;
    table[0xA5] = 4;
    table[0xA6] = 8;
    table[0xA7] = 4;
    table[0xA8] = 4;
    table[0xA9] = 4;
    table[0xAA] = 4;
    table[0xAB] = 4;
    table[0xAC] = 4;
    table[0xAD] = 4;
    table[0xAE] = 8;
    table[0xAF] = 4;

    table[0xB0] = 4;
    table[0xB1] = 4;
    table[0xB2] = 4;
    table[0xB3] = 4;
    table[0xB4] = 4;
    table[0xB5] = 4;
    table[0xB6] = 8;
    table[0xB7] = 4;
    table[0xB8] = 4;
    table[0xB9] = 4;
    table[0xBA] = 4;
    table[0xBB] = 4;
    table[0xBC] = 4;
    table[0xBD] = 4;
    table[0xBE] = 8;
    table[0xBF] = 4;

    // table[0xC0] = ;
    table[0xC1] = 12;
    // table[0xC2] = ;
    table[0xC3] = 16;
    // table[0xC4] = ;
    table[0xC5] = 16;
    table[0xC6] = 8;
    table[0xC7] = 16;
    // table[0xC8] = ;
    table[0xC9] = 16;
    // table[0xCA] = ;
    table[0xCB] = 4;
    // table[0xCC] = ;
    table[0xCD] = 24;
    table[0xCE] = 8;
    table[0xCF] = 16;

    // table[0xD0] = ;
    table[0xD1] = 12;
    // table[0xD2] = ;

    // table[0xD4] = ;
    table[0xD5] = 16;
    table[0xD6] = 8;
    table[0xD7] = 16;
    // table[0xD8] = ;
    table[0xD9] = 16;
    // table[0xDA] = ;

    // table[0xDC] = ;

    table[0xDE] = 8;
    table[0xDF] = 16;

    table[0xE0] = 12;
    table[0xE1] = 12;
    table[0xE2] = 8;


    table[0xE5] = 16;
    table[0xE6] = 8;
    table[0xE7] = 16;
    table[0xE8] = 16;
    table[0xE9] = 4;
    table[0xEA] = 16;



    table[0xEE] = 8;
    table[0xEF] = 16;

    table[0xF0] = 12;
    table[0xF1] = 12;
    table[0xF2] = 8;
    table[0xF3] = 4;

    table[0xF5] = 16;
    table[0xF6] = 8;
    table[0xF7] = 16;
    table[0xF8] = 12;
    table[0xF9] = 8;
    table[0xFA] = 16;
    table[0xFB] = 4;


    table[0xFE] = 8;
    table[0xFF] = 16;
    
    Self { cycle_table: table }
  }
}
