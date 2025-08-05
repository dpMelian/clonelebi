use cpu::instructions::CycleTable;
use cpu::instructions::Optable;
use cpu::registers::Registers;
use cpu::registers::RegisterPair;
use cpu::cpu::Cpu;
use memory::memory::Memory;


struct Setup {
  cpu: Cpu,
  memory: Memory
}

impl Setup {
  pub fn new() -> Self {
    Self {
      cpu: Cpu {
        registers: Registers::new(),
        optable: Optable::new(),
        cycles: 0,
        cycles_table: CycleTable::new()
      },
      memory: Memory::new()
    }
  }
}

#[test]
fn test_cpl() {
  let mut setup = Setup::new();

  setup.memory.write(0x0000, 0x2F);

  setup.cpu.registers.a = 0x13;
  setup.cpu.registers.pc = 0x00;

  setup.cpu.run_instruction(&mut setup.memory);

  assert_eq!(setup.cpu.registers.a, 0xEC);
}

#[test]
fn test_inc() {
  let mut setup = Setup::new();

  setup.memory.write(0x0000, 0x3C);

  setup.cpu.registers.a = 0x13;
  setup.cpu.registers.pc = 0x00;

  setup.cpu.run_instruction(&mut setup.memory);

  assert_eq!(setup.cpu.registers.a, 0x14);
}

#[test]
fn test_ld_r1_r2() {
  let mut setup = Setup::new();

  setup.memory.write(0x0000, 0x78);

  setup.cpu.registers.a = 0x13;
  setup.cpu.registers.b = 0x14;
  setup.cpu.registers.pc = 0x00;

  setup.cpu.run_instruction(&mut setup.memory);

  assert_eq!(setup.cpu.registers.a, 0x14);
}

#[test]
fn test_ld_nn_r() {
  let mut setup = Setup::new();
  let nn = 0xCDAB;

  setup.memory.write(0x0000, 0xEA);
  setup.memory.write(0x0001, 0xAB);
  setup.memory.write(0x0002, 0xCD);

  setup.cpu.registers.a = 0x13;
  setup.cpu.registers.pc = 0x00;

  setup.cpu.run_instruction(&mut setup.memory);

  assert_eq!(setup.memory.read(nn), 0x13);
}

#[test]
fn test_ret() {
  let mut setup = Setup::new();

  setup.memory.write(0x0000, 0xC9);
  setup.memory.write(0xFFF2, 0x31);
  setup.memory.write(0xFFF3, 0x13);

  setup.cpu.registers.pc = 0;
  setup.cpu.registers.sp = 0xFFF2;

  setup.cpu.run_instruction(&mut setup.memory);

  assert_eq!(setup.cpu.registers.pc, 0x1331);
}

#[test]
fn test_xor_n() {
  let mut setup = Setup::new();

  setup.memory.write(0x0000, 0xA8);
  setup.cpu.registers.a = 0x13;
  setup.cpu.registers.b = 0x19;

  setup.cpu.registers.pc = 0;

  setup.cpu.run_instruction(&mut setup.memory);

  assert_eq!(setup.cpu.registers.a, 0xA);
}

#[test]
fn test_call() {
  let mut setup = Setup::new();

  setup.memory.write(0x0000, 0xCD);
  setup.memory.write(0x0001, 0xAB);
  setup.memory.write(0x0002, 0xEF);

  setup.cpu.registers.pc = 0;
  setup.cpu.registers.sp = 0xFFFE;

  setup.cpu.run_instruction(&mut setup.memory);

  assert_eq!(setup.cpu.registers.pc, 0xEFAB);
}

#[test]
fn test_ccf() {
  let mut setup = Setup::new();

  setup.memory.write(0x0000, 0x3F);
  setup.memory.write(0x0001, 0x3F);

  setup.cpu.registers.pc = 0;
  setup.cpu.registers.unset_c_flag();

  setup.cpu.run_instruction(&mut setup.memory);
  assert_eq!(setup.cpu.registers.f, 0b_0001_0000);

  setup.cpu.run_instruction(&mut setup.memory);
  assert_eq!(setup.cpu.registers.f, 0b_0000_0000);
}

#[test]
fn test_jr_e() {
  let mut setup = Setup::new();

  setup.memory.write(0x0010, 0x18);
  setup.memory.write(0x0011, 0x13);

  setup.cpu.registers.pc = 0x0010;

  setup.cpu.run_instruction(&mut setup.memory);

  assert_eq!(setup.cpu.registers.pc, 0x25);
}

#[test]
fn test_ld_hl_n() {
  let mut setup = Setup::new();

  setup.memory.write(0x0000, 0x36);
  setup.memory.write(0x0001, 0x19);

  setup.cpu.registers.pc = 0;
  setup.cpu.registers.h = 0x13;
  setup.cpu.registers.l = 0x31;

  setup.cpu.run_instruction(&mut setup.memory);

  assert_eq!(setup.memory.read(setup.cpu.registers.get_pair(RegisterPair::HL)), 0x19);
}

#[test]
fn test_get_half_carry() {
  let mut setup = Setup::new();

  let mut prev = 0x0F;
  let mut result = 0x10;
  let mut half_carry = setup.cpu.get_half_carry(prev, result);

  assert!(half_carry);

  prev = 0x00;
  result = 0x01;
  half_carry = setup.cpu.get_half_carry(prev, result);

  assert_eq!(half_carry, false);
}
