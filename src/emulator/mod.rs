mod emulator_enum;
mod emulator_tools;

use crate::{error::*, sdk_path_from_env};
use std::process::Command;

pub use emulator_enum::*;
pub use emulator_tools::*;

#[derive(Clone, Copy)]
pub struct Emulator;

impl Emulator {
    pub fn emulator(self) -> EmulatorTools {
        EmulatorTools::new()
    }
}

pub fn emulator_tool() -> Result<Command> {
    if let Ok(emulator_tools) = which::which(bin!("emulator")) {
        return Ok(Command::new(emulator_tools));
    }
    let sdk_path = sdk_path_from_env().unwrap();
    let build_tools = sdk_path.join("emulator");
    let emulator = build_tools.join(bin!("emulator"));
    Ok(Command::new(emulator))
}
