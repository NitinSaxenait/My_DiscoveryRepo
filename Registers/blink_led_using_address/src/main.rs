#![no_main]
#![no_std]
use core::ptr;
/// This program is going to turn on the led's of the board using the address of the register only.
use cortex_m_rt::entry;
use panic_itm as _;
use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::stm32f3xx_hal::{prelude::*, stm32};

#[entry()]
fn main() -> ! {
    // Taking peripheral "gpio and device" in control.
    let device_controls = stm32::Peripherals::take().unwrap();
    let mut reset_cc = device_controls.RCC.constrain();
    let mut gpioe = device_controls.GPIOE.split(&mut reset_cc.ahb);
    let _take_leds_access = Leds::new(
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
    // This is the "BSRR" register address which is taken constant here.
    //
    // Now to write the values on the const register we used "Raw Pointer" as
    // GPIO_BSRR_REG_ADDRESS as *mut u32 and deference the address using *.
    const GPIO_BSRR_REG_ADDRESS: u32 = 0x48001018;
    // De-referencing of Raw Pointer is done under "unsafe block" as per RP rules.
    unsafe {
        *(GPIO_BSRR_REG_ADDRESS as *mut u32) = 1 << 9;
        *(GPIO_BSRR_REG_ADDRESS as *mut u32) = 1 << 11;
        *(GPIO_BSRR_REG_ADDRESS as *mut u32) = 1 << 10;
        *(GPIO_BSRR_REG_ADDRESS as *mut u32) = 1 << 9 + 16;
        *(GPIO_BSRR_REG_ADDRESS as *mut u32) = 1 << 11 + 16;
        *(GPIO_BSRR_REG_ADDRESS as *mut u32) = 1 << 10 + 16;
    }

    // When we build our program in release mode then out program get compact. Release mode
    // provides only single str(write) on the register therefore.

    //Our debug (unoptimized) program had four of them, one for each write to the register,
    // but the release (optimized) program only has one.

    // Uncomment this unsafe and comment the above one and build the program using release mode.
    /*
    unsafe {
        // A magic address!
        const GPIO_BSRR_REG_ADDRESS: u32 = 0x48001018;

        // Turn on red
        //we prevent LLVM from misoptimizing our program using volatile operations.
        ptr::write_volatile(GPIO_BSRR_REG_ADDRESS as *mut u32, 1 << 9);
        // Turn on green
        ptr::write_volatile(GPIO_BSRR_REG_ADDRESS as *mut u32, 1 << 11);
        // Turn on orange
        ptr::write_volatile(GPIO_BSRR_REG_ADDRESS as *mut u32, 1 << 1 << 10);
        // Turn off red
        ptr::write_volatile(GPIO_BSRR_REG_ADDRESS as *mut u32, 1 << 9 + 16);
        // Turn off green
        ptr::write_volatile(GPIO_BSRR_REG_ADDRESS as *mut u32, 1 << 11 + 16);
        // Turn off orange
        ptr::write_volatile(GPIO_BSRR_REG_ADDRESS as *mut u32, 1 << 10 + 16);
    }

     */

    loop {}
}
