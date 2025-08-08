pub fn get_half_carry(a: u8, b: u8) -> bool {
  if (((a & 0xF).wrapping_add(b & 0xF)) & 0x10) == 0x10 {
    true
  } else {
    false
  }
}

pub fn get_half_carry_sub(a: u8, b: u8) -> bool {
  if (((a & 0xF).wrapping_sub(b & 0xF)) & 0x10) == 0x10 {
    true
  } else {
    false
  }
}

pub fn get_half_carry_16_bit(a: u16, b: u16) -> bool {
  if (((a & 0xFFF).wrapping_add(b & 0xFFF)) & 0x1000) == 0x1000 {
    true
  } else {
    false
  }
}

pub fn get_carry(a: u8, b: u8) -> bool {
  if (((a as u16 & 0xFF).wrapping_add(b as u16 & 0xFF)) & 0x100) == 0x100 {
    true
  } else {
    false
  }
}

pub fn get_carry_sub(a: u8, b: u8) -> bool {
  if (((a as u16 & 0xFF).wrapping_sub(b as u16 & 0xFF)) & 0x100) == 0x100 {
    true
  } else {
    false
  }
}

pub fn get_carry_16_bit(a: u16, b: u16) -> bool {
  if (((a as u32 & 0xFFFF).wrapping_add(b as u32 & 0xFFFF)) & 0x10000) == 0x10000 {
    true
  } else {
    false
  }
}