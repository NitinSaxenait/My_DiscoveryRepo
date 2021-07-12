#![no_main]
#![no_std]
use core::ptr;
/// This program is going to read an INVALID ADDRESS using Raw Pointer.
use cortex_m_rt::entry;
use panic_itm as _;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;


#[entry()]
fn main() -> ! {
    unsafe {
        ptr::read_volatile(0x4800_1800 as *const u32);
    }

    loop {}
}
