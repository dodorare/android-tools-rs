use crate::error::*;
use std::process::Command;

/// List all test packages.
#[derive(Clone, Default)]
pub struct AdbShellPmListInstrumentation {
    f: bool,
    target_package: bool,
}

impl AdbShellPmListInstrumentation {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// List the APK file for the test package.
    pub fn f(&mut self, f: bool) -> &mut Self {
        self.f = f;
        self
    }

    /// List test packages for only this app.
    pub fn target_package(&mut self, target_package: bool) -> &mut Self {
        self.f = target_package;
        self
    }

    /// Runs `adb shell pm list instrumentation` commands.
    pub fn run(&self) -> Result<()> {
        let mut list_instrumentation = Command::new("adb");
        list_instrumentation.arg("shell");
        list_instrumentation.arg("pm");
        list_instrumentation.arg("list");
        list_instrumentation.arg("instrumentation");
        if self.f {
            list_instrumentation.arg("-f");
        }
        if self.target_package {
            list_instrumentation.arg("target-package");
        }
        Ok(())
    }
}
