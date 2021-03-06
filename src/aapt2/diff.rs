use super::aapt2_tool;
use crate::error::*;
use std::path::PathBuf;

/// Prints the differences in resources of two apks.
pub struct Aapt2Diff {
    input_apks: Vec<PathBuf>,
    help: bool,
}

impl Aapt2Diff {
    /// Initialize aapt2 diff and then specifies paths to input apks
    pub fn new(input_apks: &[PathBuf]) -> Self {
        Self {
            input_apks: input_apks.to_vec(),
            help: false,
        }
    }

    /// Displays this help menu
    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// Executes aapt2 diff with arguments
    pub fn run(&self) -> Result<()> {
        let mut aapt2 = aapt2_tool()?;
        aapt2.arg("diff");
        self.input_apks.iter().for_each(|input_apks| {
            aapt2.arg(input_apks);
        });
        if self.help {
            aapt2.arg("-h");
        }
        aapt2.output_err(true)?;
        Ok(())
    }
}
