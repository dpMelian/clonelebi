use cpu::registers::RegisterU8;

pub enum PrefixedInstruction {
  CBRLCR(RegisterU8),
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

    Self { prefixed_optable: table }
  }
}
