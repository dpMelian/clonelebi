pub struct Memory {
  memory: [u8; 65536] // 64 KiB of memory
}

impl Memory {
  pub fn new() -> Self {
    Self { memory: [0; 65536] }
  }

  pub fn read(&self, address: u16) -> u8 {
    // Hard-coded for GameBoy Doctor
    if address == 0xFF44 {
      self.memory[0x90];
    }
    self.memory[address as usize]
  }

  pub fn write(&mut self, address: u16, value: u8) {
    self.memory[address as usize] = value;
  }

  pub fn load_rom_into_memory(&mut self, rom: &[u8]) {
    let max_rom_length = 0x8000;
    let safe_rom_length = rom.len().min(max_rom_length);
    self.memory[0x0000..0x0000 + safe_rom_length].copy_from_slice(&rom[..safe_rom_length]);
  }

  pub fn dump(&mut self, start: u16, end: u16) {
    let mut current_address = start;
    while current_address <= end {
      print!("0x{:04X}: ", current_address);

      for i in 0..16 {
        print!("{:02X} ", self.read(current_address + i));
      }

      println!();

      current_address += 16;
    }
  }
}
