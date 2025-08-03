use cpu::instructions::CycleTable;
use cpu::instructions::Instruction;
use cpu::instructions::Optable;
use cpu::instructions::RstAddress;
use cpu::registers::Flag;
use cpu::registers::RegisterPair;
use cpu::registers::Registers;
use cpu::registers::RegisterU8;
use cpu::registers::Target;
use memory::memory::Memory;

pub const MASTER_CLOCK_SPEED: i32 = 4194304; // Hz

pub struct Cpu {
  pub registers: Registers,
  pub optable: Optable,
  pub cycles: u64,
  pub cycles_table: CycleTable,
}

impl Cpu {
  pub fn new() -> Self {
    Self {
      registers: Registers::new(),
      optable: Optable::new(),
      cycles_table: CycleTable::new(),
      cycles: 0
    }
  }

  pub fn run_instruction(&mut self, memory: &mut Memory) {
    let opcode = memory.read(self.registers.pc);

    match &self.optable.optable[opcode as usize] {
      Instruction::AdcR(r) => Self::adc_r(self, memory, *r),
      Instruction::AddHLRR(rr) => Self::add_hl_rr(self, memory, *rr),
      Instruction::AddN => Self::add_n(self, memory),
      Instruction::AddR(r) => Self::add_r(self, memory, *r),
      Instruction::AndN => Self::and_n(self, memory),
      Instruction::AndR(r) => Self::and_r(self, memory, *r),
      Instruction::Call => Self::call(self, memory),
      Instruction::Ccf => Self::ccf(self, memory),
      Instruction::Cpl => Self::cpl(self, memory),
      Instruction::CpN => Self::cp_n(self, memory),
      Instruction::CpR(r) => Self::cp_r(self, memory, *r),
      Instruction::Dec(r) => Self::dec_n(self, memory, *r),
      Instruction::Di => Self::di(self, memory),
      Instruction::Halt => Self::halt(self, memory),
      Instruction::IncR(r) => Self::inc_r(self, memory, *r),
      Instruction::IncNn(r1) => Self::inc_nn(self, memory, *r1),
      Instruction::Invalid => Self::invalid_instruction(self, memory),
      Instruction::JpCCNN(cc, set) => Self::jp_cc_nn(self, memory, *cc, *set),
      Instruction::JpNN => Self::jp_nn(self, memory),
      Instruction::JrE => Self::jr_e(self, memory),
      Instruction::JrCCE(cc, set) => Self::jr_cc_e(self, memory, *cc, *set),
      Instruction::LdAHLD => Self::ld_a_hld(self, memory),
      Instruction::LdAHLI => Self::ld_a_hli(self, memory),
      Instruction::LdhAN => Self::ldh_a_n(self, memory),
      Instruction::LdHLDA => Self::ld_hld_a(self, memory),
      Instruction::LdHLN => Self::ld_hl_n(self, memory),
      Instruction::LdhNR(r) => Self::ldh_n_r(self, memory, *r),
      Instruction::LdMemHLFromR(r) => Self::ld_mem_hl_from_r(self, memory, *r),
      Instruction::LdNnA => Self::ld_nn_a(self, memory),
      Instruction::LdNNn(n) => Self::ld_n_nn(self, memory, *n),
      Instruction::LdNnN(nn) => Self::ld_nn_n(self, memory, *nn),
      Instruction::LdNnSP => Self::ld_nn_sp(self, memory),
      Instruction::LdR1R2(r1, r2) => Self::ld_r1_r2(self, memory, *r1, *r2),
      Instruction::LdRFromMemHL(r) => Self::ld_r_from_mem_hl(self, memory, *r),
      Instruction::LdRN(r) => Self::ld_r_n(self, memory, *r),
      Instruction::LdRRA(r) => Self::ld_rr_a(self, memory, *r),
      Instruction::Nop => Self::nop(self, memory),
      Instruction::OrN => Self::or_n(self, memory),
      Instruction::OrR(r) => Self::or_r(self, memory, *r),
      Instruction::PopRR(rr) => Self::pop_rr(self, memory, *rr),
      Instruction::PushRR(r) => Self::push_rr(self, memory, *r),
      Instruction::Ret => Self::ret(self, memory),
      Instruction::Rla => Self::rla(self, memory),
      Instruction::Rlca => Self::rlca(self, memory),
      Instruction::Rra => Self::rra(self, memory),
      Instruction::Rst(jump_address) => Self::rst_n(self, memory, *jump_address),
      Instruction::SbcR(r) => Self::sbc_r(self, memory, *r),
      Instruction::Scf => Self::scf(self, memory),
      Instruction::Stop => Self::stop(self, memory),
      Instruction::SubN => Self::sub_n(self, memory),
      Instruction::SubR(r) => Self::sub_r(self, memory, *r),
      Instruction::Unimplemented => Self::unimplemented_instruction(self, memory),
      Instruction::Xor(r) => Self::xor_r(self, memory, *r),
    }

    self.cycles += self.cycles_table.cycle_table[opcode as usize];
  }

