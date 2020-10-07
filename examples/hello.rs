#![no_main]
#![no_std]

use stm32h747i_disco as board;

use defmt_rtt as _; // global logger
use panic_probe as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world1!");
    defmt::info!("Hello, world2!");

    board::exit();
}

#[defmt::timestamp]
fn timestamp() -> u64 {
    board::timestamp()
}
