use crate::error::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Default)]
pub struct AdbShellAmStart {
    d: bool,
    w: bool,
    start_profiler: Option<PathBuf>,
    p: bool,
    r: bool,
    s: bool,
    opengl_trace: bool,
    user_id: Option<String>,
}

impl AdbShellAmStart {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Enable debugging
    pub fn d(&mut self, d: bool) -> &mut Self {
        self.d = d;
        self
    }

    /// Wait for launch to complete
    pub fn w(&mut self, w: bool) -> &mut Self {
        self.w = w;
        self
    }

    /// Start profiler and send results to file
    pub fn start_profiler(&mut self, start_profiler: &Path) -> &mut Self {
        self.start_profiler = Some(start_profiler.to_owned());
        self
    }

    /// Like `--start-profiler`, but profiling stops when the app goes idle
    pub fn p(&mut self, p: bool) -> &mut Self {
        self.p = p;
        self
    }

    /// Repeat the activity launch count times. Prior to each repeat, the top
    /// activity will be finished
    pub fn r(&mut self, r: bool) -> &mut Self {
        self.r = r;
        self
    }

    /// Force stop the target app before starting the activity
    pub fn s(&mut self, s: bool) -> &mut Self {
        self.s = s;
        self
    }

    /// Enable tracing of OpenGL functions
    pub fn opengl_trace(&mut self, opengl_trace: bool) -> &mut Self {
        self.opengl_trace = opengl_trace;
        self
    }

    /// Specify which user to run as; if not specified, then run as the
    /// current user.
    pub fn user_id(&mut self, user_id: String) -> &mut Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut start = Command::new("adb");
        start.arg("shell");
        start.arg("am");
        start.arg("start");
        if self.d {
            start.arg("-D");
        }
        if self.w {
            start.arg("-W");
        }
        if let Some(start_profiler) = &self.start_profiler {
            start.arg("--start-profiler").arg(start_profiler);
        }
        if self.p {
            start.arg("-P");
        }
        if self.r {
            start.arg("-R");
        }
        if self.s {
            start.arg("-S");
        }
        if self.opengl_trace {
            start.arg("--opengl-trace");
        }
        if let Some(user_id) = &self.user_id {
            start.arg("--user").arg(user_id);
        }
        start.output_err(true)?;
        Ok(())
    }
}
