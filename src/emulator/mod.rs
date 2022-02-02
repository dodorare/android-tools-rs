mod emulator;
mod emulator_enum;

pub use emulator::*;
pub use emulator_enum::*;

#[derive(Clone, Copy)]
pub struct Emulator;

impl Emulator {
    pub fn emulator(self) -> EmulatorTools {
        EmulatorTools::new()
    }
}
