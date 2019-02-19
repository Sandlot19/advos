use crate::{print, println};
use crate::console::Console;
use crate::global_constants::{CLOCK_FREQ, CORE_LOCAL_INTERRUPT_MAP};
use core::fmt::{Error, Write};
use core::ptr::{write_volatile};

const CTX_PER_SECOND    : u64 = 1;
const TIME_TO_CTX_SWITCH: u64 = CLOCK_FREQ / CTX_PER_SECOND;

const MTIME_CMP_LO: u64 = CORE_LOCAL_INTERRUPT_MAP + 0x4000;
const MTIME_CMP_HI: u64 = CORE_LOCAL_INTERRUPT_MAP + 0x4004;
const MTIME_LO    : u64 = CORE_LOCAL_INTERRUPT_MAP + 0xBFF8;
const MTIME_HI    : u64 = CORE_LOCAL_INTERRUPT_MAP + 0xBFFC;

#[no_mangle]
pub extern "C" fn handle_trap(mcause: u32, mepc: u32) -> u32 {
  println!("0x{:x}", mcause);

  if (mcause >> 31) == 1 {
    // TODO: HandleInterrupt(mcause);
    println!("got an interrupt");
  }
  else {
    // TODO: HandleException(mcause);
    println!("got an exception");
  }

  // Clear the CLIM to indicate we've handled the interrupt
  let clim = CORE_LOCAL_INTERRUPT_MAP as *mut u32;
  unsafe { write_volatile(clim, 0); }
  println!("trap handled, returning");
  let mepc_ptr = mepc as *mut u32;
  let next_instruction: u32;
  unsafe {
    next_instruction = read_volatile(mepc_ptr);
  }

  // Compressed instructions are 2 bytes, while uncompressed are 4 bytes.
  // If the lowest 2 bits of the instruction are 0b00, then the instruction is
  // uncompressed, and if anything else, then the instruction is compressed, so
  // we can then determine how much to increment mepc by to return to the
  // correct instruction after the trap has been handled.
  if (next_instruction & 0x3) != 0 {
    mepc + 2
  }
  else {
    mepc + 4
  }

}

pub fn init_context_timer() -> Result<(), Error> {
  let cmp_lo_addr = MTIME_CMP_LO as *mut u32;
  let cmp_hi_addr = MTIME_CMP_HI as *mut u32;

  unsafe {
    write_volatile(cmp_lo_addr, (TIME_TO_CTX_SWITCH & 0xFFFFFFFF) as u32);
    write_volatile(cmp_hi_addr, ((TIME_TO_CTX_SWITCH >> 32) & 0xFFFFFFFF) as u32);
  }

  Ok(())
}
