use crate::error::*;
use std::process::Command;

#[derive(Clone, Default)]
pub struct AdbPull {
    a: bool,
    z: bool,
    disable_compression: bool,
}

impl AdbPull {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Preserve file timestamp and mode
    pub fn a(&mut self, a: bool) -> &mut Self {
        self.a = a;
        self
    }

    /// Enable compression with a specified algorithm (any, none, brotli)
    pub fn z(&mut self, z: bool) -> &mut Self {
        self.z = z;
        self
    }

    /// Disable compression
    pub fn disable_compression(&mut self, disable_compression: bool) -> &mut Self {
        self.disable_compression = disable_compression;
        self
    }

    /// Runs `adb pull` commands
    pub fn run(&self) -> Result<()> {
        let mut pull = Command::new("adb");
        pull.arg("pull");
        if self.a {
            pull.arg("-a");
        }
        if self.z {
            pull.arg("-z");
        }
        if self.disable_compression {
            pull.arg("-Z");
        }
        Ok(())
    }
}
