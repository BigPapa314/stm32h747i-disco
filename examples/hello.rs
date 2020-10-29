#![no_main]
#![no_std]

use bsp::Board;
use stm32h747i_disco as bsp;

use defmt_rtt as _; // global logger
use panic_probe as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = Board::constrain().expect("could not get the board!");

    defmt::info!("Hello, world1!");
    defmt::info!("Hello, world2!");

    board.release();

    bsp::exit();
}

#[defmt::timestamp]
fn timestamp() -> u64 {
    bsp::timestamp()
}
