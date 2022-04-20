#![no_main]
#![no_std]

use stm32h747i_disco as bsp;
use stm32h747i_disco::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut board = Board::initialize(100.MHz()).unwrap();

    defmt::println!(" ");
    defmt::println!("stm32h7xx-hal example - Blinky");
    defmt::println!(" ");

    board.leds.green.set_high();
    board.leds.orange.set_high();
    board.leds.red.set_high();
    board.leds.blue.set_high();

    board.delay(1000.millis());

    board.leds.green.set_low();
    board.delay(500.millis());
    board.leds.orange.set_low();
    board.delay(500.millis());
    board.leds.red.set_low();
    board.delay(500.millis());
    board.leds.blue.set_low();
    board.delay(500.millis());

    board.leds.green.set_high();
    board.delay(500.millis());
    board.leds.orange.set_high();
    board.delay(500.millis());
    board.leds.red.set_high();
    board.delay(500.millis());
    board.leds.blue.set_high();
    board.delay(500.millis());

    // board.release();

    bsp::exit();
}
