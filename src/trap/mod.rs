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
  let interrupt_flag: u32 = mcause >> 31;
  let exception_code: u32 = mcause & 0x1F;

  if interrupt_flag == 1 {
    // TODO: HandleInterrupt(mcause);
    println!("Got an interrupt");
  }
  else {
    // TODO: HandleException(mcause);
    println!("Got an exception");
  }

  println!("Code: 0x{:x}", exception_code);

  // Match the flag and code to see what happened
  match (interrupt_flag, exception_code) {
    (1, 0)  => println!("User software interrupt"),
    (1, 1)  => println!("Supervisor software interrupt"),
    (1, 3)  => println!("Machine software interrupt"),
    (1, 4)  => println!("User timer interrupt"),
    (1, 5)  => println!("Supervisor timer interrupt"),
    (1, 7)  => println!("Machine timer interrupt"),
    (1, 8)  => println!("User external interrupt"),
    (1, 9)  => println!("Supervisor external interrupt"),
    (1, 11) => println!("Machine external interrupt"),
    (0, 0)  => println!("Instruction address misaligned"),
    (0, 1)  => println!("Instruction access fault"),
    (0, 2)  => println!("Illegal instruction"),
    (0, 3)  => println!("Breakpoint"),
    (0, 4)  => println!("Load address misaligned"),
    (0, 5)  => println!("Load access fault"),
    (0, 6)  => println!("Store/AMO address misaligned"),
    (0, 7)  => println!("Store/AMO access fault"),
    (0, 8)  => println!("Environment call from U-mode"),
    (0, 9)  => println!("Environment call from S-mode"),
    (0, 11) => println!("Environment call from M-mode"),
    (0, 12) => println!("Instruction page fault"),
    (0, 13) => println!("Load page fault"),
    (0, 15) => println!("Store/AMO page fault"),
    (_, _)  => println!("Reserved/unknown code (THIS SHOULD NEVER HAPPEN)"),
  }

  // Clear the CLIM to indicate we've handled the interrupt
  let clim = CORE_LOCAL_INTERRUPT_MAP as *mut u32;
  unsafe { write_volatile(clim, 0); }
  println!("trap handled, returning");

  return mepc;
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
