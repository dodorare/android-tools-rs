mod emulator_enum;
mod emulator_tools;

use crate::error::*;
use std::{path::PathBuf, process::Command};

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
    let sdk_path = {
        let sdk_path = std::env::var("ANDROID_SDK_ROOT")
            .ok()
            .or_else(|| std::env::var("ANDROID_SDK_PATH").ok())
            .or_else(|| std::env::var("ANDROID_HOME").ok());
        PathBuf::from(sdk_path.ok_or(AndroidError::AndroidSdkNotFound)?)
    };
    let build_tools = sdk_path.join("emulator");
    let emulator = build_tools.join(bin!("emulator"));
    Ok(Command::new(emulator))
}
