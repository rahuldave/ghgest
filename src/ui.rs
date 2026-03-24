pub mod colors;
pub mod components;
pub mod theme;
pub mod utils;

pub fn init() {
  yansi::whenever(yansi::Condition::TTY_AND_COLOR);
}
