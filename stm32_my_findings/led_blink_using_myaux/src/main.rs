#![no_std]
#![no_main]
/// This is the main program which take mycrate function from my_auxillary.
///
/// function mycrate contains the implementation to cover the led challenge of discovery book.
use my_auxillary::*;
/// This is starting point of the no_main program.
///
/// This program is going to blink all led's of stm32f3 discovery board with delay of 1 sec between
/// each blink.
#[entry]
fn main() -> ! {
    let mut counter = 0;
    let (mut led, mut delay) = my_auxillary::mycrate();

    loop {
        while counter < counter + 1 {
            let next = counter % 8;
            led[next].on().ok();
            delay.delay_ms(1000_u16);
            led[next].off().ok();
            counter = counter + 1;
        }
    }
}
