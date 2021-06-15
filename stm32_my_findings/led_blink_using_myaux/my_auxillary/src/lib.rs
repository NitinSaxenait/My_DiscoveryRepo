#![no_std]
/// This program is lib for main program which provide the implementation to blink leds in stm32f3 board.
pub use  panic_itm;
pub use cortex_m_rt::entry;
pub use stm32f3_discovery::{
    stm32f3xx_hal,
    leds::Leds,
    switch_hal::{ActiveHigh, OutputSwitch, Switch},
};
pub use stm32f3xx_hal::{
    gpio::{gpioe, Output, PushPull},
    prelude::*,
    stm32,
};
/// LedArray is used to provide the leds an array structure.
pub type  LedArray= [Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>; 8];
pub fn mycrate() -> LedArray {
    // setting up device peripheral without the delay.
    let device_peripheral = stm32::Peripherals::take().unwrap();
    let mut reset_clock_control = device_peripheral.RCC.constrain();
    // splits the GPIOE peripheral into 16 independent pins
    let mut gpioe = device_peripheral.GPIOE.split(&mut reset_clock_control.ahb);
    //taking access to all leds pins in variable leds.
    let leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );
    leds.into_array()
}
