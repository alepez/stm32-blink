#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use stm32f0xx_hal::delay::Delay;
use stm32f0xx_hal::prelude::*;
use stm32f0xx_hal::stm32;

struct Led<'a> {
    pin: &'a mut dyn embedded_hal::digital::v1::ToggleableOutputPin,
}

impl Led<'_> {
    fn new(pin: &mut embedded_hal::digital::v1::ToggleableOutputPin) -> Led {
        Led { pin }
    }

    fn toggle(&mut self) {
        self.pin.toggle();
    }
}

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
        let mut green = gpioc.pc9.into_push_pull_output(cs);
        let mut red = gpioc.pc6.into_push_pull_output(cs);
        let mut blue = gpioc.pc7.into_push_pull_output(cs);

        let mut leds: [Led; 4] = [
            Led::new(&mut orange),
            Led::new(&mut green),
            Led::new(&mut red),
            Led::new(&mut blue),
        ];

        let mut i = 0;

        loop {
            leds[i % 4].toggle();
            i += 1;
            delay.delay_ms(50_u16);
        }
    })
}
