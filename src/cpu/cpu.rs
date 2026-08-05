use cpu::instructions::CycleTable;
use cpu::instructions::Instruction;
use cpu::instructions::Optable;
use cpu::instructions::RstAddress;
use cpu::interrupt_handler::handle_interrupt;
use cpu::prefixed_instructions::PrefixedInstruction;
use cpu::prefixed_instructions::PrefixedOptable;
use cpu::registers::Flag;
use cpu::registers::RegisterPair;
use cpu::registers::Registers;
use cpu::registers::RegisterU16;
use cpu::registers::RegisterU8;
use cpu::registers::Target;
use helpers::bit_operations;
use memory::memory::Memory;

pub const MASTER_CLOCK_SPEED: u64 = 4194304; // Hz
pub const DIV_INCREMENT_RATE: u64 = 16384; // Hz
pub const TAC_CLOCK_SELECT: [u64; 4] = [4096, 262144, 65536, 16384];

static mut DIV_INTERNAL_COUNTER: u64 = 0;
static mut TIMA_INTERNAL_COUNTER: u64 = 0;

pub struct Cpu {
  pub registers: Registers,
  pub optable: Optable,
  pub prefixed_optable: PrefixedOptable,
  pub cycles: u64,
  pub cycles_table: CycleTable,
  pub interrupt_master_enable_flag: bool,
}

impl Cpu {
  pub fn new() -> Self {
    Self {
      registers: Registers::new(),
      optable: Optable::new(),
      prefixed_optable: PrefixedOptable::new(),
      cycles_table: CycleTable::new(),
      cycles: 0,
      interrupt_master_enable_flag: false,
    }
  }

