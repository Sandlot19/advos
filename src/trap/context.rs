use crate::{print, println};
use crate::console::Console;

// A struct to hold the current context
pub struct Context {
  registers_: [u32;32],
  program_counter_: u32,
};

impl Context {
  fn new(regs: [u32;32], pc: u32) -> Context {
  }

  // Creates a new context object based on the current context and returns it
  pub fn GetCurrentContext() -> Context {
  }
}
