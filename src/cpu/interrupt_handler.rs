use cpu::cpu::Cpu;
use memory::memory::Memory;

pub fn handle_interrupt(cpu: &mut Cpu, memory: &mut Memory, interrupt_enable: u8, interrupt_flag: u8) {
  println!("Handling interrupt...");
  cpu.cycles += 2; // Two wait states are executed

  // VBlank
  if ((interrupt_enable & 0b_0000_0001) == 0b_0000_0001) & ((interrupt_flag & 0b_0000_0001) == 0b_0000_0001) {
    println!("VBlank...");

    memory.write(0xFF0F, interrupt_flag & 0b_1111_1110);
  }

  // Timer handling
  if ((interrupt_enable & 0b_0000_0100) == 0b_0000_0100) & ((interrupt_flag & 0b_0000_0100) == 0b_0000_0100) {
    println!("Timer...");
    cpu.registers.timer_counter = cpu.registers.timer_modulo;
    memory.write(0xFF05, memory.read(0xFF06));

    memory.write(0xFF0F, interrupt_flag & 0b_1111_1011);
  }
}