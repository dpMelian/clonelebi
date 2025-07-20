mod cpu;
use cpu::registers::Registers;
use cpu::registers::RegisterPairs;

fn main() {
  let mut cpu = cpu::cpu::CPU {
    registers: Registers::new(),
  };

  cpu.registers.set_pair( RegisterPairs::AF, 0x1901);

  dbg!(cpu.registers.get_pair(RegisterPairs::AF));
}
