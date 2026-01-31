pub fn get_half_carry(a: u8, b: u8) -> bool {
  if (((a & 0xF).wrapping_add(b & 0xF)) & 0x10) == 0x10 {
    true
  } else {
    false
  }
}

// TODO: use only this function instead of get_half_carry
pub fn get_half_carry_refactor(initial_value: u8, values: &[u8]) -> bool {
  let mut sum = initial_value & 0xF;

  for value in values {
    sum = sum + (value & 0xF);
  }

  if sum > 0xF {
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
pub fn get_half_carry_sub_refactor(initial_value: u8, values: &[u8]) -> bool {
  let mut sum = 0;

  for value in values {
    sum = sum + (value & 0xF);
  }

  if (initial_value & 0xF) < sum {
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

// TODO: use only this function instead of get_carry
pub fn get_carry_refactor(initial_value: u8, values: &[u8]) -> bool {
  let mut sum: u16 = initial_value as u16 & 0xFF;

  for value in values {
    sum = sum + (*value as u16 & 0xFF);
  }

  if sum > 0xFF {
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

// TODO: use only this function instead of get_carry_sub
pub fn get_carry_sub_refactor(initial_value: u8, values: &[u8]) -> bool {
  let mut sum: u16 = 0;

  for value in values {
    sum = sum + (*value as u16 & 0xFF);
  }

  if (initial_value as u16 & 0xFF) < sum {
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