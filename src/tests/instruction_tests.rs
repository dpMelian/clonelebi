use cpu::instructions::Optable;
use cpu::registers::Registers;
use cpu::cpu::Cpu;
use memory::memory::Memory;


#[test]
fn test_cpl() {
  let mut cpu = Cpu {
    registers: Registers::new(),
    optable: Optable::new(),
  };

  let mut memory = Memory::new();
  memory.write(0x0000, 0x2F);

  cpu.registers.a = 0x13;
  cpu.registers.pc = 0x00;

  cpu.run_instruction(&mut memory);

  assert_eq!(cpu.registers.a, 0xEC);
}

#[test]
fn test_inc() {
  let mut cpu = Cpu {
    registers: Registers::new(),
    optable: Optable::new(),
  };

  let mut memory = Memory::new();
  memory.write(0x0000, 0x3C);

  cpu.registers.a = 0x13;
  cpu.registers.pc = 0x00;

  cpu.run_instruction(&mut memory);

  assert_eq!(cpu.registers.a, 0x14);
}
