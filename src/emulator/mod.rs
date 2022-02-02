mod emulator_enum;
mod emulator_tools;

pub use emulator_enum::*;
pub use emulator_tools::*;

#[derive(Clone, Copy)]
pub struct Emulator;

impl Emulator {
    pub fn emulator(self) -> EmulatorTools {
        EmulatorTools::new()
    }
}
