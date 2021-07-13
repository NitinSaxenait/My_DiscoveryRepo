#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_itm as _;
use stm32::{RCC, TIM6};
use stm32f3_discovery::stm32f3xx_hal::{prelude::*, stm32};
use stm32f3_discovery::{leds::Leds, switch_hal::OutputSwitch};

fn delay(ms: u16) {
    let tim6 = unsafe { &*TIM6::ptr() };
    // arr register is the auto reload register which will provide the endValue to the counter.
    // After reaching to the arr value the counter stops and an event will occur. Then again the
    // counter starts from 0 to arr value.
    tim6.arr.write(|write| write.arr().bits(ms));
    // After providing the arr endValue we start the counter by enabling it. So that it starts
    // its work.
    tim6.cr1.modify(|_read, write| write.cen().set_bit());
    // We will wait untill the counter reaches its arr value and after that overflow event will occur.
    // then we will set the status register's uif(update interrupt flag) as 1
    while !tim6.sr.read().uif().bit_is_set() {}
    // Now we have to clear the uif bit as if we don't then in the next delay it will think that
    // update event has already been done and we will skip the busy waiting part.
    tim6.sr.modify(|_read, write| write.uif().clear_bit());
}

#[entry()]
fn main() -> ! {
    let device_peripheral = stm32::Peripherals::take().unwrap();
    let mut reset_cc = device_peripheral.RCC.constrain();
    let mut gpioe = device_peripheral.GPIOE.split(&mut reset_cc.ahb);
    // Access the hardware leds
    let _leds = Leds::new(
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
    // taking the  leds-array, rcc- RegisterBlock, time6- timer6 of apb1enr clock.
    let (mut leds, rcc, time6) = (_leds.into_array(), unsafe { &*RCC::ptr() }, unsafe {
        &*TIM6::ptr()
    });
    // Here we are going to implement the delay using hardware and for that we need to set  time6
    // peripheral of the hardware which is in apb1enr clock.
    rcc.apb1enr.modify(|_read, write| write.tim6en().set_bit());
    // time6 timer will provide this cr(ControlRegister) and now we need to write on this cr to set
    // the counter in one pulse mode which will make it work like an alarm clock.
    //
    // opm -> In this mode counter will stop after the event occur & CEN (CounterEnable gets clear)
    // Now i am going to enable the counter in opm and will not set the counter to work.
    time6.cr1.write(|write| write.opm().set_bit().cen().clear_bit());
    // psc register is used here to manage the counting of the counter.
    // psc divide or skip the counter clock frequency counts in 1 sec.
    // Inserting bits means value in (ms) in the psc register.
    // The counter clock can be divided by a psc-prescaler .
    time6.psc.write(|write| write.psc().bits(7_999));

    let ms = 1000;
    loop {
        for previous in 0..8 {
            let next = (previous + 1) % 8;

            leds[next].on().unwrap();
            delay(ms);
            leds[previous].off().unwrap();
            delay(ms);
        }
    }
}
