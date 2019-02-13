// hifive1 freq: 17,422,745
// qemu freq: 65,000,000
//
// transmit control register (txctrl) offset 0x008
// bit 0: txen: Transmit enable
// recieve control register (rxctrl) offset 0x00C
// bit 0: rxen: Recieve enable
//
// 1. Set divisor for baud rate
// 2. Enable transmission
// 3. Enable recieve
//
// Then make readchar -> u8
// 1. Read the register (use readvolatile)
// 2. Check if bit 31 is set
use core::ptr::{read_volatile, write_volatile};
use core::fmt::Error;


pub const CLOCK_FREQ: u64 = 17_422_745; // Hz
const BAUD_RATE: u64 = 115_200;
const DIVISOR: u64 = (CLOCK_FREQ / BAUD_RATE) - 1;

const UART_ADDR: u64 = 0x1001_3000;
const TXDATA: u64 = UART_ADDR + 0x000; // Transmit data register
const RXDATA: u64 = UART_ADDR + 0x004; // Recieve data register
const TXCTRL: u64 = UART_ADDR + 0x008; // Transmit control register
const RXCTRL: u64 = UART_ADDR + 0x00c; // Recieve control register
const IE:     u64 = UART_ADDR + 0x010; // UART interrupt enable
const IP:     u64 = UART_ADDR + 0x014; // UART interrupt pending
const DIV:    u64 = UART_ADDR + 0x018; // Baud rate divisor

pub fn init() -> Result<(), Error> {
    let div = DIV as *mut u32;
    let txctrl = TXCTRL as *mut u32;
    let rxctrl = RXCTRL as *mut u32;
    unsafe {
        write_volatile(div, DIVISOR as u32 & 0x0000_FFFF);
        write_volatile(txctrl, read_volatile(txctrl) | 1);
        write_volatile(rxctrl, read_volatile(rxctrl) | 1);
    }

    Ok(())
}

pub fn readchar() -> Option<u8> {
    let rxdata = RXDATA as *mut u32;
    let r: u32;
    unsafe {
        r = read_volatile(rxdata);
    }
    if r >> 31 == 1 { None }
    else { Some(r as u8) }
}

pub fn writechar(byte: u8) -> () {
    let txdata = TXDATA as *mut u32;
    unsafe {
        let mut t: u32;
        // Block until the write FIFO has space
        loop {
            t = read_volatile(txdata);
            if t >> 31 == 0 { break; }
        }
        write_volatile(txdata, (t & 0x0000) | byte as u32);
    }
}
