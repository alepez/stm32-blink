#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate panic_halt;
extern crate stm32f0;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use stm32f0xx_hal::delay::Delay;
use stm32f0xx_hal::prelude::*;
use stm32f0xx_hal::stm32;

// use `main` as the entry point of this application
#[entry]
fn main() -> ! {
    let mut peripherals = stm32::Peripherals::take().unwrap();
    let cp = Peripherals::take().unwrap();

    cortex_m::interrupt::free(|cs| {
        let mut rcc = peripherals
            .RCC
            .configure()
            .sysclk(8.mhz())
            .freeze(&mut peripherals.FLASH);

        let gpioc = peripherals.GPIOC.split(&mut rcc);

        let mut delay = Delay::new(cp.SYST, &rcc);

        let mut orange = gpioc.pc8.into_push_pull_output(cs);

        loop {
            orange.toggle();
            delay.delay_ms(50_u16);
        }
    })
}
