use std::path::{Path, PathBuf};
use crate::error::*;
use super::AdbPush;

#[derive(Clone, Default)]
pub struct AdbTools {
    install: Option<PathBuf>,
    forward: Option<String>,
    pull: Option<PathBuf>,
    push: Option<AdbPush>,
    help: bool,
    shell: bool,
    d: bool,
    e: bool,
    s: bool,
    a: bool,
    H: bool,
    P: bool,
    L: bool,
}

impl AdbTools {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Only push files that are newer on the host than the device
    pub fn sync(&mut self, push: &AdbPush) -> &mut Self {
        self.push = Some(push.to_owned());
        self
    }

    pub fn run(&self) -> Result<PathBuf> {
        
    }
}