#![no_main]
#![no_std]
use cortex_m_rt::entry;
extern crate panic_halt;
extern crate stm32f3xx_hal;

/// This program is just used for testing the working with the micro-controller, and to debug the program.
/// This #[entry] is the entry point of the program, the point where the program halt or begin from.
#[entry]
fn main() -> ! {

    let  x = 42;
    let  a=100;
    //using _ in front of just means i am telling the compiler to ignore the unused warning of z.
    let  _z= x+a;

    // infinite loop; just so we don't leave this stack frame
    loop {}
}
