mod emulator_enum;
mod emulator_tools;

use crate::error::*;
use std::{process::Command, path::PathBuf};

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
    let target_sdk_version = std::fs::read_dir(&build_tools)
        .map_err(|_| Error::PathNotFound(build_tools.clone()))?
        .filter_map(|path| path.ok())
        .filter(|path| path.path().is_dir())
        .filter_map(|path| path.file_name().into_string().ok())
        .filter(|name| name.chars().next().unwrap().is_digit(10))
        .max()
        .ok_or(AndroidError::BuildToolsNotFound)?;
    let emulator = build_tools.join(target_sdk_version).join(bin!("emulator"));
    Ok(Command::new(emulator))
}