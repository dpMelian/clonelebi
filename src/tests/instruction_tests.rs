use cpu::instructions::CycleTable;
use cpu::instructions::Optable;
use cpu::registers::Registers;
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