  pub fn run_instruction(&mut self, memory: &mut Memory) {
    let opcode = memory.read(self.registers.pc);

    if opcode == 0xCB {
      let cb_opcode = memory.read(self.registers.pc + 1);
      self.registers.pc += 1;
      match &self.prefixed_optable.prefixed_optable[cb_opcode as usize] {
        PrefixedInstruction::CBRLCR(r) => Self::cb_rlc_r(self, memory, *r),
        PrefixedInstruction::CBRlR(r) => Self::cb_rl_r(self, memory, *r),
        PrefixedInstruction::CBRRCR(r) => Self::cb_rrc_r(self, memory, *r),
        PrefixedInstruction::CBRRR(r) => Self::cb_rr_r(self, memory, *r),
        PrefixedInstruction::CBSetBHL(b) => Self::cb_set_b_hl(self, memory, *b),
        PrefixedInstruction::CBSRLR(r) => Self::cb_srl_r(self, memory, *r),
        PrefixedInstruction::CBSlaR(r) => Self::cb_sla_r(self, memory, *r),
        PrefixedInstruction::CBSraR(r) => Self::cb_sra_r(self, memory, *r),
        PrefixedInstruction::Unimplemented => Self::unimplemented_instruction(self, memory),
      }
    } else {
      match &self.optable.optable[opcode as usize] {
        Instruction::AdcHL => Self::adc_hl(self, memory),
        Instruction::AdcN => Self::adc_n(self, memory),
        Instruction::AdcR(r) => Self::adc_r(self, memory, *r),
        Instruction::AddHL => Self::add_hl(self, memory),
        Instruction::AddHLRR(rr) => Self::add_hl_rr(self, memory, *rr),
        Instruction::AddN => Self::add_n(self, memory),
        Instruction::AddR(r) => Self::add_r(self, memory, *r),
        Instruction::AddSPE => Self::add_sp_e(self, memory),
        Instruction::AndHL => Self::and_hl(self, memory),
        Instruction::AndN => Self::and_n(self, memory),
        Instruction::AndR(r) => Self::and_r(self, memory, *r),
        Instruction::Call => Self::call_nn(self, memory),
        Instruction::CallCcNn(cc, set ) => Self::call_cc_nn(self, memory, *cc, *set),
        Instruction::Ccf => Self::ccf(self, memory),
        Instruction::CpHL => Self::cp_hl(self, memory),
        Instruction::Cpl => Self::cpl(self, memory),
        Instruction::CpN => Self::cp_n(self, memory),
        Instruction::CpR(r) => Self::cp_r(self, memory, *r),
        Instruction::Daa => Self::daa(self, memory),
        Instruction::Dec(r) => Self::dec_n(self, memory, *r),
        Instruction::DecHL => Self::dec_hl(self, memory),
        Instruction::DecRR(rr) => Self::dec_rr(self, memory, *rr),
        Instruction::Di => Self::di(self, memory),
        Instruction::Ei => Self::ei(self, memory),
        Instruction::Halt => Self::halt(self, memory),
        Instruction::IncHL => Self::inc_hl(self, memory),
        Instruction::IncNn(r1) => Self::inc_nn(self, memory, *r1),
        Instruction::IncR(r) => Self::inc_r(self, memory, *r),
        Instruction::Invalid => Self::invalid_instruction(self, memory),
        Instruction::JpCCNN(cc, set) => Self::jp_cc_nn(self, memory, *cc, *set),
        Instruction::JpHL => Self::jp_hl(self, memory),
        Instruction::JpNN => Self::jp_nn(self, memory),
        Instruction::JrCCE(cc, set) => Self::jr_cc_e(self, memory, *cc, *set),
        Instruction::JrE => Self::jr_e(self, memory),
        Instruction::LdAHLD => Self::ld_a_hld(self, memory),
        Instruction::LdAHLI => Self::ld_a_hli(self, memory),
        Instruction::LdANn => Self::ld_a_nn(self, memory),
        Instruction::LdARR(rr) => Self::ld_a_rr(self, memory, *rr),
        Instruction::LdhAC => Self::ldh_a_c(self, memory),
        Instruction::LdhAN => Self::ldh_a_n(self, memory),
        Instruction::LdhCA => Self::ldh_c_a(self, memory),
        Instruction::LdHLDA => Self::ld_hld_a(self, memory),
        Instruction::LdHLIA => Self::ld_hli_a(self, memory),
        Instruction::LdHLN => Self::ld_hl_n(self, memory),
        Instruction::LdHLSPE => Self::ld_hl_sp_e(self, memory),
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
        Instruction::LdSPHL => Self::ld_sp_hl(self, memory),
        Instruction::Nop => Self::nop(self, memory),
        Instruction::OrAHL => Self::or_a_hl(self, memory),
        Instruction::OrN => Self::or_n(self, memory),
        Instruction::OrR(r) => Self::or_r(self, memory, *r),
        Instruction::PopRR(rr) => Self::pop_rr(self, memory, *rr),
        Instruction::PushRR(r) => Self::push_rr(self, memory, *r),
        Instruction::Ret => Self::ret(self, memory),
        Instruction::RetCC(cc, set) => Self::ret_cc(self, memory, *cc, *set),
        Instruction::Reti => Self::reti(self, memory),
        Instruction::Rla => Self::rla(self, memory),
        Instruction::Rlca => Self::rlca(self, memory),
        Instruction::Rra => Self::rra(self, memory),
        Instruction::Rrca => Self::rrca(self, memory),
        Instruction::Rst(jump_address) => Self::rst_n(self, memory, *jump_address),
        Instruction::SbcHL => Self::sbc_hl(self, memory),
        Instruction::SbcN => Self::sbc_n(self, memory),
        Instruction::SbcR(r) => Self::sbc_r(self, memory, *r),
        Instruction::Scf => Self::scf(self, memory),
        Instruction::Stop => Self::stop(self, memory),
        Instruction::SubHL => Self::sub_hl(self, memory),
        Instruction::SubN => Self::sub_n(self, memory),
        Instruction::SubR(r) => Self::sub_r(self, memory, *r),
        Instruction::Unimplemented => Self::unimplemented_instruction(self, memory),
        Instruction::Xor(r) => Self::xor_r(self, memory, *r),
        Instruction::XorAN => Self::xor_a_n(self, memory),
        Instruction::XorHL => Self::xor_hl(self, memory),
      }
    }

    self.cycles += self.cycles_table.cycle_table[opcode as usize];

    unsafe { TIMA_INTERNAL_COUNTER += self.cycles_table.cycle_table[opcode as usize] };

    // Increment TIMA if TAC is enabled
    if (memory.read(0xFF07) & 0b_0000_0100) == 0b_0000_0100 {
      let tac_clock_select = memory.read(0xFF07) & (1 << 2) - 1;
      let mut incremented_value: u16 = self.registers.tima.into();

      match tac_clock_select {
        0b_00 => {
          while unsafe { TIMA_INTERNAL_COUNTER } >= ((MASTER_CLOCK_SPEED / TAC_CLOCK_SELECT[0]) / 4) {
            incremented_value = self.registers.tima.wrapping_add(1).into();
            unsafe { TIMA_INTERNAL_COUNTER = TIMA_INTERNAL_COUNTER.wrapping_sub((MASTER_CLOCK_SPEED / TAC_CLOCK_SELECT[0]) / 4); };
          }
        },
        0b_01 => {
          while unsafe { TIMA_INTERNAL_COUNTER } >= ((MASTER_CLOCK_SPEED / TAC_CLOCK_SELECT[1]) / 4) {
            incremented_value = self.registers.tima.wrapping_add(1).into();
            unsafe { TIMA_INTERNAL_COUNTER = TIMA_INTERNAL_COUNTER.wrapping_sub((MASTER_CLOCK_SPEED / TAC_CLOCK_SELECT[1]) / 4); };
          }
        },
        0b_10 => {
          while unsafe { TIMA_INTERNAL_COUNTER } >= ((MASTER_CLOCK_SPEED / TAC_CLOCK_SELECT[2]) / 4) {
            incremented_value = self.registers.tima.wrapping_add(1).into();
            unsafe { TIMA_INTERNAL_COUNTER = TIMA_INTERNAL_COUNTER.wrapping_sub((MASTER_CLOCK_SPEED / TAC_CLOCK_SELECT[2]) / 4); };
          }
        },
        0b_11 => {
          while unsafe { TIMA_INTERNAL_COUNTER } >= ((MASTER_CLOCK_SPEED / TAC_CLOCK_SELECT[3]) / 4) {
            incremented_value = self.registers.tima.wrapping_add(1).into();
            unsafe { TIMA_INTERNAL_COUNTER = TIMA_INTERNAL_COUNTER.wrapping_sub((MASTER_CLOCK_SPEED / TAC_CLOCK_SELECT[3]) / 4); };
          }
        },
        _ => {
          incremented_value = self.registers.tima.wrapping_add(1).into();
        }
      }

      // Request timer interrupt if TIMA overflows
      if incremented_value > 0xFF {
        let mut interrupt_flag = memory.read(0xFF0F);
        interrupt_flag |= 0b_0000_0100;
        memory.write(0xFF0F, interrupt_flag);
      }

      self.registers.tima = incremented_value.to_le_bytes()[0];
      memory.write(0xFF05, incremented_value.to_le_bytes()[0]);
    }

    unsafe { DIV_INTERNAL_COUNTER += self.cycles_table.cycle_table[opcode as usize] };

    while unsafe { DIV_INTERNAL_COUNTER } >= (MASTER_CLOCK_SPEED / DIV_INCREMENT_RATE) {
      self.registers.div = self.registers.div.wrapping_add(1);
      unsafe { DIV_INTERNAL_COUNTER = DIV_INTERNAL_COUNTER.wrapping_sub(MASTER_CLOCK_SPEED / DIV_INCREMENT_RATE); };
    }

    let interrupt_flag = memory.read(0xFF0F);

    if self.interrupt_master_enable_flag {
      if interrupt_flag != 0 {
        handle_interrupt(self, memory, memory.read(0xFFFF), interrupt_flag);
      }
    }
  }

