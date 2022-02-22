use crate::error::*;
use std::process::Command;

#[derive(Clone, Default)]
pub struct AdbShellAmSetDebugApp {
    persistent: bool,
    w: bool,
}

impl AdbShellAmSetDebugApp {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Retain this value
    pub fn persistent(&mut self, persistent: bool) -> &mut Self {
        self.persistent = persistent;
        self
    }

    /// Wait for debugger when app starts
    pub fn w(&mut self, w: bool) -> &mut Self {
        self.w = w;
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut set_debug_app = Command::new("adb");
        set_debug_app.arg("shell");
        set_debug_app.arg("am");
        set_debug_app.arg("set-debug-app");
        if self.persistent {
            set_debug_app.arg("--persistent");
        }
        if self.w {
            set_debug_app.arg("-w");
        }
        set_debug_app.output_err(true)?;
        Ok(())
    }
}
