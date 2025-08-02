extern crate minifb;
mod cpu;
mod memory;
mod tests;
mod ppu;

use std::fs::{self, OpenOptions};
use std::io::Result;
use std::io::Write;

use cpu::cpu::Cpu;
use memory::memory::Memory;

fn load_rom_file() -> Vec<u8> {
  let rom = fs::read("roms/06-ld r,r.gb")
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

  // Initial setup
  cpu.registers.a = 0x01;
  cpu.registers.f = 0xB0;
  cpu.registers.b = 0x00;
  cpu.registers.c = 0x13;
  cpu.registers.d = 0x00;
  cpu.registers.e = 0xD8;
  cpu.registers.h = 0x01;
  cpu.registers.l = 0x4D;
  cpu.registers.sp = 0xFFFE;
  cpu.registers.pc = 0x0100;

  let serial_output_address = 0xFF02;
  let mut serial_output_value = memory.read(serial_output_address);

  while serial_output_value != 0x81 {
    serial_output_value = memory.read(serial_output_address);

    println!("PC: {:X}", cpu.registers.pc);

    let contents = format!("A:{A:02X} F:{F:02X} B:{B:02X} C:{C:02X} D:{D:02X} E:{E:02X} H:{H:02X} L:{L:02X} SP:{SP:04X} PC:{PC:04X} PCMEM:{PCMEM0:02X},{PCMEM1:02X},{PCMEM2:02X},{PCMEM3:02X}\n",
      A = cpu.registers.a,
      F = cpu.registers.f,
      B = cpu.registers.b,
      C = cpu.registers.c,
      D = cpu.registers.d,
      E = cpu.registers.e,
      H = cpu.registers.h,
      L = cpu.registers.l,
      SP = cpu.registers.sp,
      PC = cpu.registers.pc,
      PCMEM0 = memory.read(cpu.registers.pc),
      PCMEM1 = memory.read(cpu.registers.pc + 1),
      PCMEM2 = memory.read(cpu.registers.pc + 2),
      PCMEM3 = memory.read(cpu.registers.pc + 3),
    );

    match append_to_file("./logs/cpu_log.txt", &contents) {
        Ok(_) => {},
        Err(e) => println!("Error appending to file: {}", e),
    }

    cpu.run_instruction(&mut memory);
  }

  if serial_output_value == 0x81 {
    dbg!(memory.read(serial_output_address));
  }
}

fn append_to_file(file_path: &str, contents: &str) -> Result<()> {
  let mut file = OpenOptions::new()
    .create(true)
    .write(true)
    .append(true)
    .open(file_path)?;

  file.write_all(contents.as_bytes())
}
