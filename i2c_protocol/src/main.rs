#![no_main]
#![no_std]
use cortex_m::{asm::delay, iprintln};
use cortex_m_rt::entry;
use panic_itm as _;
use stm32::I2C1;
use stm32f3_discovery::lsm303dlhc::Lsm303dlhc;
use stm32f3_discovery::stm32f3xx_hal::{i2c::I2c, prelude::*, stm32};

#[entry()]
fn main() -> ! {
    // Slave address
    const MAGNETOMETER: u16 = 0b0011_1100;
    // Addresses of the magnetometer's registers
    const OUTX_L_REG_M: u8 = 0x068;

    let device_peripheral = stm32::Peripherals::take().unwrap();
    let core_peripheral = cortex_m::Peripherals::take().unwrap();
    let mut rcc = device_peripheral.RCC.constrain();
    let mut flash = device_peripheral.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpiob = device_peripheral.GPIOB.split(&mut rcc.ahb);
    // gpiob.afrl(alternate function register low)
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let mut itm = core_peripheral.ITM;
    let i2c = I2c::new(
        device_peripheral.I2C1,
        (scl, sda),
        400.khz(),
        clocks,
        &mut rcc.apb1,
    );
    Lsm303dlhc::new(i2c).unwrap();
    let i2c_rb = unsafe { &*I2C1::ptr() };
    // Broadcast START and the address of Magnetometer
    loop {
        i2c_rb.cr2.write(|write| {
            write.start().set_bit();
            // Here we have to send the address(of mag_register).
            write.sadd().bits(MAGNETOMETER);
            // After those 7 bits of address we need to send the mode for read and write with slave.
            // clear_bit -> Master will perform send/"write" operation.
            // set_bit -> Maaster will perform receive/"read" operation.
            write.rd_wrn().clear_bit();
            // The number of bytes to be transmitted/received is programmed there.
            write.nbytes().bits(1);
            // this condition sets the tc(transfer complete) flag to tell that now transfer is completed and software
            // provide the autoend to the transfer using clear_bit.
            write.autoend().clear_bit()
        });

        while i2c_rb.isr.read().txis().bit_is_clear() {}
        // Here we are sending the address of IRA__ REGISTER using "transfer_data_register".
        i2c_rb.txdr.write(|w| w.txdata().bits(OUTX_L_REG_M));
        // we will wait un-till the the transfer get completed.
        // isr- interrupt-status-register read the value of tc(register)-> transfer complete.
        // This flag is set by hardware when RELOAD=0, AUTO-END=0 and N-BYTES data have been
        // transferred.
        while i2c_rb.isr.read().tc().bit_is_clear() {}

        i2c_rb.cr2.modify(|_read, write| {
            write.start().set_bit();
            write.rd_wrn().set_bit();
            write.nbytes().bits(6);
            // Automatic end mode: a STOP condition is automatically sent when N-BYTES data are
            // transferred.
            // This condition will set the "stop" to the communication after the data is read.
            write.autoend().set_bit()
        });
        // Taken an array of type u8 and size 6
        // Now we are taking mutable reference(so that we can work on that particular address) of
        // each value of the array.
        // Now we are inserting  the reading to each of the referenced address of the array's value.
        // And using * to deference ourselves from that address to work with other.
        // And now the array values are updated with the (register)value.
        let mut buffer = [0, 1, 2, 3, 4, 5];
        for count_byte in &mut buffer {
            // we need to wait un-till we receive the register content.
            // This bit is set by hardware when the received data is copied into the I2C_RXDR register,
            // and is ready to be read. It is cleared(bit_is_clear) when I2C_RXDR is read.
            while i2c_rb.isr.read().rxne().bit_is_clear() {}
            // Now the whole data is inside the rxdr register and now we will read that content from
            // this rxdr register.
            *count_byte = i2c_rb.rxdr.read().rxdata().bits();
        }
        // Broadcast STOP (automatic because of `AUTOEND = 1`)

        iprintln!(&mut itm.stim[0], "{:?}", buffer);

        delay(10_00_000);
    }
}
