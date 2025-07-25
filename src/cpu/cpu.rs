use cpu::instructions::Instruction;
use cpu::instructions::Optable;
use cpu::instructions::RstAddress;
use cpu::registers::Registers;
use cpu::registers::RegisterU8;
use cpu::registers::Target;
use memory::memory::Memory;

pub struct Cpu {
  pub registers: Registers,
  pub optable: Optable
}

impl Cpu {
  pub fn new() -> Self {
    Self {
      registers: Registers::new(),
      optable: Optable::new(),
    }
  }

  pub fn run_instruction(&mut self, memory: &mut Memory) {
    let opcode = memory.read(self.registers.pc);

    match &self.optable.optable[opcode as usize] {
      Instruction::Nop => Self::nop(self, memory),
      Instruction::Unimplemented => Self::unimplemented_instruction(self, memory),
      Instruction::LdR1R2(r1, r2) => Self::ld_r1_r2(self, memory, *r1, *r2),
      Instruction::JpNN => Self::jp_nn(self, memory),
      Instruction::Cpl => Self::cpl(self, memory),
      Instruction::Rst(jump_address) => Self::rst_n(self, memory, *jump_address),
      Instruction::Inc(r1) => Self::inc_n(self, memory, *r1),
      Instruction::Xor(r1) => Self::xor_n(self, memory, *r1),
      Instruction::LdNNn(n) => Self::ld_n_nn(self, memory, *n),
    }

    self.registers.pc += 1;
  }

  fn unimplemented_instruction(&mut self, memory: &mut Memory) {
    panic!("Instruction not yet implemented. Opcode: 0x{:02X}. PC: 0x{:02X}", memory.read(self.registers.pc), self.registers.pc);
  }

  fn nop(&mut self, _memory: &mut Memory) {
  }

  fn ld_r1_r2(&mut self, _memory: &mut Memory, r1: RegisterU8, r2: RegisterU8) {
    self.registers[r1] = self.registers[r2];
  }

  fn jp_nn(&mut self, memory: &mut Memory) {
    let low: u8 = memory.read(self.registers.pc);
    self.registers.pc += 1;
    let high: u8 = memory.read(self.registers.pc);

    self.registers.pc = ((high as u16) << 8) | (low as u16);
  }

  fn cpl(&mut self, _memory: &mut Memory) {
    self.registers.a = !self.registers.a;
  }

  fn rst_n(&mut self, memory: &mut Memory, jump_address: RstAddress) {
    let split_u8_values = self.registers.pc.to_le_bytes();

    self.registers.sp -= 0x1;
    memory.write(self.registers.sp, split_u8_values[0]);

    self.registers.sp -= 0x1;
    memory.write(self.registers.sp, split_u8_values[1]);
    
    self.registers.pc = jump_address as u16;
  }

  fn inc_n(&mut self, _memory: &mut Memory, r1: RegisterU8) {
    self.registers[r1] += 1;
  }

  fn xor_n(&mut self, _memory: &mut Memory, r1: RegisterU8) {
    self.registers[r1] = self.registers[r1] ^ self.registers[r1];
  }

  fn ld_n_nn(&mut self, memory: &mut Memory, n: Target) {
    let low: u8 = memory.read(self.registers.pc);
    self.registers.pc += 1;
    let high: u8 = memory.read(self.registers.pc);

    if let Target::Single(_register) = n {
      self.registers.sp = ((high as u16) >> 8) | (low as u16);
    }

    if let Target::Pair(register_pair) = n {
      self.registers.set_pair(register_pair, ((high as u16) >> 8) | low as u16);
    }
  }
}
