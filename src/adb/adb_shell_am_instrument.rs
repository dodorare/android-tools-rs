use crate::error::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Default)]
pub struct AdbShellAmInstrument {
    e: Option<Vec<String>>,
    w: bool,
    no_window_animation: bool,
    p: Option<PathBuf>,
    r: bool,
    user_id: Option<String>,
}

impl AdbShellAmInstrument {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    ///  Set argument name to value. For test runners a common form is -e testrunner_flag
    ///  value[,value...]
    pub fn e(&mut self, e: Vec<String>) -> &mut Self {
        self.e = Some(e);
        self
    }

    /// Wait for instrumentation to finish before returning. Required for test runners.
    pub fn w(&mut self, w: bool) -> &mut Self {
        self.w = w;
        self
    }

    /// Write profiling data to file
    pub fn p(&mut self, p: &Path) -> &mut Self {
        self.p = Some(p.to_owned());
        self
    }

    /// Print raw results (otherwise decode report_key_streamresult). Use with
    /// [-e perf true] to generate raw output for performance measurements
    pub fn r(&mut self, r: bool) -> &mut Self {
        self.r = r;
        self
    }

    /// Turn off window animations while running
    pub fn no_window_animation(&mut self, no_window_animation: bool) -> &mut Self {
        self.no_window_animation = no_window_animation;
        self
    }

    /// Specify which user instrumentation runs in; current user if not specified
    pub fn user_id(&mut self, user_id: String) -> &mut Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut instrument = Command::new("adb");
        instrument.arg("shell");
        instrument.arg("am");
        instrument.arg("instrument");
        if self.w {
            instrument.arg("-w");
        }
        if let Some(p) = &self.p {
            instrument.arg("-p").arg(p);
        }
        if let Some(e) = &self.e {
            instrument.arg("-e").arg(
                e.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if self.no_window_animation {
            instrument.arg("--no-window-animation");
        }
        if self.r {
            instrument.arg("-r");
        }
        if let Some(user_id) = &self.user_id {
            instrument.arg("--user").arg(user_id);
        }
        instrument.output_err(true)?;
        Ok(())
    }
}
