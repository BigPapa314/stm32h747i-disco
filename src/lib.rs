#![no_std]
#![allow(non_camel_case_types)]

pub extern crate stm32h7xx_hal as hal;

extern crate cortex_m;
extern crate cortex_m_rt;

pub use crate::hal::stm32::interrupt::*;
pub use crate::hal::stm32::*;
pub use crate::hal::*;
pub use cortex_m::*;
pub use cortex_m_rt::*;

use core::sync::atomic::{AtomicUsize, Ordering};

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

pub fn timestamp() -> u64 {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n as u64
}
