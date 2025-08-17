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

// TODO: use only this function instead of get_half_carry_sub
pub fn get_half_carry_sub_refactor(initial_value: u16, values: &[u16]) -> bool {
  let mut result = initial_value & 0xF;

  for value in values {
    result = result - (value & 0xF);
  }

  if result & 0x10 == 0x10 {
    true
  } else {
    false
  }
}

pub fn get_half_carry_16_bit_high(a: u16, b: u16) -> bool {
  if (((a & 0xFFF).wrapping_add(b & 0xFFF)) & 0x1000) == 0x1000 {
    true
  } else {
    false
  }
}

pub fn get_half_carry_16_bit_low(a: u16, b: u16, c: u16) -> bool {
  if (((a & 0xF) + (b & 0xF)) + (c & 0xF) & 0x10) == 0x10 {
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

pub fn get_carry_16_bit_high(a: u16, b: u16) -> bool {
  if (((a as u32 & 0xFFFF).wrapping_add(b as u32 & 0xFFFF)) & 0x10000) == 0x10000 {
    true
  } else {
    false
  }
}

pub fn get_carry_16_bit_low(a: u16, b: u16, c: u16) -> bool {
  if (((a & 0xFF) + (b & 0xFF)) + (c & 0xFF) & 0x100) == 0x100 {
    true
  } else {
    false
  }
}