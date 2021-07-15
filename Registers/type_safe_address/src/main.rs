#![no_main]
#![no_std]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use panic_itm as _;
use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::stm32f3xx_hal::{pac::GPIOE, prelude::*, stm32};

#[entry]
fn main() -> ! {
    let gpioe = unsafe { &*GPIOE::ptr() };
    let device_peripherals = stm32::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_peripherals.RCC.constrain();

    // initialize user leds
    let mut gpio_pe = device_peripherals
        .GPIOE
        .split(&mut reset_and_clock_control.ahb);
    let _leds = Leds::new(
        gpio_pe.pe8,
        gpio_pe.pe9,
        gpio_pe.pe10,
        gpio_pe.pe11,
        gpio_pe.pe12,
        gpio_pe.pe13,
        gpio_pe.pe14,
        gpio_pe.pe15,
        &mut gpio_pe.moder,
        &mut gpio_pe.otyper,
    );

    loop {
        // Turn "on" the North LED
        gpioe.bsrr.write(|write| write.bs9().set_bit());
        delay(3_0000_00);
        // Turn "on" the North LED
        gpioe.bsrr.write(|write| write.bs11().set_bit());
        delay(3_0000_00);
        // Turn "off" the North LED
        gpioe.bsrr.write(|write| write.br9().set_bit());
        delay(3_0000_00);
        // Turn "off" the North LED
        gpioe.bsrr.write(|write| write.br11().set_bit());
        delay(3_0000_00);
    }
}
