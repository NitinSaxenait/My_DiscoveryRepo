#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_itm as _;
use stm32::{GPIOE, RCC};
use stm32f3_discovery::stm32f3xx_hal::stm32;

#[entry]
fn main() -> ! {
    // Taking RegisterBlocks of GPIOE port and RCC as reference using "&*".
    //
    // Using "&*" bcz GPIO and RCC are const -> Raw pointer, therefore need to use reference after
    // dereference.
    let (gpio_port_e, rcc) = unsafe { (&*GPIOE::ptr(), &*RCC::ptr()) };

    // This step is taking access of gpioe port.
    //
    // -> iopeen -? "in/out port enable".
    rcc.ahbenr.write(|write| write.iopeen().set_bit());

    // Here we are changing LEDs mode from (by default) input to output.
    // moder.write -> reset and write the value.
    // moder.modify -> first read then write the value.
    gpio_port_e
        .moder
        .modify(|_read, write| write.moder8().output());
    gpio_port_e.moder.modify(|_read, write| write.moder9().output());
    gpio_port_e
        .moder
        .modify(|_read, write| write.moder10().output());
    gpio_port_e
        .moder
        .modify(|_read, write| write.moder11().output());
    gpio_port_e
        .moder
        .modify(|_read, write| write.moder12().output());
    gpio_port_e
        .moder
        .modify(|_read, write| write.moder13().output());
    gpio_port_e
        .moder
        .modify(|_read, write| write.moder14().output());
    gpio_port_e.moder.modify(|_read,write| write.moder15().output());

    // Here we are providing the value as 1 to the odr register to "on" led.
    gpio_port_e.odr.write(|write| {
        write.odr8().set_bit();
        write.odr9().set_bit();
        write.odr10().set_bit();
        write.odr11().set_bit();
        write.odr12().set_bit();
        write.odr13().set_bit();
        write.odr14().set_bit();
        write.odr15().set_bit()
    });

    loop {}
}
