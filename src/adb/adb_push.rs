use crate::error::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Default)]
pub struct AdbPush {
    sync: Option<PathBuf>,
    n: bool,
    z_enable_compression: bool,
    z_disable_compression: bool,
}

impl AdbPush {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Only push files that are newer on the host than the device
    pub fn sync(&mut self, sync: &Path) -> &mut Self {
        self.sync = Some(sync.to_owned());
        self
    }

    /// Dry run: push files to device without storing to the filesystem
    pub fn n(&mut self, n: bool) -> &mut Self {
        self.n = n;
        self
    }

    /// Enable compression with a specified algorithm (any, none, brotli)
    pub fn z_enable_compression(&mut self, z_enable_compression: bool) -> &mut Self {
        self.z_enable_compression = z_enable_compression;
        self
    }

    /// Disable compression
    pub fn z_disable_compression(&mut self, z_disable_compression: bool) -> &mut Self {
        self.z_disable_compression = z_disable_compression;
        self
    }

    /// Runs `adb push` commands
    pub fn run(&self) -> Result<()> {
        let mut adb_push = Command::new("adb");
        adb_push.arg("push");
        if let Some(sync) = &self.sync {
            adb_push.arg("--sync").arg(sync);
        }
        if self.n {
            adb_push.arg("-n");
        }
        if self.z_enable_compression {
            adb_push.arg("-z");
        }
        if self.z_disable_compression {
            adb_push.arg("-Z");
        }
        adb_push.output_err(true)?;
        Ok(())
    }
}