  pub fn handle_flags(&mut self, z: Option<bool>, n: Option<bool>, h: Option<bool>, c: Option<bool>) {
    if let Some(true) = z {
      self.registers.set_z_flag();
    } else if let Some(false) = z {
      self.registers.unset_z_flag();
    }

    if let Some(true) = n {
      self.registers.set_n_flag();
    } else if let Some(false) = n {
      self.registers.unset_n_flag();
    }

    if let Some(true) = h {
      self.registers.set_h_flag();
    } else if let Some(false) = h {
      self.registers.unset_h_flag();
    }

    if let Some(true) = c {
      self.registers.set_c_flag();
    } else if let Some(false) = c {
      self.registers.unset_c_flag();
    }
  }

  fn unimplemented_instruction(&mut self, memory: &mut Memory) {
    if memory.read(self.registers.pc - 1) == 0xCB {
      panic!("Prefixed instruction not yet implemented. Opcode: 0x{:02X}. PC: 0x{:02X}", memory.read(self.registers.pc), self.registers.pc);
    } else {
      panic!("Instruction not yet implemented. Opcode: 0x{:02X}. PC: 0x{:02X}", memory.read(self.registers.pc), self.registers.pc);
    }
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

  fn ld_a_nn(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let low = memory.read(pc + 1);
    let high = memory.read(pc + 2);
    let nn = ((high as u16) << 8) | (low as u16);

    self.registers.a = memory.read(nn);

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

  fn ld_a_rr(&mut self, memory: &mut Memory, rr: RegisterPair) {
    self.registers.a = memory.read(self.registers.get_pair(rr));
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

  fn ldh_c_a(&mut self, memory: &mut Memory) {
    memory.write((0xFF as u16) << 8 | self.registers.c as u16, self.registers.a);

    self.registers.pc += 1;
  }

  fn ldh_a_c(&mut self, memory: &mut Memory) {
    self.registers.a = memory.read((0xFF as u16) << 8 | self.registers.c as u16);

    self.registers.pc += 1;
  }

  fn ld_hl_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let n = memory.read(pc + 1);

    memory.write(self.registers.get_pair(RegisterPair::HL), n);

    self.registers.pc += 2;
  }

  fn ld_hl_sp_e(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let sp = self.registers.sp;
    let e = memory.read(pc + 1) as i8;
    let result = sp.wrapping_add_signed(e.into());
    let (mut set_h_flag, mut set_c_flag) = (false, false);

    self.registers.set_pair(RegisterPair::HL, result);

    let half_carry;
    
    if (((sp & 0xF).wrapping_add_signed(e as i16 & 0xF)) & 0x10) == 0x10 {
      half_carry = true;
    } else {
      half_carry = false;
    };

    if half_carry {
      set_h_flag = true;
    }

    let carry;

    if (((sp & 0xFF).wrapping_add_signed(e as i16 & 0xFF)) & 0x100) == 0x100 {
      carry = true;
    } else {
      carry = false;
    }

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(false), Some(false), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 2;
  }

  fn ld_hli_a(&mut self, memory: &mut Memory) {
    let hl = self.registers.get_pair(RegisterPair::HL);
    memory.write(hl, self.registers.a);
    self.registers.set_pair(RegisterPair::HL, hl.wrapping_add(1));
    self.registers.pc += 1;
  }

  fn ld_sp_hl(&mut self, _memory: &mut Memory) {
    self.registers.sp = self.registers.get_pair(RegisterPair::HL);

    self.registers.pc += 1;
  }

  fn call_nn(&mut self, memory: &mut Memory) {
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

  fn call_cc_nn(&mut self, memory: &mut Memory, cc: Flag, set: bool) {
    let pc = self.registers.pc;
    let sp = self.registers.sp;
    let low = memory.read(pc + 1);
    let high = memory.read(pc + 2);
    let nn = ((high as u16) << 8) | (low as u16);
    let split_u8_values = (pc + 3).to_le_bytes();
    self.registers.pc += 3;

    match cc {
      Flag::Z => {
        if set && self.registers.get_z_flag() {
          self.registers.sp -= 1;
          memory.write(sp - 1, split_u8_values[1]);
          self.registers.sp -= 1;
          memory.write(sp - 2, split_u8_values[0]);
          self.registers.pc = nn;
        } else if !set && !self.registers.get_z_flag() {
          self.registers.sp -= 1;
          memory.write(sp - 1, split_u8_values[1]);
          self.registers.sp -= 1;
          memory.write(sp - 2, split_u8_values[0]);
          self.registers.pc = nn;
        }
      },
      Flag::C => {
        if set && self.registers.get_c_flag() {
          self.registers.sp -= 1;
          memory.write(sp - 1, split_u8_values[1]);
          self.registers.sp -= 1;
          memory.write(sp - 2, split_u8_values[0]);
          self.registers.pc = nn;
        } else if !set && !self.registers.get_c_flag() {
          self.registers.sp -= 1;
          memory.write(sp - 1, split_u8_values[1]);
          self.registers.sp -= 1;
          memory.write(sp - 2, split_u8_values[0]);
          self.registers.pc = nn;
        }
      },
      Flag::N => panic!("This flag must not be used here"),
      Flag::H => panic!("This flag must not be used here"),
    }
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
        } else {
          self.registers.pc += 3;
        }
      },
      Flag::C => {
        if set && self.registers.get_c_flag() {
          self.registers.pc = nn;
        } else if !set && !self.registers.get_c_flag() {
          self.registers.pc = nn;
        } else {
          self.registers.pc += 3;
        }
      },
      Flag::N => panic!("This flag must not be used here"),
      Flag::H => panic!("This flag must not be used here"),
    }
  }

