use crate::{print, println, CORE_LOCAL_INTERRUPT_MAP};
use crate::console::Console;
use core::fmt::Write;
use core::ptr::{write_volatile};

#[no_mangle]
pub fn handle_trap(cause: u32, mepc: u32) -> u32 {
    println!("mcause = 0x{:x} mepc = 0x{:x}", cause, mepc);

    // Clear the CLIM to indicate we've handled the interrupt
    let clim = CORE_LOCAL_INTERRUPT_MAP as *mut u32;
    unsafe { write_volatile(clim, 0); }
    println!("trap handled, returning");

    return mepc;
}
