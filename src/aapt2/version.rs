use super::aapt2_tool;
use crate::error::*;

/// Prints the version of aapt.
pub struct Aapt2Version {
    version: String,
    help: bool,
}

impl Aapt2Version {
    /// Prints the version of aapt2
    pub fn new(version: String) -> Self {
        Self {
            version,
            help: false,
        }
    }

    /// Displays this help menu
    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// Executes aapt2 version with arguments
    pub fn run(&self) -> Result<()> {
        let mut aapt2 = aapt2_tool()?;
        aapt2.arg("version");
        aapt2.arg(&self.version);
        if self.help {
            aapt2.arg("-h");
        }
        aapt2.output_err(true)?;
        Ok(())
    }
}