  pub fn get_half_carry(&mut self, prev: u8, result: u8) -> bool {
    if ((prev.wrapping_shr(4)) & 1) != ((result.wrapping_shr(4)) & 1) {
      true
    } else {
      false
    }
  }

  pub fn get_carry(&mut self, prev: u8, result: u8) -> bool {
    if ((prev.wrapping_shr(8)) & 1) != ((result.wrapping_shr(8)) & 1) {
      true
    } else {
      false
    }
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

  fn ld_r_n(&mut self, memory: &mut Memory, r: RegisterU8) {
    let pc = self.registers.pc;
    self.registers[r] = memory.read(pc + 1);

    self.registers.pc += 2;
  }

  fn ld_rr_a(&mut self, memory: &mut Memory, r: RegisterPair) {
    memory.write(self.registers.get_pair(r), self.registers.a);

    self.registers.pc += 1;
  }

  fn ld_nn_a(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    
    let low = memory.read(pc + 1);
    let high = memory.read(pc + 2);

    memory.write(((high as u16) << 8) | (low as u16), self.registers.a);

    self.registers.pc += 3;
  }

  fn ldh_n_r(&mut self, memory: &mut Memory, r: RegisterU8) {
    let pc = self.registers.pc;
    let destination_address: u16 = ((0xFF as u16) << 8) | (memory.read(pc + 1) as u16);

    memory.write(destination_address, self.registers[r]);

    self.registers.pc += 2;
  }

  fn ld_r_from_mem_hl(&mut self, memory: &mut Memory, r: RegisterU8) {
    let value = memory.read(self.registers.get_pair(RegisterPair::HL));

    self.registers[r] = value;

    self.registers.pc += 1;
  }

  fn ld_mem_hl_from_r(&mut self, memory: &mut Memory, r: RegisterU8) {
    memory.write(self.registers.get_pair(RegisterPair::HL), self.registers[r]);

    self.registers.pc += 1;
  }

  fn ld_hld_a(&mut self, memory: &mut Memory) {
    let hl = self.registers.get_pair(RegisterPair::HL);
    memory.write(hl, self.registers.a);
    self.registers.set_pair(RegisterPair::HL, hl.wrapping_sub(1));

    self.registers.pc += 1;
  }

  fn ld_a_hld(&mut self, memory: &mut Memory) {
    let hl = self.registers.get_pair(RegisterPair::HL);
    self.registers.a = memory.read(hl);
    self.registers.set_pair(RegisterPair::HL, hl.wrapping_sub(1));

    self.registers.pc += 1;
  }

  fn ld_a_hli(&mut self, memory: &mut Memory) {
    let hl = self.registers.get_pair(RegisterPair::HL);
    self.registers.a = memory.read(hl);
    self.registers.set_pair(RegisterPair::HL, hl.wrapping_add(1));

    self.registers.pc += 1;
  }

  fn ld_n_nn(&mut self, memory: &mut Memory, n: Target) {
    let pc = self.registers.pc;
    let low: u8 = memory.read(pc + 1);
    let high: u8 = memory.read(pc + 2);

    if let Target::SingleU16(_register) = n {
      self.registers.sp = ((high as u16) << 8) | (low as u16);
    }

    if let Target::Pair(register_pair) = n {
      self.registers.set_pair(register_pair, ((high as u16) << 8) | low as u16);
    }

    self.registers.pc += 3;
  }

  fn ld_nn_n(&mut self, memory: &mut Memory, nn: RegisterU8) {
    let pc = self.registers.pc;

    self.registers[nn] = memory.read(pc + 1);
    self.registers.pc += 2;
  }

  fn ld_nn_sp(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let low = memory.read(pc + 1);
    let high = memory.read(pc + 2);

    let nn = (high as u16) << 8 | low as u16;
    let sp = self.registers.sp.to_le_bytes();

    memory.write(nn, sp[0]);
    memory.write(nn + 1, sp[1]);

    self.registers.pc += 3;
  }

  fn ldh_a_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let low = memory.read(pc + 1);

    let address = ((0xFF as u16) << 8) | low as u16;

    self.registers.a = memory.read(address);

    self.registers.pc += 2;
  }

