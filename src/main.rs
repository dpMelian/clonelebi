mod cpu;
mod memory;
mod tests;

use std::fs;

use cpu::cpu::Cpu;
use memory::memory::Memory;

fn load_rom_file() -> Vec<u8> {
  let rom = fs::read("roms/tetris.gb")
    .expect("Should have been able to read the file");

  if rom.is_empty() {
    panic!("Could not load ROM");
  }

  rom
}

fn main() {
  let rom = load_rom_file();

  let mut cpu: Cpu = Cpu::new();
  let mut memory: Memory = Memory::new();

  memory.load_rom_into_memory(&rom);

  // Power up setup
  cpu.registers.pc = 0x0100;
  cpu.registers.sp = 0xFFFE;

  while cpu.registers.pc <= 0x8000 {
    cpu.run_instruction(&mut memory);
    println!("{:X}", cpu.registers.pc);
  }
}
