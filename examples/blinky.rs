#![no_main]
#![no_std]

use bsp::Board;
use stm32h747i_disco as bsp;

use defmt_rtt as _; // global logger
use panic_probe as _;

use stm32h7xx_hal::hal::digital::v2::OutputPin;
use stm32h7xx_hal::{pac, prelude::*};

use defmt::info;

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = Board::constrain().expect("could not get the board!");

    let (board, dp) = board.core_peripherals();

    let cp = cortex_m::Peripherals::take().unwrap();
    //let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    info!("Setup PWR...                  ");
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.smps().freeze();

    // Constrain and Freeze clock
    info!("Setup RCC...                  ");
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.mhz()).freeze(pwrcfg, &dp.SYSCFG);

    info!(" ");
    info!("stm32h7xx-hal example - Blinky");
    info!(" ");

    let gpioi = dp.GPIOI.split(ccdr.peripheral.GPIOI);

    // Configure PE1 as output.
    let mut green = gpioi.pi12.into_push_pull_output();
    let mut orange = gpioi.pi13.into_push_pull_output();
    let mut red = gpioi.pi14.into_push_pull_output();
    let mut blue = gpioi.pi15.into_push_pull_output();

    green.set_high().unwrap();
    orange.set_high().unwrap();
    red.set_high().unwrap();
    blue.set_high().unwrap();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    delay.delay_ms(1000_u16);

    green.set_low().unwrap();
    delay.delay_ms(500_u16);
    orange.set_low().unwrap();
    delay.delay_ms(500_u16);
    red.set_low().unwrap();
    delay.delay_ms(500_u16);
    blue.set_low().unwrap();
    delay.delay_ms(500_u16);

    green.set_high().unwrap();
    delay.delay_ms(500_u16);
    orange.set_high().unwrap();
    delay.delay_ms(500_u16);
    red.set_high().unwrap();
    delay.delay_ms(500_u16);
    blue.set_high().unwrap();
    delay.delay_ms(500_u16);

    board.release();

    bsp::exit();
}

#[defmt::timestamp]
fn timestamp() -> u64 {
    bsp::timestamp()
}
