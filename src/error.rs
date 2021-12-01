//! Contains `Error` type and `CommandExt` impl used by `android-tools-rs`.

use displaydoc::Display;
use std::process::Command;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Command had a non-zero exit code. Stdout: {0} Stderr: {1}
    CmdFailed(String, String),
    /// Bundletool is not found
    BundletoolNotFound,
    /// Compiled resources not found
    CompiledResourcesNotFound,
    /// IO error
    Io(#[from] std::io::Error),
}

/// Extension trait for [`Command`] that helps
/// to wrap output and print logs from command execution.
///
/// [`Command`]: std::process::Command
pub trait CommandExt {
    /// Executes the command as a child process, then captures an output and return it.
    /// If command termination wasn't successful wraps an output into error and return it.
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
