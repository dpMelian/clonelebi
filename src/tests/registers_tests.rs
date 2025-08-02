use cpu::registers::Registers;

struct Setup {
  registers: Registers
}

impl Setup {
  pub fn new() -> Self {
    Self {
      registers: Registers::new(),
    }
  }
}

#[test]
fn test_get_z_flag() {
  let mut setup = Setup::new();

  setup.registers.f = 0b_1000_0000;
  assert!(setup.registers.get_z_flag());

  setup.registers.f = 0b_0000_0000;
  assert_eq!(setup.registers.get_z_flag(), false);
}

#[test]
fn test_get_n_flag() {
  let mut setup = Setup::new();

  setup.registers.f = 0b_0100_0000;
  assert!(setup.registers.get_n_flag());

  setup.registers.f = 0b_0000_0000;
  assert_eq!(setup.registers.get_n_flag(), false);
}

#[test]
fn test_get_h_flag() {
  let mut setup = Setup::new();

  setup.registers.f = 0b_0010_0000;
  assert!(setup.registers.get_h_flag());

  setup.registers.f = 0b_0000_0000;
  assert_eq!(setup.registers.get_h_flag(), false);
}

#[test]
fn test_get_c_flag() {
  let mut setup = Setup::new();

  setup.registers.f = 0b_0001_0000;
  assert!(setup.registers.get_c_flag());

  setup.registers.f = 0b_0000_0000;
  assert_eq!(setup.registers.get_c_flag(), false);
}
