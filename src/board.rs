use core::sync::atomic::{AtomicBool, Ordering};

use crate::hal::stm32;

pub struct Board {}

pub struct BoardImpl<CorePereipheral> {
    core_peripherals: CorePereipheral,
}

static IS_CONSTRAINED: AtomicBool = AtomicBool::new(false);

impl Board {
    pub fn constrain() -> Option<BoardImpl<stm32::Peripherals>> {
        if IS_CONSTRAINED.load(Ordering::Relaxed) {
            None
        } else {
            IS_CONSTRAINED.store(true, Ordering::Relaxed);
            Some(BoardImpl {
                core_peripherals: stm32::Peripherals::take()
                    .expect("could not get core peripherals"),
            })
        }
    }
}

impl<CorePereipheral> BoardImpl<CorePereipheral> {
    pub fn release(self) {
        IS_CONSTRAINED.store(false, Ordering::Relaxed);
    }
}

impl BoardImpl<stm32::Peripherals> {
    pub fn core_peripherals(self) -> (BoardImpl<()>, stm32::Peripherals) {
        (
            BoardImpl {
                core_peripherals: (),
            },
            self.core_peripherals,
        )
    }
}
