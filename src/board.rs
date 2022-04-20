use stm32h7xx_hal::delay;
use stm32h7xx_hal::gpio;
use stm32h7xx_hal::prelude::*;
use stm32h7xx_hal::time::Hertz;
use stm32h7xx_hal::time::MilliSeconds;

pub struct Board {
    pub leds: Leds,
    delay: delay::Delay,
}

pub struct Leds {
    pub green: gpio::Pin<'I', 12_u8, gpio::Output<gpio::PushPull>>,
    pub orange: gpio::Pin<'I', 13_u8, gpio::Output<gpio::PushPull>>,
    pub red: gpio::Pin<'I', 14_u8, gpio::Output<gpio::PushPull>>,
    pub blue: gpio::Pin<'I', 15_u8, gpio::Output<gpio::PushPull>>,
}

impl Board {
    pub fn initialize(frequency: Hertz) -> Option<Board> {
        let cortex_peripherals = cortex_m::Peripherals::take()?;
        let stm32h7_peripherals = stm32h7::stm32h747cm7::Peripherals::take()?;

        defmt::println!("Setup PWR...");
        let pwr = stm32h7_peripherals.PWR.constrain();

        let pwrcfg = pwr.smps().freeze();

        // Constrain and Freeze clock
        defmt::println!("Setup RCC...");
        let rcc = stm32h7_peripherals.RCC.constrain();
        let ccdr = rcc
            .sys_ck(frequency)
            .freeze(pwrcfg, &stm32h7_peripherals.SYSCFG);

        let gpioi = stm32h7_peripherals.GPIOI.split(ccdr.peripheral.GPIOI);

        Some(Board {
            leds: Leds {
                green: gpioi.pi12.into_push_pull_output(),
                orange: gpioi.pi13.into_push_pull_output(),
                red: gpioi.pi14.into_push_pull_output(),
                blue: gpioi.pi15.into_push_pull_output(),
            },
            delay: cortex_peripherals.SYST.delay(ccdr.clocks),
        })
    }

    pub fn delay(&mut self, time: MilliSeconds) {
        self.delay.delay_ms(time.to_millis());
    }
}
