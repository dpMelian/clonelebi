use cpu::cpu::Cpu;
use memory::memory::Memory;

const INTERRUPT_FLAG_ADDRESS: u16 = 0xFF0F;

pub fn handle_interrupt(cpu: &mut Cpu, memory: &mut Memory, interrupt_enable: u8, interrupt_flag: u8) {
  let sp = cpu.registers.sp;
  let pc = cpu.registers.pc.to_le_bytes();
  cpu.cycles += 2; // Two wait states are executed

  // Push PC to stack
  memory.write(sp - 1, pc[1]);
  memory.write(sp - 2, pc[0]);
  cpu.registers.sp -= 2;

  // VBlank
  if ((interrupt_enable & 0b_0000_0001) == 0b_0000_0001) & ((interrupt_flag & 0b_0000_0001) == 0b_0000_0001) {

    memory.write(INTERRUPT_FLAG_ADDRESS, interrupt_flag & 0b_1111_1110);
  }

  // Timer handling
  if ((interrupt_enable & 0b_0000_0100) == 0b_0000_0100) & ((interrupt_flag & 0b_0000_0100) == 0b_0000_0100) {
    cpu.registers.timer_counter = cpu.registers.timer_modulo;
    memory.write(0xFF05, memory.read(0xFF06));

    memory.write(INTERRUPT_FLAG_ADDRESS, interrupt_flag & 0b_1111_1011);
  }

  cpu.ret(memory);
}