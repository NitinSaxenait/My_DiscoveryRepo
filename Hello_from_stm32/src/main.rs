#![no_main]
#![no_std]


use cortex_m_rt::entry;
use cortex_m::iprintln;
use panic_itm as _;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;

#[entry()]
fn main() -> !{

    let peripherals = cortex_m::Peripherals::take().unwrap();
    let mut itm = peripherals.ITM;


    iprintln!(&mut itm.stim[0],"Hello Discovery!");

    loop {

    }
}