  fn jp_hl(&mut self, _memory: &mut Memory) {
    self.registers.pc += 1;

    self.registers.pc = self.registers.get_pair(RegisterPair::HL);
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
    self.registers.pc += 1;
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

    let half_carry = bit_operations::get_half_carry(prev, 1);

    if half_carry {
      self.registers.set_h_flag();
    } else {
      self.registers.unset_h_flag();
    }
  
    self.registers.pc += 1;
  }

  fn inc_nn(&mut self, _memory: &mut Memory, r1: Target) {
    if let Target::SingleU16(_register) = r1 {
      self.registers.sp = self.registers.sp.wrapping_add(1);
    }

    if let Target::Pair(register) = r1 {
      self.registers.set_pair(register, self.registers.get_pair(register).wrapping_add(1));
    }

    self.registers.pc += 1;
  }

  fn inc_hl(&mut self, memory: &mut Memory) {
    let hl = self.registers.get_pair(RegisterPair::HL);
    let prev = memory.read(hl);
    let result = prev.wrapping_add(1);
    let (mut set_z_flag, mut set_h_flag) = (false, false);

    memory.write(hl, result);

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry = bit_operations::get_half_carry(prev, 1);

    if half_carry {
      set_h_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(set_h_flag), None);

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

    let half_carry = bit_operations::get_half_carry_sub(prev, 1);

    if half_carry {
      self.registers.set_h_flag();
    } else {
      self.registers.unset_h_flag();
    }

    self.registers.pc += 1;
  }

  fn dec_hl(&mut self, memory: &mut Memory) {
    let hl = self.registers.get_pair(RegisterPair::HL);
    let data = memory.read(hl);
    let result = data.wrapping_sub(1);
    let (mut set_z_flag, mut set_h_flag) = (false, false);
  
    memory.write(hl, result);

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry = bit_operations::get_half_carry_sub(data, 1);

    if half_carry {
      set_h_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(true), Some(set_h_flag), None);

    self.registers.pc += 1;
  }

  fn dec_rr(&mut self, _memory: &mut Memory, rr: Target) {
    if let Target::Pair(register_pair) = rr {
      self.registers.set_pair(register_pair, self.registers.get_pair(register_pair).wrapping_sub(1));
    } else if let Target::SingleU16(RegisterU16::SP) = rr {
      self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    self.registers.pc += 1;
  }

  fn add_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let prev = self.registers.a;
    let r = self.registers[r];
    let result = self.registers.a.wrapping_add(r);
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);

    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry = bit_operations::get_half_carry(prev, r);

    if half_carry {
      set_h_flag = true;
    }

