use cpu::registers::RegisterU8;

pub enum PrefixedInstruction {
  CBRlcR(RegisterU8),
  CBRlcHL(),
  CBRlR(RegisterU8),
  CBRlHL(),
  CBRrcR(RegisterU8),
  CBRrcHL(),
  CBRrR(RegisterU8),
  CBRrHL(),
  CBSrlR(RegisterU8),
  CBSrlHL(),
  CBSlaR(RegisterU8),
  CBSlaHL(),
  CBSraR(RegisterU8),
  CBSraHL(),
  CBSwapR(RegisterU8),
  CBSwapHL(),
  CBBitBR(i32, RegisterU8),
  CBBitBHL(i32),
  CBResBR(i32, RegisterU8),
  CBResHL(i32),
  CBSetBR(i32, RegisterU8),
  CBSetBHL(usize),
  Unimplemented,
}

pub struct PrefixedOptable {
  pub prefixed_optable: [PrefixedInstruction; 256]
}

impl PrefixedOptable {
  pub fn new() -> Self {
    let mut table: [PrefixedInstruction; 256] = [const { PrefixedInstruction::Unimplemented }; 256];


    table[0x00] = PrefixedInstruction::CBRlcR(RegisterU8::B);
    table[0x01] = PrefixedInstruction::CBRlcR(RegisterU8::C);
    table[0x02] = PrefixedInstruction::CBRlcR(RegisterU8::D);
    table[0x03] = PrefixedInstruction::CBRlcR(RegisterU8::E);
    table[0x04] = PrefixedInstruction::CBRlcR(RegisterU8::H);
    table[0x05] = PrefixedInstruction::CBRlcR(RegisterU8::L);
    table[0x06] = PrefixedInstruction::CBRlcHL();
    table[0x07] = PrefixedInstruction::CBRlcR(RegisterU8::A);
    table[0x08] = PrefixedInstruction::CBRrcR(RegisterU8::B);
    table[0x09] = PrefixedInstruction::CBRrcR(RegisterU8::C);
    table[0x0A] = PrefixedInstruction::CBRrcR(RegisterU8::D);
    table[0x0B] = PrefixedInstruction::CBRrcR(RegisterU8::E);
    table[0x0C] = PrefixedInstruction::CBRrcR(RegisterU8::H);
    table[0x0D] = PrefixedInstruction::CBRrcR(RegisterU8::L);
    table[0x0E] = PrefixedInstruction::CBRrcHL();
    table[0x0F] = PrefixedInstruction::CBRrcR(RegisterU8::A);
    table[0x10] = PrefixedInstruction::CBRlR(RegisterU8::B);
    table[0x11] = PrefixedInstruction::CBRlR(RegisterU8::C);
    table[0x12] = PrefixedInstruction::CBRlR(RegisterU8::D);
    table[0x13] = PrefixedInstruction::CBRlR(RegisterU8::E);
    table[0x14] = PrefixedInstruction::CBRlR(RegisterU8::H);
    table[0x15] = PrefixedInstruction::CBRlR(RegisterU8::L);
    table[0x16] = PrefixedInstruction::CBRlHL();
    table[0x17] = PrefixedInstruction::CBRlR(RegisterU8::A);
    table[0x18] = PrefixedInstruction::CBRrR(RegisterU8::B);
    table[0x19] = PrefixedInstruction::CBRrR(RegisterU8::C);
    table[0x1A] = PrefixedInstruction::CBRrR(RegisterU8::D);
    table[0x1B] = PrefixedInstruction::CBRrR(RegisterU8::E);
    table[0x1C] = PrefixedInstruction::CBRrR(RegisterU8::H);
    table[0x1D] = PrefixedInstruction::CBRrR(RegisterU8::L);
    table[0x1E] = PrefixedInstruction::CBRrHL();
    table[0x1F] = PrefixedInstruction::CBRrR(RegisterU8::A);
    table[0x20] = PrefixedInstruction::CBSlaR(RegisterU8::B);
    table[0x21] = PrefixedInstruction::CBSlaR(RegisterU8::C);
    table[0x22] = PrefixedInstruction::CBSlaR(RegisterU8::D);
    table[0x23] = PrefixedInstruction::CBSlaR(RegisterU8::E);
    table[0x24] = PrefixedInstruction::CBSlaR(RegisterU8::H);
    table[0x25] = PrefixedInstruction::CBSlaR(RegisterU8::L);
    table[0x26] = PrefixedInstruction::CBSlaHL();
    table[0x27] = PrefixedInstruction::CBSlaR(RegisterU8::A);
    table[0x28] = PrefixedInstruction::CBSraR(RegisterU8::B);
    table[0x29] = PrefixedInstruction::CBSraR(RegisterU8::C);
    table[0x2A] = PrefixedInstruction::CBSraR(RegisterU8::D);
    table[0x2B] = PrefixedInstruction::CBSraR(RegisterU8::E);
    table[0x2C] = PrefixedInstruction::CBSraR(RegisterU8::H);
    table[0x2D] = PrefixedInstruction::CBSraR(RegisterU8::L);
    table[0x2E] = PrefixedInstruction::CBSraHL();
    table[0x2F] = PrefixedInstruction::CBSraR(RegisterU8::A);
    table[0x30] = PrefixedInstruction::CBSwapR(RegisterU8::B);
    table[0x31] = PrefixedInstruction::CBSwapR(RegisterU8::C);
    table[0x32] = PrefixedInstruction::CBSwapR(RegisterU8::D);
    table[0x33] = PrefixedInstruction::CBSwapR(RegisterU8::E);
    table[0x34] = PrefixedInstruction::CBSwapR(RegisterU8::H);
    table[0x35] = PrefixedInstruction::CBSwapR(RegisterU8::L);
    table[0x36] = PrefixedInstruction::CBSwapHL();
    table[0x37] = PrefixedInstruction::CBSwapR(RegisterU8::A);
    table[0x38] = PrefixedInstruction::CBSrlR(RegisterU8::B);
    table[0x39] = PrefixedInstruction::CBSrlR(RegisterU8::C);
    table[0x3A] = PrefixedInstruction::CBSrlR(RegisterU8::D);
    table[0x3B] = PrefixedInstruction::CBSrlR(RegisterU8::E);
    table[0x3C] = PrefixedInstruction::CBSrlR(RegisterU8::H);
    table[0x3D] = PrefixedInstruction::CBSrlR(RegisterU8::L);
    table[0x3E] = PrefixedInstruction::CBSrlHL();
    table[0x3F] = PrefixedInstruction::CBSrlR(RegisterU8::A);
    table[0x40] = PrefixedInstruction::CBBitBR(0, RegisterU8::B);
    table[0x41] = PrefixedInstruction::CBBitBR(0, RegisterU8::C);
    table[0x42] = PrefixedInstruction::CBBitBR(0, RegisterU8::D);
    table[0x43] = PrefixedInstruction::CBBitBR(0, RegisterU8::E);
    table[0x44] = PrefixedInstruction::CBBitBR(0, RegisterU8::H);
    table[0x45] = PrefixedInstruction::CBBitBR(0, RegisterU8::L);
    table[0x46] = PrefixedInstruction::CBBitBHL(0);
    table[0x47] = PrefixedInstruction::CBBitBR(0, RegisterU8::A);
    table[0x48] = PrefixedInstruction::CBBitBR(1, RegisterU8::B);
    table[0x49] = PrefixedInstruction::CBBitBR(1, RegisterU8::C);
    table[0x4A] = PrefixedInstruction::CBBitBR(1, RegisterU8::D);
    table[0x4B] = PrefixedInstruction::CBBitBR(1, RegisterU8::E);
    table[0x4C] = PrefixedInstruction::CBBitBR(1, RegisterU8::H);
    table[0x4D] = PrefixedInstruction::CBBitBR(1, RegisterU8::L);
    table[0x4E] = PrefixedInstruction::CBBitBHL(1);
    table[0x4F] = PrefixedInstruction::CBBitBR(1, RegisterU8::A);
    table[0x50] = PrefixedInstruction::CBBitBR(2, RegisterU8::B);
    table[0x51] = PrefixedInstruction::CBBitBR(2, RegisterU8::C);
    table[0x52] = PrefixedInstruction::CBBitBR(2, RegisterU8::D);
    table[0x53] = PrefixedInstruction::CBBitBR(2, RegisterU8::E);
    table[0x54] = PrefixedInstruction::CBBitBR(2, RegisterU8::H);
    table[0x55] = PrefixedInstruction::CBBitBR(2, RegisterU8::L);
    table[0x56] = PrefixedInstruction::CBBitBHL(2);
    table[0x57] = PrefixedInstruction::CBBitBR(2, RegisterU8::A);
    table[0x58] = PrefixedInstruction::CBBitBR(3, RegisterU8::B);
    table[0x59] = PrefixedInstruction::CBBitBR(3, RegisterU8::C);
    table[0x5A] = PrefixedInstruction::CBBitBR(3, RegisterU8::D);
    table[0x5B] = PrefixedInstruction::CBBitBR(3, RegisterU8::E);
    table[0x5C] = PrefixedInstruction::CBBitBR(3, RegisterU8::H);
    table[0x5D] = PrefixedInstruction::CBBitBR(3, RegisterU8::L);
    table[0x5E] = PrefixedInstruction::CBBitBHL(3);
    table[0x5F] = PrefixedInstruction::CBBitBR(3, RegisterU8::A);
    table[0x60] = PrefixedInstruction::CBBitBR(4, RegisterU8::B);
    table[0x61] = PrefixedInstruction::CBBitBR(4, RegisterU8::C);
    table[0x62] = PrefixedInstruction::CBBitBR(4, RegisterU8::D);
    table[0x63] = PrefixedInstruction::CBBitBR(4, RegisterU8::E);
    table[0x64] = PrefixedInstruction::CBBitBR(4, RegisterU8::H);
    table[0x65] = PrefixedInstruction::CBBitBR(4, RegisterU8::L);
    table[0x66] = PrefixedInstruction::CBBitBHL(4);
    table[0x67] = PrefixedInstruction::CBBitBR(4, RegisterU8::A);
    table[0x68] = PrefixedInstruction::CBBitBR(5, RegisterU8::B);
    table[0x69] = PrefixedInstruction::CBBitBR(5, RegisterU8::C);
    table[0x6A] = PrefixedInstruction::CBBitBR(5, RegisterU8::D);
    table[0x6B] = PrefixedInstruction::CBBitBR(5, RegisterU8::E);
    table[0x6C] = PrefixedInstruction::CBBitBR(5, RegisterU8::H);
    table[0x6D] = PrefixedInstruction::CBBitBR(5, RegisterU8::L);
    table[0x6E] = PrefixedInstruction::CBBitBHL(5);
    table[0x6F] = PrefixedInstruction::CBBitBR(5, RegisterU8::A);
    table[0x70] = PrefixedInstruction::CBBitBR(6, RegisterU8::B);
    table[0x71] = PrefixedInstruction::CBBitBR(6, RegisterU8::C);
    table[0x72] = PrefixedInstruction::CBBitBR(6, RegisterU8::D);
    table[0x73] = PrefixedInstruction::CBBitBR(6, RegisterU8::E);
    table[0x74] = PrefixedInstruction::CBBitBR(6, RegisterU8::H);
    table[0x75] = PrefixedInstruction::CBBitBR(6, RegisterU8::L);
    table[0x76] = PrefixedInstruction::CBBitBHL(6);
    table[0x77] = PrefixedInstruction::CBBitBR(6, RegisterU8::A);
    table[0x78] = PrefixedInstruction::CBBitBR(7, RegisterU8::B);
    table[0x79] = PrefixedInstruction::CBBitBR(7, RegisterU8::C);
    table[0x7A] = PrefixedInstruction::CBBitBR(7, RegisterU8::D);
    table[0x7B] = PrefixedInstruction::CBBitBR(7, RegisterU8::E);
    table[0x7C] = PrefixedInstruction::CBBitBR(7, RegisterU8::H);
    table[0x7D] = PrefixedInstruction::CBBitBR(7, RegisterU8::L);
    table[0x7E] = PrefixedInstruction::CBBitBHL(7);
    table[0x7F] = PrefixedInstruction::CBBitBR(7, RegisterU8::A);
    table[0x80] = PrefixedInstruction::CBResBR(0, RegisterU8::B);
    table[0x81] = PrefixedInstruction::CBResBR(0, RegisterU8::C);
    table[0x82] = PrefixedInstruction::CBResBR(0, RegisterU8::D);
    table[0x83] = PrefixedInstruction::CBResBR(0, RegisterU8::E);
    table[0x84] = PrefixedInstruction::CBResBR(0, RegisterU8::H);
    table[0x85] = PrefixedInstruction::CBResBR(0, RegisterU8::L);
    table[0x86] = PrefixedInstruction::CBResHL(0);
    table[0x87] = PrefixedInstruction::CBResBR(0, RegisterU8::A);
    table[0x88] = PrefixedInstruction::CBResBR(1, RegisterU8::B);
    table[0x89] = PrefixedInstruction::CBResBR(1, RegisterU8::C);
    table[0x8A] = PrefixedInstruction::CBResBR(1, RegisterU8::D);
    table[0x8B] = PrefixedInstruction::CBResBR(1, RegisterU8::E);
    table[0x8C] = PrefixedInstruction::CBResBR(1, RegisterU8::H);
    table[0x8D] = PrefixedInstruction::CBResBR(1, RegisterU8::L);
    table[0x8E] = PrefixedInstruction::CBResHL(1);
    table[0x8F] = PrefixedInstruction::CBResBR(1, RegisterU8::A);
    table[0x90] = PrefixedInstruction::CBResBR(2, RegisterU8::B);
    table[0x91] = PrefixedInstruction::CBResBR(2, RegisterU8::C);
    table[0x92] = PrefixedInstruction::CBResBR(2, RegisterU8::D);
    table[0x93] = PrefixedInstruction::CBResBR(2, RegisterU8::E);
    table[0x94] = PrefixedInstruction::CBResBR(2, RegisterU8::H);
    table[0x95] = PrefixedInstruction::CBResBR(2, RegisterU8::L);
    table[0x96] = PrefixedInstruction::CBResHL(2);
    table[0x97] = PrefixedInstruction::CBResBR(2, RegisterU8::A);
    table[0x98] = PrefixedInstruction::CBResBR(3, RegisterU8::B);
    table[0x99] = PrefixedInstruction::CBResBR(3, RegisterU8::C);
    table[0x9A] = PrefixedInstruction::CBResBR(3, RegisterU8::D);
    table[0x9B] = PrefixedInstruction::CBResBR(3, RegisterU8::E);
    table[0x9C] = PrefixedInstruction::CBResBR(3, RegisterU8::H);
    table[0x9D] = PrefixedInstruction::CBResBR(3, RegisterU8::L);
    table[0x9E] = PrefixedInstruction::CBResHL(3);
    table[0x9F] = PrefixedInstruction::CBResBR(3, RegisterU8::A);
    table[0xA0] = PrefixedInstruction::CBResBR(4, RegisterU8::B);
    table[0xA1] = PrefixedInstruction::CBResBR(4, RegisterU8::C);
    table[0xA2] = PrefixedInstruction::CBResBR(4, RegisterU8::D);
    table[0xA3] = PrefixedInstruction::CBResBR(4, RegisterU8::E);
    table[0xA4] = PrefixedInstruction::CBResBR(4, RegisterU8::H);
    table[0xA5] = PrefixedInstruction::CBResBR(4, RegisterU8::L);
    table[0xA6] = PrefixedInstruction::CBResHL(4);
    table[0xA7] = PrefixedInstruction::CBResBR(4, RegisterU8::A);
    table[0xA8] = PrefixedInstruction::CBResBR(5, RegisterU8::B);
    table[0xA9] = PrefixedInstruction::CBResBR(5, RegisterU8::C);
    table[0xAA] = PrefixedInstruction::CBResBR(5, RegisterU8::D);
    table[0xAB] = PrefixedInstruction::CBResBR(5, RegisterU8::E);
    table[0xAC] = PrefixedInstruction::CBResBR(5, RegisterU8::H);
    table[0xAD] = PrefixedInstruction::CBResBR(5, RegisterU8::L);
    table[0xAE] = PrefixedInstruction::CBResHL(5);
    table[0xAF] = PrefixedInstruction::CBResBR(5, RegisterU8::A);
    table[0xB0] = PrefixedInstruction::CBResBR(6, RegisterU8::B);
    table[0xB1] = PrefixedInstruction::CBResBR(6, RegisterU8::C);
    table[0xB2] = PrefixedInstruction::CBResBR(6, RegisterU8::D);
    table[0xB3] = PrefixedInstruction::CBResBR(6, RegisterU8::E);
    table[0xB4] = PrefixedInstruction::CBResBR(6, RegisterU8::H);
    table[0xB5] = PrefixedInstruction::CBResBR(6, RegisterU8::L);
    table[0xB6] = PrefixedInstruction::CBResHL(6);
    table[0xB7] = PrefixedInstruction::CBResBR(6, RegisterU8::A);
    table[0xB8] = PrefixedInstruction::CBResBR(7, RegisterU8::B);
    table[0xB9] = PrefixedInstruction::CBResBR(7, RegisterU8::C);
    table[0xBA] = PrefixedInstruction::CBResBR(7, RegisterU8::D);
    table[0xBB] = PrefixedInstruction::CBResBR(7, RegisterU8::E);
    table[0xBC] = PrefixedInstruction::CBResBR(7, RegisterU8::H);
    table[0xBD] = PrefixedInstruction::CBResBR(7, RegisterU8::L);
    table[0xBE] = PrefixedInstruction::CBResHL(7);
    table[0xBF] = PrefixedInstruction::CBResBR(7, RegisterU8::A);
    table[0xC0] = PrefixedInstruction::CBSetBR(0, RegisterU8::B);
    table[0xC1] = PrefixedInstruction::CBSetBR(0, RegisterU8::C);
    table[0xC2] = PrefixedInstruction::CBSetBR(0, RegisterU8::D);
    table[0xC3] = PrefixedInstruction::CBSetBR(0, RegisterU8::E);
    table[0xC4] = PrefixedInstruction::CBSetBR(0, RegisterU8::H);
    table[0xC5] = PrefixedInstruction::CBSetBR(0, RegisterU8::L);
    table[0xC6] = PrefixedInstruction::CBSetBHL(0);
    table[0xC7] = PrefixedInstruction::CBSetBR(0, RegisterU8::A);
    table[0xC8] = PrefixedInstruction::CBSetBR(1, RegisterU8::B);
    table[0xC9] = PrefixedInstruction::CBSetBR(1, RegisterU8::C);
    table[0xCA] = PrefixedInstruction::CBSetBR(1, RegisterU8::D);
    table[0xCB] = PrefixedInstruction::CBSetBR(1, RegisterU8::E);
    table[0xCC] = PrefixedInstruction::CBSetBR(1, RegisterU8::H);
    table[0xCD] = PrefixedInstruction::CBSetBR(1, RegisterU8::L);
    table[0xCE] = PrefixedInstruction::CBSetBHL(1);
    table[0xCF] = PrefixedInstruction::CBSetBR(1, RegisterU8::A);
    table[0xD0] = PrefixedInstruction::CBSetBR(2, RegisterU8::B);
    table[0xD1] = PrefixedInstruction::CBSetBR(2, RegisterU8::C);
    table[0xD2] = PrefixedInstruction::CBSetBR(2, RegisterU8::D);
    table[0xD3] = PrefixedInstruction::CBSetBR(2, RegisterU8::E);
    table[0xD4] = PrefixedInstruction::CBSetBR(2, RegisterU8::H);
    table[0xD5] = PrefixedInstruction::CBSetBR(2, RegisterU8::L);
    table[0xD6] = PrefixedInstruction::CBSetBHL(2);
    table[0xD7] = PrefixedInstruction::CBSetBR(2, RegisterU8::A);
    table[0xD8] = PrefixedInstruction::CBSetBR(3, RegisterU8::B);
    table[0xD9] = PrefixedInstruction::CBSetBR(3, RegisterU8::C);
    table[0xDA] = PrefixedInstruction::CBSetBR(3, RegisterU8::D);
    table[0xDB] = PrefixedInstruction::CBSetBR(3, RegisterU8::E);
    table[0xDC] = PrefixedInstruction::CBSetBR(3, RegisterU8::H);
    table[0xDD] = PrefixedInstruction::CBSetBR(3, RegisterU8::L);
    table[0xDE] = PrefixedInstruction::CBSetBHL(3);
    table[0xDF] = PrefixedInstruction::CBSetBR(3, RegisterU8::A);
    table[0xE0] = PrefixedInstruction::CBSetBR(4, RegisterU8::B);
    table[0xE1] = PrefixedInstruction::CBSetBR(4, RegisterU8::C);
    table[0xE2] = PrefixedInstruction::CBSetBR(4, RegisterU8::D);
    table[0xE3] = PrefixedInstruction::CBSetBR(4, RegisterU8::E);
    table[0xE4] = PrefixedInstruction::CBSetBR(4, RegisterU8::H);
    table[0xE5] = PrefixedInstruction::CBSetBR(4, RegisterU8::L);
    table[0xE6] = PrefixedInstruction::CBSetBHL(4);
    table[0xE7] = PrefixedInstruction::CBSetBR(4, RegisterU8::A);
    table[0xE8] = PrefixedInstruction::CBSetBR(5, RegisterU8::B);
    table[0xE9] = PrefixedInstruction::CBSetBR(5, RegisterU8::C);
    table[0xEA] = PrefixedInstruction::CBSetBR(5, RegisterU8::D);
    table[0xEB] = PrefixedInstruction::CBSetBR(5, RegisterU8::E);
    table[0xEC] = PrefixedInstruction::CBSetBR(5, RegisterU8::H);
    table[0xED] = PrefixedInstruction::CBSetBR(5, RegisterU8::L);
    table[0xEE] = PrefixedInstruction::CBSetBHL(5);
    table[0xEF] = PrefixedInstruction::CBSetBR(5, RegisterU8::A);
    table[0xF0] = PrefixedInstruction::CBSetBR(6, RegisterU8::B);
    table[0xF1] = PrefixedInstruction::CBSetBR(6, RegisterU8::C);
    table[0xF2] = PrefixedInstruction::CBSetBR(6, RegisterU8::D);
    table[0xF3] = PrefixedInstruction::CBSetBR(6, RegisterU8::E);
    table[0xF4] = PrefixedInstruction::CBSetBR(6, RegisterU8::H);
    table[0xF5] = PrefixedInstruction::CBSetBR(6, RegisterU8::L);
    table[0xF6] = PrefixedInstruction::CBSetBHL(6);
    table[0xF7] = PrefixedInstruction::CBSetBR(6, RegisterU8::A);
    table[0xF8] = PrefixedInstruction::CBSetBR(7, RegisterU8::B);
    table[0xF9] = PrefixedInstruction::CBSetBR(7, RegisterU8::C);
    table[0xFA] = PrefixedInstruction::CBSetBR(7, RegisterU8::D);
    table[0xFB] = PrefixedInstruction::CBSetBR(7, RegisterU8::E);
    table[0xFC] = PrefixedInstruction::CBSetBR(7, RegisterU8::H);
    table[0xFD] = PrefixedInstruction::CBSetBR(7, RegisterU8::L);
    table[0xFE] = PrefixedInstruction::CBSetBHL(7);
    table[0xFF] = PrefixedInstruction::CBSetBR(7, RegisterU8::A);

    Self { prefixed_optable: table }
  }
}
