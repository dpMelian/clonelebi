use cpu::registers::RegisterU8;

pub enum PrefixedInstruction {
  CBRLCR(RegisterU8),
  CBRlR(RegisterU8),
  CBRRCR(RegisterU8),
  CBRRR(RegisterU8),
  CBSetBHL(usize),
  CBSRLR(RegisterU8),
  CBSlaR(RegisterU8),
  CBSraR(RegisterU8),
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
    table[0x08] = PrefixedInstruction::CBRRCR(RegisterU8::B);
    table[0x09] = PrefixedInstruction::CBRRCR(RegisterU8::C);
    table[0x0A] = PrefixedInstruction::CBRRCR(RegisterU8::D);
    table[0x0B] = PrefixedInstruction::CBRRCR(RegisterU8::E);
    table[0x0C] = PrefixedInstruction::CBRRCR(RegisterU8::H);
    table[0x0D] = PrefixedInstruction::CBRRCR(RegisterU8::L);
    table[0x0F] = PrefixedInstruction::CBRRCR(RegisterU8::A);
    table[0x10] = PrefixedInstruction::CBRlR(RegisterU8::B);
    table[0x11] = PrefixedInstruction::CBRlR(RegisterU8::C);
    table[0x12] = PrefixedInstruction::CBRlR(RegisterU8::D);
    table[0x13] = PrefixedInstruction::CBRlR(RegisterU8::E);
    table[0x14] = PrefixedInstruction::CBRlR(RegisterU8::H);
    table[0x15] = PrefixedInstruction::CBRlR(RegisterU8::L);
    table[0x17] = PrefixedInstruction::CBRlR(RegisterU8::A);
    table[0x18] = PrefixedInstruction::CBRRR(RegisterU8::B);
    table[0x19] = PrefixedInstruction::CBRRR(RegisterU8::C);
    table[0x1A] = PrefixedInstruction::CBRRR(RegisterU8::D);
    table[0x1B] = PrefixedInstruction::CBRRR(RegisterU8::E);
    table[0x1C] = PrefixedInstruction::CBRRR(RegisterU8::H);
    table[0x1D] = PrefixedInstruction::CBRRR(RegisterU8::L);
    table[0x1F] = PrefixedInstruction::CBRRR(RegisterU8::A);
    table[0x20] = PrefixedInstruction::CBSlaR(RegisterU8::B);
    table[0x21] = PrefixedInstruction::CBSlaR(RegisterU8::C);
    table[0x22] = PrefixedInstruction::CBSlaR(RegisterU8::D);
    table[0x23] = PrefixedInstruction::CBSlaR(RegisterU8::E);
    table[0x24] = PrefixedInstruction::CBSlaR(RegisterU8::H);
    table[0x25] = PrefixedInstruction::CBSlaR(RegisterU8::L);
    table[0x27] = PrefixedInstruction::CBSlaR(RegisterU8::A);
    table[0x28] = PrefixedInstruction::CBSraR(RegisterU8::B);
    table[0x29] = PrefixedInstruction::CBSraR(RegisterU8::C);
    table[0x2A] = PrefixedInstruction::CBSraR(RegisterU8::D);
    table[0x2B] = PrefixedInstruction::CBSraR(RegisterU8::E);
    table[0x2C] = PrefixedInstruction::CBSraR(RegisterU8::H);
    table[0x2D] = PrefixedInstruction::CBSraR(RegisterU8::L);
    table[0x2F] = PrefixedInstruction::CBSraR(RegisterU8::A);
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