    let carry = bit_operations::get_carry(prev, r);

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn add_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let n = memory.read(pc + 1);
    let prev = self.registers.a;
    let result = self.registers.a.wrapping_add(n);
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);

    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry = bit_operations::get_half_carry(prev, n);

    if half_carry {
      set_h_flag = true;
    }

    let carry = bit_operations::get_carry(prev, n);

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 2;
  }

  fn add_hl_rr(&mut self, _memory: &mut Memory, rr: Target) {
    let hl = self.registers.get_pair(RegisterPair::HL);
    let result;
    let (mut set_h_flag, mut set_c_flag) = (false, false);

    if let Target::Pair(register_pair) = rr {
      result = hl.wrapping_add(self.registers.get_pair(register_pair));
    } else {
      result = hl.wrapping_add(self.registers.sp);
    }

    let half_carry;

    if let Target::Pair(register_pair) = rr {
      half_carry = bit_operations::get_half_carry_16_bit_high(hl, self.registers.get_pair(register_pair));

      if half_carry {
        set_h_flag = true;
      }
    } else {
      half_carry = bit_operations::get_half_carry_16_bit_high(hl, self.registers.sp);

      if half_carry {
        set_h_flag = true;
      }
    }

    let carry;

    if let Target::Pair(register_pair) = rr {
      carry = bit_operations::get_carry_16_bit_high(hl, self.registers.get_pair(register_pair));

      if carry {
        set_c_flag = true;
      }
    } else {
      carry = bit_operations::get_carry_16_bit_high(hl, self.registers.sp);

      if carry {
        set_c_flag = true;
      }
    }

    self.registers.set_pair(RegisterPair::HL, result);

    Self::handle_flags(self, None, Some(false), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn add_hl(&mut self, memory: &mut Memory) {
    let hl = self.registers.get_pair(RegisterPair::HL);
    let a = self.registers.a;
    let prev = memory.read(hl);
    let result = prev.wrapping_add(a);
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);

    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry = bit_operations::get_half_carry(prev, a);

    if half_carry {
      set_h_flag = true;
    }

    let carry = bit_operations::get_carry(prev, a);

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn add_sp_e(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let sp = self.registers.sp;
    let e = memory.read(pc + 1) as i8;
    let result = sp.wrapping_add_signed(e.into());
    let (mut set_h_flag, mut set_c_flag) = (false, false);

    self.registers.sp = result;

    let half_carry;
    
    if (((sp & 0xF).wrapping_add_signed(e as i16 & 0xF)) & 0x10) == 0x10 {
      half_carry = true;
    } else {
      half_carry = false;
    };

    if half_carry {
      set_h_flag = true;
    }

    let carry;

    if (((sp & 0xFF).wrapping_add_signed(e as i16 & 0xFF)) & 0x100) == 0x100 {
      carry = true;
    } else {
      carry = false;
    }

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(false), Some(false), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 2;
  }

  fn adc_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let c_flag = self.registers.get_c_flag();
    let prev = self.registers.a;
    let r = self.registers[r];
    let result;
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);

    if c_flag {
      result = self.registers.a.wrapping_add(r.wrapping_add(1));
    } else {
      result = self.registers.a.wrapping_add(r);
    }

    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry;

    if c_flag {
      half_carry = bit_operations::get_half_carry_16_bit_low(prev as u16, r as u16, 1);
    } else {
      half_carry = bit_operations::get_half_carry_16_bit_low(prev as u16, r as u16, 0);
    }

    if half_carry {
      set_h_flag = true;
    }

    let carry;

    if c_flag {
      carry = bit_operations::get_carry_16_bit_low(prev as u16, r as u16, 1);
    } else {
      carry = bit_operations::get_carry_16_bit_low(prev as u16, r as u16, 0);
    }

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn adc_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let n = memory.read(pc + 1);
    let prev = self.registers.a;
    let c_flag = self.registers.get_c_flag();
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);

    let result;

    if c_flag {
      result = prev.wrapping_add(n).wrapping_add(1);
    } else {
      result = prev.wrapping_add(n);
    }

    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry;

    if c_flag {
      half_carry = bit_operations::get_half_carry_refactor(prev, &[n, 1]);
    } else {
      half_carry = bit_operations::get_half_carry_refactor(prev, &[n]);
    }

    if half_carry {
      set_h_flag = true;
    }

    let carry;

    if c_flag {
      carry = bit_operations::get_carry_refactor(prev, &[n, 1]);
    } else {
      carry = bit_operations::get_carry_refactor(prev, &[n]);
    }

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 2;
  }

  fn adc_hl(&mut self, memory: &mut Memory) {
    let a = self.registers.a;
    let c_flag = self.registers.get_c_flag();
    let hl = self.registers.get_pair(RegisterPair::HL);
    let data = memory.read(hl);
    let result;
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);

    if c_flag {
      result = a.wrapping_add(data.wrapping_add(1));
    } else {
      result = a.wrapping_add(data);
    }

    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry;

    if c_flag {
      half_carry = bit_operations::get_half_carry_16_bit_low(a as u16, data as u16, 1);
    } else {
      half_carry = bit_operations::get_half_carry_16_bit_low(a as u16, data as u16, 0);
    }

    if half_carry {
      set_h_flag = true;
    }

    let carry;

    if c_flag {
      carry = bit_operations::get_carry_16_bit_low(a as u16, data as u16, 1);
    } else {
      carry = bit_operations::get_carry_16_bit_low(a as u16, data as u16, 0);
    }

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn sub_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let prev = self.registers.a;
    let r = self.registers[r];
    let result = self.registers.a.wrapping_sub(r);
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);

    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry = bit_operations::get_half_carry_sub(prev, r);

    if half_carry {
      set_h_flag = true;
    }

    let carry = bit_operations::get_carry_sub(prev, r);

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(true), Some(set_h_flag), Some(set_c_flag));
    
    self.registers.pc += 1;
  }

  fn sub_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let prev = self.registers.a;
    let n = memory.read(pc + 1);
    let result = self.registers.a.wrapping_sub(n);
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);
  
    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry = bit_operations::get_half_carry_sub(prev, n);

    if half_carry {
      set_h_flag = true;
    }

    let carry = bit_operations::get_carry_sub(prev, n);

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(true), Some(set_h_flag), Some(set_c_flag));
  
    self.registers.pc += 2;
  }

  fn sub_hl(&mut self, memory: &mut Memory) {
    let hl = self.registers.get_pair(RegisterPair::HL);
    let a = self.registers.a;
    let data = memory.read(hl);
    let result = a.wrapping_sub(data);
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);

    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry = bit_operations::get_half_carry_sub(a, data);

    if half_carry {
      set_h_flag = true;
    }

    let carry = bit_operations::get_carry_sub(a, data);

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(true), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn sbc_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let c_flag = self.registers.get_c_flag();
    let prev = self.registers.a;
    let r = self.registers[r];
    let result;
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);

    if c_flag {
      result = self.registers.a.wrapping_sub(r).wrapping_sub(1);
    } else {
      result = self.registers.a.wrapping_sub(r);
    }

    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry;

    if c_flag {
      half_carry = bit_operations::get_half_carry_sub_refactor(prev, &[r, 1]);
    } else {
      half_carry = bit_operations::get_half_carry_sub_refactor(prev, &[r]);
    }

    if half_carry {
      set_h_flag = true;
    }

    let carry;

    if c_flag {
      carry = bit_operations::get_carry_sub_refactor(prev, &[r, 1]);
    } else {
      carry = bit_operations::get_carry_sub_refactor(prev, &[r]);
    }

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(true), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn sbc_hl(&mut self, memory: &mut Memory) {
    let hl = self.registers.get_pair(RegisterPair::HL);
    let a = self.registers.a;
    let c_flag = self.registers.get_c_flag();
    let data = memory.read(hl);
    let result;
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);

    if c_flag {
      result = a.wrapping_sub(data).wrapping_sub(1);
    } else {
      result = a.wrapping_sub(data);
    }

    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry;

    if c_flag {
      half_carry = bit_operations::get_half_carry_sub_refactor(a, &[data, 1]);
    } else {
      half_carry = bit_operations::get_half_carry_sub_refactor(a, &[data]);
    }

    if half_carry {
      set_h_flag = true;
    }

    let carry;

    if c_flag {
      carry = bit_operations::get_carry_sub(a, data.wrapping_sub(1));
    } else {
      carry = bit_operations::get_carry_sub(a, data);
    }

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(true), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn sbc_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let c_flag = self.registers.get_c_flag();
    let prev = self.registers.a;
    let result;
    let n = memory.read(pc + 1);
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);

    if c_flag {
      result = self.registers.a.wrapping_sub(n).wrapping_sub(1);
    } else {
      result = self.registers.a.wrapping_sub(n);
    }

    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry;

    if c_flag {
      half_carry = bit_operations::get_half_carry_sub_refactor(prev, &[n, 1]);
    } else {
      half_carry = bit_operations::get_half_carry_sub_refactor(prev, &[n]);
    }

    if half_carry {
      set_h_flag = true;
    }

    let carry;

    if c_flag {
      carry = bit_operations::get_carry_sub_refactor(prev, &[n, 1]);
    } else {
      carry = bit_operations::get_carry_sub_refactor(prev, &[n]);
    }

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(true), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 2;
  }

  fn xor_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let result = self.registers[r] ^ self.registers.a;
    let mut set_z_flag = false;
    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(false), Some(false));

    self.registers.pc += 1;
  }

  fn xor_hl(&mut self, memory: &mut Memory) {
    let data = memory.read(self.registers.get_pair(RegisterPair::HL));
    let result = self.registers.a ^ data;
    let mut set_z_flag = false;
    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(false), Some(false));

    self.registers.pc += 1;
  }

  fn xor_a_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let n =  memory.read(pc + 1);
    let result = self.registers.a ^ n;
    let mut set_z_flag = false;
    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(false), Some(false));

    self.registers.pc += 2;
  }

  fn and_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let result = self.registers.a & memory.read(pc + 1);
    let mut set_z_flag = false;

    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(true), Some(false));

    self.registers.pc += 2;
  }

  fn and_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let result = self.registers.a & self.registers[r];
    let mut set_z_flag = false;
    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(true), Some(false));

    self.registers.pc += 1;
  }

  fn and_hl(&mut self, memory: &mut Memory) {
    let data = memory.read(self.registers.get_pair(RegisterPair::HL));
    let result = self.registers.a & data;
    self.registers.a = result;
    let mut set_z_flag = false; 

    if result == 0 {
      set_z_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(true), Some(false));

    self.registers.pc += 1;
  }

  fn or_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let result = self.registers.a | memory.read(pc + 1);
    let mut set_z_flag = false;
  
    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(false), Some(false));

    self.registers.pc += 2;
  }

  fn or_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let result = self.registers.a | self.registers[r];
    let mut set_z_flag = false;

    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(false), Some(false));
    
    self.registers.pc += 1;
  }

  fn or_a_hl(&mut self, memory: &mut Memory) {
    let result = self.registers.a | memory.read(self.registers.get_pair(RegisterPair::HL));
    let mut set_z_flag = false;

    self.registers.a = result;

    if result == 0 {
      set_z_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(false), Some(false));
    
    self.registers.pc += 1;
  }

  fn ei(&mut self, _memory: &mut Memory) {
    self.interrupt_master_enable_flag = true;

    self.registers.pc += 1;
  }

  fn di(&mut self, _memory: &mut Memory) {
    self.interrupt_master_enable_flag = false;

    self.registers.pc += 1;
  }

  fn reti(&mut self, memory: &mut Memory) {
    let sp = self.registers.sp;
    self.registers.sp += 2;

    let low = memory.read(sp);
    let high = memory.read(sp + 1);

    self.registers.pc = ((high as u16) << 8) | (low as u16);
    
    self.interrupt_master_enable_flag = true;
  }

  pub fn ret(&mut self, memory: &mut Memory) {
    let sp = self.registers.sp;
    self.registers.sp += 2;

    let low = memory.read(sp);
    let high = memory.read(sp + 1);

    self.registers.pc = ((high as u16) << 8) | (low as u16);
  }

  fn ret_cc(&mut self, memory: &mut Memory, cc: Flag, set: bool) {
    let sp = self.registers.sp;
    let low = memory.read(sp);
    let high = memory.read(sp + 1);
    self.registers.pc += 1;

    match cc {
      Flag::Z => {
        if set && self.registers.get_z_flag() {
          self.registers.sp += 2;
          self.registers.pc = ((high as u16) << 8) | (low as u16);
        } else if !set && !self.registers.get_z_flag() {
          self.registers.sp += 2;
          self.registers.pc = ((high as u16) << 8) | (low as u16);
        }
      },
      Flag::C => {
        if set && self.registers.get_c_flag() {
          self.registers.sp += 2;
          self.registers.pc = ((high as u16) << 8) | (low as u16);
        } else if !set && !self.registers.get_c_flag() {
          self.registers.sp += 2;
          self.registers.pc = ((high as u16) << 8) | (low as u16);
        }
      },
      Flag::N => panic!("This flag must not be used here"),
      Flag::H => panic!("This flag must not be used here"),
    }
  }

  fn stop(&mut self, _memory: &mut Memory) {
    // TODO
    self.registers.div = 0;

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
    let mut set_c_flag = true;

    let c_flag = self.registers.get_c_flag();

    if c_flag {
      set_c_flag = false;
    }

    Self::handle_flags(self, None, Some(false), Some(false), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn rrca(&mut self, _memory: &mut Memory) {
    let b0 = self.registers.a & (1 << 0) != 0;
    let mut set_c_flag: bool = false;

    self.registers.a = self.registers.a.rotate_right(1);
    
    if b0 {
      self.registers.a |= 0b1000_0000;
    } else {
      self.registers.a &= 0b0111_1111;
    }

    if b0 {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(false), Some(false), Some(false), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn rra(&mut self, _memory: &mut Memory) {
    let b0 = self.registers.a & (1 << 0) != 0;
    let c_flag = self.registers.get_c_flag();
    let mut set_c_flag = false;

    self.registers.a = self.registers.a.rotate_right(1);
    
    if c_flag {
      self.registers.a |= 0b1000_0000;
    } else {
      self.registers.a &= 0b0111_1111;
    }

    if b0 {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(false), Some(false), Some(false), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn rla(&mut self, _memory: &mut Memory) {
    let b7 = self.registers.a & (1 << 7) != 0;
    let c_flag = self.registers.get_c_flag();
    let mut set_c_flag = false;

    self.registers.a = self.registers.a.rotate_left(1);
    
    if c_flag {
      self.registers.a |= 0b0000_0001;
    } else {
      self.registers.a &= 0b1111_1110;
    }

    if b7 {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(false), Some(false), Some(false), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn rlca(&mut self, _memory: &mut Memory) {
    let b7 = self.registers.a & (1 << 7) != 0;
    let mut set_c_flag = false;

    self.registers.a = self.registers.a.rotate_left(1);
    
    if b7 {
      self.registers.a |= 0b0000_0001;
    } else {
      self.registers.a &= 0b1111_1110;
    }

    if b7 {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(false), Some(false), Some(false), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn cp_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let prev = self.registers.a;
    let result = self.registers.a.wrapping_sub(self.registers[r]);
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry = bit_operations::get_half_carry_sub(prev, self.registers[r]);
    
    if half_carry {
      set_h_flag = true;
    }

    let carry = bit_operations::get_carry_sub(prev, self.registers[r]);

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(true), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn cp_n(&mut self, memory: &mut Memory) {
    let pc = self.registers.pc;
    let n = memory.read(pc + 1);
    let prev = self.registers.a;
    let result = self.registers.a.wrapping_sub(n);
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry = bit_operations::get_half_carry_sub(prev, n);

    if half_carry {
      set_h_flag = true;
    }

    let carry = bit_operations::get_carry_sub(prev, n);

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(true), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 2;
  }

  fn cp_hl(&mut self, memory: &mut Memory) {
    let a = self.registers.a;
    let data = memory.read(self.registers.get_pair(RegisterPair::HL));
    let result = self.registers.a.wrapping_sub(data);
    let (mut set_z_flag, mut set_h_flag, mut set_c_flag) = (false, false, false);

    if result == 0 {
      set_z_flag = true;
    }

    let half_carry = bit_operations::get_half_carry_sub(a, data);

    if half_carry {
      set_h_flag = true;
    }

    let carry = bit_operations::get_carry_sub(a, data);

    if carry {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(true), Some(set_h_flag), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn halt(&mut self, _memory: &mut Memory) {
    // TODO

    self.registers.pc += 1;
  }

  fn pop_rr(&mut self, memory: &mut Memory, rr: RegisterPair) {
    let sp = self.registers.sp;

    let low = memory.read(sp);
    let high = memory.read(sp + 1);

    let is_af_register = matches!(rr, RegisterPair::AF { .. });

    if is_af_register {
      let (mut set_z_flag, mut set_n_flag, mut set_h_flag, mut set_c_flag) = (false, false, false, false);

      let b7 = low & (1 << 7) != 0;
      let b6 = low & (1 << 6) != 0;
      let b5 = low & (1 << 5) != 0;
      let b4 = low & (1 << 4) != 0;

      set_z_flag = b7;
      set_n_flag = b6;
      set_h_flag = b5;
      set_c_flag = b4;

      self.registers.set_pair(rr, ((high as u16) << 8) | ((low & 0xF0) as u16));
      Self::handle_flags(self, Some(set_z_flag), Some(set_n_flag), Some(set_h_flag), Some(set_c_flag));
    } else {
      self.registers.set_pair(rr, ((high as u16) << 8) | (low as u16));
    }

    self.registers.sp += 2;
    self.registers.pc += 1;
  }

  fn cb_rlc_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let b7 = self.registers[r] & (1 << 7) != 0;
    let (mut set_z_flag, mut set_c_flag) = (false, false);

    self.registers[r] = self.registers[r].rotate_left(1);

    if b7 {
      self.registers[r] |= 0b0000_0001;
    } else {
      self.registers[r] &= 0b1111_1110;
    }

    if self.registers[r] == 0 {
      set_z_flag = true;
    }

    if b7 {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(false), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn cb_srl_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let b0 = self.registers[r] & (1) != 0;
    let (mut set_z_flag, mut set_c_flag) = (false, false);
    self.registers[r] = self.registers[r] >> 1;

    if self.registers[r] == 0 {
      set_z_flag = true;
    }

    if b0 {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(false), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn cb_rr_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let b0 = self.registers[r] & (1) != 0;
    let c_flag = self.registers.get_c_flag();
    let (mut set_z_flag, mut set_c_flag) = (false, false);
    self.registers[r] = self.registers[r].rotate_right(1);

    if c_flag {
      self.registers[r] |= 0b1000_0000;
    } else {
      self.registers[r] &= 0b0111_1111;
    }

    if self.registers[r] == 0 {
      set_z_flag = true;
    }

    if b0 {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(false), Some(set_c_flag));
    
    self.registers.pc += 1;
  }

  fn cb_set_b_hl(&mut self, memory: &mut Memory, b: usize) {
    let hl = self.registers.get_pair(RegisterPair::HL);
    let data = memory.read(hl);
    let result = self.registers.set_bit(data, b, true);

    memory.write(hl, result);

    self.registers.pc += 1;
  }

  fn cb_rrc_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let b0 = self.registers[r] & (1) != 0;

    let (mut set_z_flag, mut set_c_flag) = (false, false);

    self.registers[r] = self.registers[r].rotate_right(1);

    if b0 {
      self.registers[r] |= 0b1000_0000;
    } else {
      self.registers[r] &= 0b0111_1111;
    }

    if self.registers[r] == 0 {
      set_z_flag = true;
    }

    if b0 {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(false), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn cb_rl_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let b7 = self.registers[r] & (1 << 7) != 0;
    let c_flag = self.registers.get_c_flag();
    let (mut set_z_flag, mut set_c_flag) = (false, false);

    self.registers[r] = self.registers[r].rotate_left(1);

    if c_flag {
      self.registers[r] |= 0b0000_0001;
    } else {
      self.registers[r] &= 0b1111_1110;
    }

    if self.registers[r] == 0 {
      set_z_flag = true;
    }

    if b7 {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(false), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn cb_sla_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let b7 = self.registers[r] & (1 << 7) != 0;
    self.registers[r] = self.registers[r] << 1;
    let (mut set_z_flag, mut set_c_flag) = (false, false);

    if self.registers[r] == 0 {
      set_z_flag = true;
    }

    if b7 {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(false), Some(set_c_flag));

    self.registers.pc += 1;
  }

  fn cb_sra_r(&mut self, _memory: &mut Memory, r: RegisterU8) {
    let b0 = self.registers[r] & (1 << 0) != 0;
    self.registers[r] = self.registers[r] >> 1;
    let (mut set_z_flag, mut set_c_flag) = (false, false);

    if self.registers[r] == 0 {
      set_z_flag = true;
    }

    if b0 {
      set_c_flag = true;
    }

    Self::handle_flags(self, Some(set_z_flag), Some(false), Some(false), Some(set_c_flag));

    self.registers.pc += 1;
  }

  // Inspired by https://blog.ollien.com/posts/gb-daa/
  fn daa(&mut self, _memory: &mut Memory) {
    let mut offset = 0_u8;
    let mut should_carry = false;
    let mut result = 0_u8;

    let a = self.registers.a;

    let half_carry = self.registers.get_h_flag();
    let carry = self.registers.get_c_flag();
    let subtract = self.registers.get_n_flag();

    if (!subtract && a & 0xF > 0x09 || half_carry) {
      offset |= 0x06;
    }

    if (!subtract && a > 0x99 || carry) {
      offset |= 0x60;
      should_carry = true;
    }

    if !subtract {
      result = a.wrapping_add(offset);
    } else {
      result = a.wrapping_sub(offset);
    };

    self.registers.a = result;
    Self::handle_flags(self, Some((result == 0)), None, Some(false), Some(should_carry));

    self.registers.pc += 1;
  }
}
