#![no_std]
#![no_main]
/// This program is simply blink the led's of the stm32_discovery board with delay's in between the led's.
use cortex_m::asm::delay;
use cortex_m_rt::entry;
use hal::prelude::*;
use hal::stm32f30x;
use panic_halt as _;
use stm32f30x_hal as hal;

#[entry]
fn main() -> ! {
    //setting up device peripheral to access the led's pin.
    let device_peripheral = stm32f30x::Peripherals::take().unwrap();
    let mut reset_clock_control = device_peripheral.RCC.constrain();
    // splits the GPIOE peripheral into 16 independent pins + registers
    let mut gpioe = device_peripheral.GPIOE.split(&mut reset_clock_control.ahb);
    // configure the pin PE9 as an output
    //MODER register is 32 bit register for configuring function of each pin.
    //OTYPER register specifies how the output pin is connected
    // (push_pull(set high is only available on such configuration) or open drain)
    let mut led_3 = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_4 = gpioe
        .pe8
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    loop {
        {
            led_3.set_high();
            delay(3_000_000_u32);
            led_3.set_low();
            delay(3_000_000_u32);
            led_4.set_high();
            delay(3_000_000_u32);
            led_4.set_low();
            delay(3_000_000_u32);
        }
    }
}
