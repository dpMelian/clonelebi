use cpu::instructions::CycleTable;
use cpu::instructions::Instruction;
use cpu::instructions::Optable;
use cpu::instructions::RstAddress;
use cpu::registers::RegisterPair;
use cpu::registers::Registers;
use cpu::registers::RegisterU8;
use cpu::registers::Target;
use memory::memory::Memory;

pub const MASTER_CLOCK_SPEED: i32 = 4194304; // Hz

pub struct Cpu {
  pub registers: Registers,
  pub optable: Optable,
  pub cycles: u64
}

impl Cpu {
  pub fn new() -> Self {
    Self {
      registers: Registers::new(),
      optable: Optable::new(),
      cycles: 0
    }
  }

  pub fn run_instruction(&mut self, memory: &mut Memory) {
    let opcode = memory.read(self.registers.pc);
    let cycles_table = CycleTable::new();

    match &self.optable.optable[opcode as usize] {
      Instruction::Cpl => Self::cpl(self, memory),
      Instruction::Dec(r1) => Self::dec_n(self, memory, *r1),
      Instruction::Di => Self::di(self, memory),
      Instruction::Inc(r1) => Self::inc_n(self, memory, *r1),
      Instruction::IncNn(r1) => Self::inc_nn(self, memory, *r1),
      Instruction::Invalid => Self::invalid_instruction(self, memory),
      Instruction::JpNN => Self::jp_nn(self, memory),
      Instruction::LdMemHLFromR(r) => Self::ld_mem_hl_from_r(self, memory, *r),
      Instruction::LdNNn(n) => Self::ld_n_nn(self, memory, *n),
      Instruction::LdNnN(nn) => Self::ld_nn_n(self, memory, *nn),
      Instruction::LdR1R2(r1, r2) => Self::ld_r1_r2(self, memory, *r1, *r2),
      Instruction::LdRFromMemHL(r) => Self::ld_r_from_mem_hl(self, memory, *r),
      Instruction::Nop => Self::nop(self, memory),
      Instruction::Rst(jump_address) => Self::rst_n(self, memory, *jump_address),
      Instruction::Unimplemented => Self::unimplemented_instruction(self, memory),
      Instruction::Xor(r1) => Self::xor_n(self, memory, *r1),
    }

    self.cycles += cycles_table.cycle_table[opcode as usize];
  }

  fn unimplemented_instruction(&mut self, memory: &mut Memory) {
    panic!("Instruction not yet implemented. Opcode: 0x{:02X}. PC: 0x{:02X}", memory.read(self.registers.pc), self.registers.pc);
  }

  fn invalid_instruction(&mut self, memory: &mut Memory) {
    panic!("Invalid instruction. Opcode: 0x{:02X}. PC: 0x{:02X}", memory.read(self.registers.pc), self.registers.pc);
  }

  fn nop(&mut self, _memory: &mut Memory) {
    self.registers.pc += 1;
  }

  fn ld_r1_r2(&mut self, _memory: &mut Memory, r1: RegisterU8, r2: RegisterU8) {
    self.registers[r1] = self.registers[r2];
    self.registers.pc += 1;
  }

  fn ld_r_from_mem_hl(&mut self, memory: &mut Memory, r: RegisterU8) {
    let value = memory.read(self.registers.get_pair(RegisterPair::HL));

    self.registers[r] = value;

    self.registers.pc += 2;
  }

  fn ld_mem_hl_from_r(&mut self, memory: &mut Memory, r: RegisterU8) {
    memory.write(self.registers.get_pair(RegisterPair::HL), self.registers[r]);

    self.registers.pc += 1;
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

    self.registers.pc += 2;
  }

  fn ld_nn_n(&mut self, memory: &mut Memory, nn: RegisterU8) {
    let pc = self.registers.pc;

    self.registers[nn] = memory.read(pc + 1);
    self.registers.pc += 2;
  }

  fn jp_nn(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;

    let low: u8 = memory.read(pc + 1);
    let high: u8 = memory.read(pc + 2);

    let address = ((high as u16) << 8) | (low as u16);
    self.registers.pc = address;
  }

  fn cpl(&mut self, _memory: &mut Memory) {
    self.registers.a = !self.registers.a;
    self.registers.pc += 1;
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
    self.registers.pc += 1;
  }

  fn inc_nn(&mut self, _memory: &mut Memory, r1: Target) {
    if let Target::Single(_register) = r1 {
      self.registers.sp = self.registers.sp + 1;
    }

    if let Target::Pair(register) = r1 {
      self.registers.set_pair(register, self.registers.get_pair(register) + 1);
    }

    self.registers.pc += 1;
  }

  fn dec_n(&mut self, _memory: &mut Memory, r1: RegisterU8) {
    if self.registers[r1] > 0 {
      self.registers[r1] -= 1;
    }

    self.registers.pc += 1;
  }

  fn xor_n(&mut self, _memory: &mut Memory, r1: RegisterU8) {
    self.registers[r1] = self.registers[r1] ^ self.registers[r1];
    self.registers.pc += 1;
  }

  fn di(&mut self, _memory: &mut Memory) {
    // TODO
    self.registers.pc += 1;
  }
}
