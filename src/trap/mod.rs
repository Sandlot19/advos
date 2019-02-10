use crate::{print, println};
use crate::console::Console;
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn handle_trap(cause: u32) -> () {
    println!("mcause = 0x{:x}", cause);
    unsafe { asm!("csrrs zero, mip, zero" ::::"volatile"); asm!("mret" ::::"volatile"); }
    //println!("here");
    return;
}
