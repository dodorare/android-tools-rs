//! Contains `Error` type and `CommandExt` impl used by `android-tools-rs`.

use displaydoc::Display;
use std::{path::PathBuf, process::Command};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Android specific error type
#[derive(Display, Debug, Error)]
pub enum AndroidError {
    /// Android SDK or Android NDK is not found.
    AndroidToolIsNotFound,
    /// Android SDK is not found
    AndroidSdkNotFound,
    /// Android NDK is not found
    AndroidNdkNotFound,
    /// Android SDK has no build tools
    BuildToolsNotFound,
    /// Android SDK has no platforms installed
    NoPlatformsFound,
    /// Failed to create directory
    DirectoryWasNotCreated,
    /// Platform {0} is not installed
    PlatformNotFound(u32),
    /// Target is not supported
    UnsupportedTarget,
    /// Host {0} is not supported
    UnsupportedHost(String),
    /// Invalid semver
    InvalidSemver,
    /// Unsupported or invalid target: {0}
    InvalidBuildTarget(String),
    /// Failed to find AndroidManifest.xml in path: {0}
    FailedToFindAndroidManifest(String),
    /// Unable to find NDK file
    UnableToFindNDKFile,
}

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Path {0:?} doesn't exist
    PathNotFound(PathBuf),
    /// Command {0} not found
    CmdNotFound(String),
    /// Command had a non-zero exit code. Stdout: {0} Stderr: {1}
    CmdFailed(String, String),
    /// Bundletool is not found. Please, use crossbundle install command to setup bundletool
    BundletoolNotFound,
    /// Compiled resources not found
    CompiledResourcesNotFound,
    /// IO error
    Io(#[from] std::io::Error),
    /// IO error
    AndroidError(#[from] AndroidError),
}

/// Extension trait for [`Command`] that helps
/// to wrap output and print logs from command execution.
///
/// [`Command`](std::process::Command)
pub trait CommandExt {
    /// Executes the command as a child process, then captures an output and return it.
    /// If command termination wasn't successful wraps an output into error and return it
    fn output_err(self, print_logs: bool) -> Result<std::process::Output>;
}

impl CommandExt for Command {
    fn output_err(mut self, print_logs: bool) -> Result<std::process::Output> {
        // Enables log print during command execution
        let output = match print_logs {
            true => self.spawn().and_then(|p| p.wait_with_output())?,
            false => self.output()?,
        };
        if !output.status.success() {
            return Err(Error::CmdFailed(
                String::from_utf8_lossy(&output.stdout).to_string(),
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        Ok(output)
    }
}
