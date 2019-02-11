#![feature(panic_info_message,allocator_api,asm,lang_items,compiler_builtins_lib)]
//We are not permitted to use the standard library since it isn't written for
//our operating system
#![no_std]
#![no_mangle]
#![allow(dead_code,unused_variables)]

mod console;
mod trap;

use console::Console;
use core::fmt::Write;
use core::ptr::{write_volatile};

#[macro_export]
macro_rules! print {
    ($fmt:expr) => {
        write!(Console, $fmt).unwrap();
    };
    ($fmt:expr, $($args:tt)*) => {
        write!(Console, "{}", format_args!($fmt, $($args)*)).unwrap();
    };
}

#[macro_export]
macro_rules! println {
    () => ( print!("\n") );
    ($fmt:expr) => { print!(concat!($fmt, "\n")); };
    ($fmt:expr, $($args:tt)*) => {
        print!("{}", format_args!(concat!($fmt, "\r\n"), $($args)*))
    };
}

//The eh_personality tells our program how to unwind. We aren't going to write
//that, so tell it to do nothing.
#[lang = "eh_personality"]
pub extern fn eh_personality() {}

//Abort will be used when panic can't
#[no_mangle]
fn abort() -> !
{
   loop {}
}

//Panic handler will execute whenever our rust code panics. -> ! means that this
//function won't return, so we have to make sure it doesn't.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    abort()
}

const CORE_LOCAL_INTERRUPT_MAP: u64 = 0x0200_0000;

#[no_mangle]
fn main() {
    // Intialize UART for reading/writing
    console::uart::init().unwrap();

    // Test lines for formatting with println!

    println!();
    println!("Test lines: ");
    println!("  Lowercase Hex: 15 = {:x}", 15);
    println!("  Uppercase Hex: 26 = {:X}", 26);
    println!("  Named References: for hello=7, reference hello yields {hello}", hello=7);
    println!("  Octal: 12 = {:o}", 12);
    println!("  Formatted Double: 1.23456 of width 3 is {:.3}", 1.23456);
    println!("  Formatted Int: 42 of width 4 with leading zeroes is {:04}", 42);
    println!();

    let clim = CORE_LOCAL_INTERRUPT_MAP as *mut u32;
    let interrupt_mask: u32 = 0x008;

    println!("sending software interrupt");
    unsafe { write_volatile(clim, interrupt_mask); }

    loop {
        if let Some(s) = console::Console::read() {
            print!("\r\nread \"");
            for c in s.iter() {
                print!("{}", c);
            }
            println!("\" from uart");
        }
    }
}