  fn ld_hl_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let n = memory.read(pc + 1);

    memory.write(self.registers.get_pair(RegisterPair::HL), n);

    self.registers.pc += 2;
  }

  fn call(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let sp = self.registers.sp;

    let low = memory.read(pc + 1);
    let high = memory.read(pc + 2);

    let split_u8_values = (self.registers.pc + 3).to_le_bytes();

    self.registers.sp -= 1;
    memory.write(sp - 1, split_u8_values[1]);
    self.registers.sp -= 1;
    memory.write(sp - 2, split_u8_values[0]);

    self.registers.pc = ((high as u16) << 8) | (low as u16);
  }

  fn jp_nn(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;

    let low: u8 = memory.read(pc + 1);
    let high: u8 = memory.read(pc + 2);

    let address = ((high as u16) << 8) | (low as u16);
    self.registers.pc = address;
  }

  fn jp_cc_nn(&mut self, memory: &mut Memory, cc: Flag, set: bool) {
    let pc = self.registers.pc;
    let low = memory.read(pc + 1);
    let high = memory.read(pc + 2);
    
    let nn = ((high as u16) << 8) | (low as u16);

    match cc {
      Flag::Z => {
        if set && self.registers.get_z_flag() {
          self.registers.pc = nn;
        } else if !set && !self.registers.get_z_flag() {
          self.registers.pc = nn;
        }
      },
      Flag::C => {
        if set && self.registers.get_c_flag() {
          self.registers.pc = nn;
        } else if !set && !self.registers.get_c_flag() {
          self.registers.pc = nn;
        }
      },
      Flag::N => panic!("This flag must not be used here"),
      Flag::H => panic!("This flag must not be used here"),
    }
  }

  fn jr_e(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let e = memory.read(pc + 1) as i8;

    let destination_address = (pc + 2).wrapping_add_signed(e.into());

    self.registers.pc = destination_address;
  }

  fn jr_cc_e(&mut self, memory: &mut Memory, cc: Flag, set: bool) {
    let pc = self.registers.pc;
    let e = memory.read(pc + 1) as i8;
  
    match cc {
      Flag::Z => {
        if set && self.registers.get_z_flag() {
          self.registers.pc = (pc + 2).wrapping_add_signed(e.into());
        } else if !set && !self.registers.get_z_flag() {
          self.registers.pc = (pc + 2).wrapping_add_signed(e.into());
        } else {
          self.registers.pc += 2;
        }
      },
      Flag::C => {
        if set && self.registers.get_c_flag() {
          self.registers.pc = (pc + 2).wrapping_add_signed(e.into());
        } else if !set && !self.registers.get_c_flag() {
          self.registers.pc = (pc + 2).wrapping_add_signed(e.into());
        } else {
          self.registers.pc += 2;
        }
      },
      Flag::N => panic!("This flag must not be used here"),
      Flag::H => panic!("This flag must not be used here"),
    }
  }

  fn cpl(&mut self, _memory: &mut Memory) {
    self.registers.a = !self.registers.a;
  
    self.registers.set_n_flag();
    self.registers.set_h_flag();
  
    self.registers.pc += 1;
  }

  fn scf(&mut self, _memory: &mut Memory) {
    self.registers.unset_n_flag();
    self.registers.unset_h_flag();
    self.registers.set_c_flag();

    self.registers.pc += 1;
  }

  fn rst_n(&mut self, memory: &mut Memory, jump_address: RstAddress) {
    let sp = self.registers.sp;
    let split_u8_values = self.registers.pc.to_le_bytes();

    self.registers.sp -= 1;
    memory.write(sp - 1, split_u8_values[1]);

    self.registers.sp -= 1;
    memory.write(sp - 2, split_u8_values[0]);
    
    self.registers.pc = jump_address as u16;
  }

  fn inc_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let prev: u8 = self.registers[r];
    let result = self.registers[r].wrapping_add(1);
  
    self.registers[r] = result;

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.unset_n_flag();

    let half_carry = Self::get_half_carry(self, prev, result);

    if half_carry {
      self.registers.set_h_flag();
    } else {
      self.registers.unset_h_flag();
    }
  
    self.registers.pc += 1;
  }

  fn inc_nn(&mut self, _memory: &mut Memory, r1: Target) {
    if let Target::SingleU16(_register) = r1 {
      self.registers.sp += 1;
    }

    if let Target::Pair(register) = r1 {
      self.registers.set_pair(register, self.registers.get_pair(register).wrapping_add(1));
    }

    self.registers.pc += 1;
  }

  fn dec_n(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let prev = self.registers[r];
    let result = self.registers[r].wrapping_sub(1);
  
    self.registers[r] = result;

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.set_n_flag();

    let half_carry = Self::get_half_carry(self, prev, result);

    if half_carry {
      self.registers.set_h_flag();
    } else {
      self.registers.unset_h_flag();
    }

    self.registers.pc += 1;
  }

  fn add_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let prev = self.registers.a;
    let result = self.registers.a.wrapping_add(self.registers[r]);

    self.registers.a = result;

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.unset_n_flag();

    let half_carry = Self::get_half_carry(self, prev, result);

    if half_carry {
      self.registers.set_h_flag();
    } else {
      self.registers.unset_h_flag();
    }

    let carry = Self::get_carry(self, prev, result);

    if carry {
      self.registers.set_c_flag();
    } else {
      self.registers.unset_c_flag();
    }

    self.registers.pc += 1;
  }

  fn add_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let n = memory.read(pc + 1);

    let prev = self.registers.a;
    let result = self.registers.a.wrapping_add(n);

    self.registers.a = result;

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.unset_n_flag();

    let half_carry = Self::get_half_carry(self, prev, result);

    if half_carry {
      self.registers.set_h_flag();
    } else {
      self.registers.unset_h_flag();
    }

    let carry = Self::get_carry(self, prev, result);

    if carry {
      self.registers.set_c_flag();
    } else {
      self.registers.unset_c_flag();
    }

    self.registers.pc += 2;
  }

  fn add_hl_rr(&mut self, _memory: &mut Memory, rr: Target) {
    let result;
    let carry_per_bit;

    if let Target::Pair(register_pair) = rr {
      result = self.registers.get_pair(RegisterPair::HL).wrapping_add(self.registers.get_pair(register_pair));
    } else {
      result = self.registers.get_pair(RegisterPair::HL).wrapping_add(self.registers.sp);
    }

    if let Target::Pair(register_pair) = rr {
      carry_per_bit = self.registers.get_pair(RegisterPair::HL).wrapping_add(self.registers.get_pair(register_pair));
    } else {
      carry_per_bit = self.registers.get_pair(RegisterPair::HL).wrapping_add(self.registers.sp);
    }

    self.registers.set_pair(RegisterPair::HL, result);

    self.registers.unset_n_flag();

    if ((carry_per_bit >> 11) & 1) == 1 {
      self.registers.set_h_flag();
    } else {
      self.registers.unset_h_flag();
    }

    if ((carry_per_bit >> 15) & 1) == 1 {
      self.registers.set_c_flag();
    } else {
      self.registers.unset_c_flag();
    }

    self.registers.pc += 1;
  }

  fn adc_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let c_flag = self.registers.get_c_flag();
    let prev = self.registers.a;
    let result;

    if c_flag {
      result = self.registers.a.wrapping_add(self.registers[r].wrapping_add(1));
    } else {
      result = self.registers.a.wrapping_add(self.registers[r]);
    }

    self.registers.a = result;

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.unset_n_flag();

    let half_carry = Self::get_half_carry(self, prev, result);

    if half_carry {
      self.registers.set_h_flag();
    } else {
      self.registers.unset_h_flag();
    }

    let carry = Self::get_carry(self, prev, result);

    if carry {
      self.registers.set_c_flag();
    } else {
      self.registers.unset_c_flag();
    }

    self.registers.pc += 1;
  }

  fn sub_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let prev = self.registers.a;
    let result = self.registers.a.wrapping_sub(self.registers[r]);

    self.registers.a = result;

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.set_n_flag();

    let half_carry = Self::get_half_carry(self, prev, result);

    if half_carry {
      self.registers.set_h_flag();
    } else {
      self.registers.unset_h_flag();
    }

    let carry = Self::get_carry(self, prev, result);

    if carry {
      self.registers.set_c_flag();
    } else {
      self.registers.unset_c_flag();
    }
    
    self.registers.pc += 1;
  }

  fn sbc_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let c_flag = self.registers.get_c_flag();
    let prev = self.registers.a;
    let result;

    if c_flag {
      result = self.registers.a.wrapping_sub(self.registers[r].wrapping_sub(1));
    } else {
      result = self.registers.a.wrapping_sub(self.registers[r]);
    }

    self.registers.a = result;

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.set_n_flag();

    let half_carry = Self::get_half_carry(self, prev, result);

    if half_carry {
      self.registers.set_h_flag();
    } else {
      self.registers.unset_h_flag();
    }

    let carry = Self::get_carry(self, prev, result);

    if carry {
      self.registers.set_c_flag();
    } else {
      self.registers.unset_c_flag();
    }

    self.registers.pc += 1;
  }

  fn xor_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let result = self.registers[r] ^ self.registers.a;
    self.registers.a = result;

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.unset_n_flag();
    self.registers.unset_h_flag();
    self.registers.unset_c_flag();

    self.registers.pc += 1;
  }

  fn and_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let result = self.registers.a & memory.read(pc + 1);

    self.registers.a = result;

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.unset_n_flag();
    self.registers.set_h_flag();
    self.registers.unset_c_flag();

    self.registers.pc += 2;
  }

  fn and_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let result = self.registers.a & self.registers[r];
    self.registers.a = result;

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.unset_n_flag();
    self.registers.set_h_flag();
    self.registers.unset_c_flag();

    self.registers.pc += 1;
  }

  fn or_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let result = self.registers.a | memory.read(pc + 1);
  
    self.registers.a = result;

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.unset_n_flag();
    self.registers.unset_h_flag();
    self.registers.unset_c_flag();

    self.registers.pc += 2;
  }

  fn or_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let result = self.registers.a | self.registers[r];

    self.registers.a = result;

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.unset_n_flag();
    self.registers.unset_h_flag();
    self.registers.unset_c_flag();
    
    self.registers.pc += 1;
  }

  fn di(&mut self, _memory: &mut Memory) {
    // TODO
    self.registers.pc += 1;
  }

  fn ret(&mut self, memory: &mut Memory) {
    let sp = self.registers.sp;

    let low = memory.read(sp);
    self.registers.sp += 1;
    let high = memory.read(sp + 1);
    self.registers.sp += 1;

    self.registers.pc = ((high as u16) << 8) | (low as u16);
  }

  fn sub_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;

    let prev = self.registers.a;
    let result = self.registers.a.wrapping_sub(memory.read(pc + 1));
  
    self.registers.a = result;

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.set_n_flag();

    let half_carry = Self::get_half_carry(self, prev, result);

    if half_carry {
      self.registers.set_h_flag();
    } else {
      self.registers.unset_h_flag();
    }

    let carry = Self::get_carry(self, prev, result);

    if carry {
      self.registers.set_c_flag();
    } else {
      self.registers.unset_c_flag();
    }
  
    self.registers.pc += 1;
  }

  fn stop(&mut self, _memory: &mut Memory) {
    // TODO
    self.registers.pc += 2;
  }

  fn push_rr(&mut self, memory: &mut Memory, r: RegisterPair) {
    let sp = self.registers.sp;
    let value = self.registers.get_pair(r).to_le_bytes();

    self.registers.sp -= 1;
    memory.write(sp - 1, value[1]);
    self.registers.sp -= 1;
    memory.write(sp - 2, value[0]);

    self.registers.pc += 1;
  }

  fn ccf(&mut self, _memory: &mut Memory) {
    self.registers.unset_n_flag();
    self.registers.unset_h_flag();

    let c_flag = self.registers.get_c_flag();

    if c_flag {
      self.registers.unset_c_flag();
    } else {
      self.registers.set_c_flag();
    }

    self.registers.pc += 1;
  }

  fn rra(&mut self, _memory: &mut Memory) {
    let b0 = self.registers.a & (1 << 0) != 0;
    let c_flag = self.registers.get_c_flag();

    self.registers.a = self.registers.a.rotate_right(1);
    
    if c_flag {
      self.registers.a |= 0b1000_0000;
    } else {
      self.registers.a &= 0b1000_0000;
    }

    self.registers.unset_z_flag();
    self.registers.unset_n_flag();
    self.registers.unset_h_flag();

    if b0 {
      self.registers.set_c_flag();
    } else {
      self.registers.unset_c_flag();
    }

    self.registers.pc += 1;
  }

  fn rla(&mut self, _memory: &mut Memory) {
    let b7 = self.registers.a & (1 << 7) != 0;
    let c_flag = self.registers.get_c_flag();

    self.registers.a = self.registers.a.rotate_left(1);
    
    if c_flag {
      self.registers.a |= 0b0000_0001;
    } else {
      self.registers.a &= 0b0000_0001;
    }

    self.registers.unset_z_flag();
    self.registers.unset_n_flag();
    self.registers.unset_h_flag();

    if b7 {
      self.registers.set_c_flag();
    } else {
      self.registers.unset_c_flag();
    }

    self.registers.pc += 1;
  }

  fn rlca(&mut self, _memory: &mut Memory) {
    let b7 = self.registers.a & (1 << 7) != 0;

    self.registers.a = self.registers.a.rotate_left(1);
    
    if b7 {
      self.registers.a |= 0b0000_0001;
    } else {
      self.registers.a &= 0b0000_0001;
    }

    self.registers.unset_z_flag();
    self.registers.unset_n_flag();
    self.registers.unset_h_flag();

    if b7 {
      self.registers.set_c_flag();
    } else {
      self.registers.unset_c_flag();
    }

    self.registers.pc += 1;
  }

  fn cp_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let prev = self.registers.a;
    let result = self.registers.a.wrapping_sub(self.registers[r]);

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.set_n_flag();

    let half_carry = Self::get_half_carry(self, prev, result);
    
    if half_carry {
      self.registers.set_h_flag();
    } else {
      self.registers.unset_h_flag();
    }

    let carry = Self::get_carry(self, prev, result);

    if carry {
      self.registers.set_c_flag();
    } else {
      self.registers.unset_c_flag();
    }

    self.registers.pc += 1;
  }

  fn cp_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let n = memory.read(pc + 1);
    let prev = self.registers.a;
    let result = self.registers.a.wrapping_sub(n);

    if result == 0 {
      self.registers.set_z_flag();
    } else {
      self.registers.unset_z_flag();
    }

    self.registers.set_n_flag();

    let half_carry = Self::get_half_carry(self, prev, result);

    if half_carry {
      self.registers.set_h_flag();
    } else {
      self.registers.unset_h_flag();
    }

    let carry = Self::get_carry(self, prev, result);

    if carry {
      self.registers.set_c_flag();
    } else {
      self.registers.unset_c_flag();
    }

    self.registers.pc += 2;
  }

  fn halt(&mut self, _memory: &mut Memory) {
    // TODO

    self.registers.pc += 1;
  }

  fn pop_rr(&mut self, memory: &mut Memory, rr: RegisterPair) {
    let sp = self.registers.sp;

    let low = memory.read(sp);
    self.registers.sp += 1;

    let high = memory.read(sp + 1);
    self.registers.sp += 1;

    self.registers.set_pair(rr, ((high as u16) << 8) | (low as u16));

    self.registers.pc += 1;
  }
}
