pub struct Registers {
  pub a: u8, // Accumulator
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub f: u8, // Flags
  pub h: u8,
  pub l: u8,
  pub sp: u16, // Stack Pointer
  pub pc: u16, // Program Counter/Pointer
}

pub enum RegisterPairs {
  AF,
  BC,
  DE,
  HL,
}

impl Registers {
  pub fn new() -> Self {
    Self { a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0, sp: 0, pc: 0 }
  }

  pub fn get_pair(&self, case: RegisterPairs) -> u16 {
    match case {
      RegisterPairs::AF => (self.a as u16 * 256) + self.f as u16,
      RegisterPairs::BC => (self.b as u16 * 256) + self.c as u16,
      RegisterPairs::DE => (self.d as u16 * 256) + self.e as u16,
      RegisterPairs::HL => (self.h as u16 * 256) + self.l as u16,
    }
  }

  pub fn set_pair(&mut self, case: RegisterPairs, value: u16) {
    let split_u8_values = value.to_be_bytes();
    
    match case {
      RegisterPairs::AF => {
        self.a = split_u8_values[0];
        self.f = split_u8_values[1];
      },
      RegisterPairs::BC => {
        self.b = split_u8_values[0];
        self.c = split_u8_values[1];
      },
      RegisterPairs::DE => {
        self.d = split_u8_values[0];
        self.e = split_u8_values[1];
      },
      RegisterPairs::HL => {
        self.h = split_u8_values[0];
        self.l = split_u8_values[1];
      },
    }
  }
}
