use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RegisterU8 {
  A,
  B,
  C,
  D,
  E,
  F,
  H,
  L,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RegisterU16 {
  SP,
  PC,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Flag {
  Z,
  N,
  H,
  C
}

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

impl Index<RegisterU8> for Registers {
  type Output = u8;

  fn index(&self, register: RegisterU8) -> &Self::Output {
    match register {
      RegisterU8::A => &self.a,
      RegisterU8::B => &self.b,
      RegisterU8::C => &self.c,
      RegisterU8::D => &self.d,
      RegisterU8::E => &self.e,
      RegisterU8::F => &self.f,
      RegisterU8::H => &self.h,
      RegisterU8::L => &self.l,
    }
  }
}

impl Index<RegisterU16> for Registers {
  type Output = u16;

  fn index(&self, register: RegisterU16) -> &Self::Output {
    match register {
      RegisterU16::SP => &self.sp,
      RegisterU16::PC => &self.pc
    }
  }
}

impl IndexMut<RegisterU8> for Registers {
  fn index_mut(&mut self, register: RegisterU8) -> &mut Self::Output {
    match register {
      RegisterU8::A => &mut self.a,
      RegisterU8::B => &mut self.b,
      RegisterU8::C => &mut self.c,
      RegisterU8::D => &mut self.d,
      RegisterU8::E => &mut self.e,
      RegisterU8::F => &mut self.f,
      RegisterU8::H => &mut self.h,
      RegisterU8::L => &mut self.l,
    }
  }
}

impl IndexMut<RegisterU16> for Registers {
  fn index_mut(&mut self, register: RegisterU16) -> &mut Self::Output {
    match register {
      RegisterU16::SP => &mut self.sp,
      RegisterU16::PC => &mut self.pc
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RegisterPair {
  AF,
  BC,
  DE,
  HL,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Target {
  Pair(RegisterPair),
  SingleU16(RegisterU16),
  SingleU8(RegisterU8)
}

impl Registers {
  pub fn new() -> Self {
    Self { a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0, sp: 0, pc: 0 }
  }

  pub fn get_pair(&self, case: RegisterPair) -> u16 {
    match case {
      RegisterPair::AF => (self.a as u16 * 256) + self.f as u16,
      RegisterPair::BC => (self.b as u16 * 256) + self.c as u16,
      RegisterPair::DE => (self.d as u16 * 256) + self.e as u16,
      RegisterPair::HL => (self.h as u16 * 256) + self.l as u16,
    }
  }

  pub fn set_pair(&mut self, case: RegisterPair, value: u16) {
    let split_u8_values = value.to_be_bytes();
    
    match case {
      RegisterPair::AF => {
        self.a = split_u8_values[0];
        self.f = split_u8_values[1];
      },
      RegisterPair::BC => {
        self.b = split_u8_values[0];
        self.c = split_u8_values[1];
      },
      RegisterPair::DE => {
        self.d = split_u8_values[0];
        self.e = split_u8_values[1];
      },
      RegisterPair::HL => {
        self.h = split_u8_values[0];
        self.l = split_u8_values[1];
      },
    }
  }

  pub fn get_z_flag(&self) -> bool {
    let flag = self.f;

    let mask = 0b1000_0000u8;

    ((flag & mask) >> 7) != 0
  }

  pub fn get_n_flag(&self) -> bool {
    let flag = self.f;

    let mask = 0b0100_0000u8;

    ((flag & mask) >> 6) != 0
  }

  pub fn get_h_flag(&self) -> bool {
    let flag = self.f;

    let mask = 0b0010_0000u8;

    ((flag & mask) >> 5) != 0
  }

  pub fn get_c_flag(&self) -> bool {
    let flag = self.f;

    let mask = 0b0001_0000u8;

    ((flag & mask) >> 4) != 0
  }

  pub fn set_z_flag(&mut self) {
    self.f = Self::set_bit(self.f, 7, true);
  }

  pub fn set_n_flag(&mut self) {
    self.f = Self::set_bit(self.f, 6, true);
  }

  pub fn set_h_flag(&mut self) {
    self.f = Self::set_bit(self.f, 5, true);
  }

  pub fn set_c_flag(&mut self) {
    self.f = Self::set_bit(self.f, 4, true);
  }

  pub fn unset_z_flag(&mut self) {
    self.f = Self::set_bit(self.f, 7, false);
  }

  pub fn unset_n_flag(&mut self) {
    self.f = Self::set_bit(self.f, 6, false);
  }

  pub fn unset_h_flag(&mut self) {
    self.f = Self::set_bit(self.f, 5, false);
  }

  pub fn unset_c_flag(&mut self) {
    self.f = Self::set_bit(self.f, 4, false);
  }

  pub fn set_bit(x: u8, idx: usize, b: bool) -> u8 {
    let flag = 1 << idx;
    if b {
        x | flag
    } else {
        x & !flag
    }
  }
}
