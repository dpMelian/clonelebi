use cpu::registers::Registers;
use cpu::registers::RegisterPairs;
use memory::memory::Memory;

pub struct Cpu {
  pub registers: Registers,
}

impl Cpu {
  pub fn run_instruction(&mut self, memory: &mut Memory) {
    let opcode = memory.read(self.registers.pc);
    self.registers.pc += 1;

    match opcode {
      0x00 => self.nop(),
      0x21 => self.ld_hl_nn(memory),
      0x7F => self.ld_a_a(),
      0xAF => self.xor_a_a(),
      0xC3 => self.jp_nn(memory),
      0xDF => self.rst_18(),
      0xFF => self.rst_38(),
      _ => panic!("Instruction not yet implemented. Opcode: {:#X}", opcode)
    }
  }

  pub fn ld_a_a(&mut self) {
    self.registers.a = self.registers.a;
  }

  pub fn nop(&self) {
  }

  pub fn jp_nn(&mut self, memory: &mut Memory) {
    let low: u8 = memory.read(self.registers.pc);
    self.registers.pc += 1;
    let high: u8 = memory.read(self.registers.pc);

    self.registers.pc = ((high as u16) << 8) | (low as u16);
  }

  pub fn xor_a_a(&mut self) {
    self.registers.a = self.registers.a ^ self.registers.a;
  }

  pub fn ld_hl_nn(&mut self, memory: &mut Memory) {
    let low: u8 = memory.read(self.registers.pc);
    self.registers.pc += 1;
    let high: u8 = memory.read(self.registers.pc);

    self.registers.set_pair(RegisterPairs::HL, ((high as u16) >> 8) | low as u16);
  }

  pub fn rst_18(&mut self) {
    // todo!("push return address to stack");
    self.registers.pc = 0x0018;
  }

  pub fn rst_38(&mut self) {
    // todo!("push return address to stack");
    self.registers.pc = 0x0038;
  }
}
