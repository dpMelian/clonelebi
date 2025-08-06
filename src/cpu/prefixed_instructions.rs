use cpu::registers::RegisterU8;

pub enum PrefixedInstruction {
  CBRLCR(RegisterU8),
  CBRRR(RegisterU8),
  CBSetBHL(usize),
  CBSRLR(RegisterU8),
  Unimplemented,
}

pub struct PrefixedOptable {
  pub prefixed_optable: [PrefixedInstruction; 256]
}

impl PrefixedOptable {
  pub fn new() -> Self {
    let mut table: [PrefixedInstruction; 256] = [const { PrefixedInstruction::Unimplemented }; 256];

    table[0x00] = PrefixedInstruction::CBRLCR(RegisterU8::B);
    table[0x01] = PrefixedInstruction::CBRLCR(RegisterU8::C);
    table[0x02] = PrefixedInstruction::CBRLCR(RegisterU8::D);
    table[0x03] = PrefixedInstruction::CBRLCR(RegisterU8::E);
    table[0x04] = PrefixedInstruction::CBRLCR(RegisterU8::H);
    table[0x05] = PrefixedInstruction::CBRLCR(RegisterU8::L);
    table[0x07] = PrefixedInstruction::CBRLCR(RegisterU8::A);
    table[0x18] = PrefixedInstruction::CBRRR(RegisterU8::B);
    table[0x19] = PrefixedInstruction::CBRRR(RegisterU8::C);
    table[0x1A] = PrefixedInstruction::CBRRR(RegisterU8::D);
    table[0x1B] = PrefixedInstruction::CBRRR(RegisterU8::E);
    table[0x1C] = PrefixedInstruction::CBRRR(RegisterU8::H);
    table[0x1D] = PrefixedInstruction::CBRRR(RegisterU8::L);
    table[0x1F] = PrefixedInstruction::CBRRR(RegisterU8::A);
    table[0x38] = PrefixedInstruction::CBSRLR(RegisterU8::B);
    table[0x39] = PrefixedInstruction::CBSRLR(RegisterU8::C);
    table[0x3A] = PrefixedInstruction::CBSRLR(RegisterU8::D);
    table[0x3B] = PrefixedInstruction::CBSRLR(RegisterU8::E);
    table[0x3C] = PrefixedInstruction::CBSRLR(RegisterU8::H);
    table[0x3D] = PrefixedInstruction::CBSRLR(RegisterU8::L);
    table[0x3F] = PrefixedInstruction::CBSRLR(RegisterU8::A);
    table[0xC6] = PrefixedInstruction::CBSetBHL(0);
    table[0xCE] = PrefixedInstruction::CBSetBHL(1);
    table[0xD6] = PrefixedInstruction::CBSetBHL(2);
    table[0xDE] = PrefixedInstruction::CBSetBHL(3);
    table[0xE6] = PrefixedInstruction::CBSetBHL(4);
    table[0xEE] = PrefixedInstruction::CBSetBHL(5);
    table[0xF6] = PrefixedInstruction::CBSetBHL(6);
    table[0xFE] = PrefixedInstruction::CBSetBHL(7);

    Self { prefixed_optable: table }
  }
}
