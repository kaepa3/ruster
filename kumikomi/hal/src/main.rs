#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::debug;
use stm32f4xx_hal::{delay::Delay, prelude::*, stm32};

#[entry]
fn main() -> ! {
    if let Some(dp) = stm32::Peripherals::take() {
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
        let gploc = dp.GPIOC.split();

        let config = Config::default()
            .baudrate(115200.bps())
            .wordlength_8()
            .parity_none()
            .stopbits(StopBits::Stop1);

        let mut pins = (
            gploc.pc10.into_alternate_af8(),
            gploc.pc11.into_alternate_af8(),
        );

        let mut uart4 = Serial::uart4(dp.UART4, pins, config, clocks).unwrap();

        let _ = uart4.write(b'H');
        let _ = uart4.write(b'e');
        let _ = uart4.write(b'l');
        let _ = uart4.write(b'l');
        let _ = uart4.write(b'o');
    }
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
