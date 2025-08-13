use cpu::cpu::Cpu;
use memory::memory::Memory;

const INTERRUPT_FLAG_ADDRESS: u16 = 0xFF0F;
const TIMA_ADDRESS: u16 = 0xFF05;
const TMA_ADDRESS: u16 = 0xFF06;

pub fn handle_interrupt(cpu: &mut Cpu, memory: &mut Memory, interrupt_enable: u8, interrupt_flag: u8) {
  let sp = cpu.registers.sp;
  let pc = cpu.registers.pc.to_le_bytes();

  // Push PC to stack
  memory.write(sp - 1, pc[1]);
  memory.write(sp - 2, pc[0]);
  cpu.registers.sp -= 2;

  // VBlank
  if ((interrupt_enable & 0b_0000_0001) == 0b_0000_0001) & ((interrupt_flag & 0b_0000_0001) == 0b_0000_0001) {
    // TODO
    memory.write(INTERRUPT_FLAG_ADDRESS, interrupt_flag & 0b_1111_1110);
  }

  // LCD
  if ((interrupt_enable & 0b_0000_0010) == 0b_0000_0010) & ((interrupt_flag & 0b_0000_0010) == 0b_0000_0010) {
    // TODO
    memory.write(INTERRUPT_FLAG_ADDRESS, interrupt_flag & 0b_1111_1101);
  }

  // Timer handling
  if ((interrupt_enable & 0b_0000_0100) == 0b_0000_0100) & ((interrupt_flag & 0b_0000_0100) == 0b_0000_0100) {
    cpu.registers.tima = cpu.registers.tma;
    memory.write(TIMA_ADDRESS, memory.read(TMA_ADDRESS));

    memory.write(INTERRUPT_FLAG_ADDRESS, interrupt_flag & 0b_1111_1011);
  }

  // Serial
  if ((interrupt_enable & 0b_0000_1000) == 0b_0000_1000) & ((interrupt_flag & 0b_0000_1000) == 0b_0000_1000) {
    // TODO
    memory.write(INTERRUPT_FLAG_ADDRESS, interrupt_flag & 0b_1111_0111);
  }

  // Joypad
  if ((interrupt_enable & 0b_0001_0000) == 0b_0001_0000) & ((interrupt_flag & 0b_0001_0000) == 0b_0001_0000) {
    // TODO
    memory.write(INTERRUPT_FLAG_ADDRESS, interrupt_flag & 0b_1110_1111);
  }

  cpu.cycles += 5; // The entire process lasts 5 M-cycles

  cpu.ret(memory);
}