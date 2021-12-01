//! Contains `Error` type and `CommandExt` impl used by `android-tools-rs`.

use displaydoc::Display;
use std::process::Command;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Command '{0:?}' had a non-zero exit code. Stdout: {1} Stderr: {2}
    CmdFailed(Command, String, String),
    /// Bundletool is not found
    BundletoolNotFound,
    /// Command execution failed error: {0}
    CommandFailed(String),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::CommandFailed(error.to_string())
    }
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
                self,
                String::from_utf8_lossy(&output.stdout).to_string(),
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        Ok(output)
    }
}
