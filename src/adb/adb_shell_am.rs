use crate::error::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

use super::ScreenCompatibilityMode;

#[derive(Clone, Default)]
pub struct AdbShellAm {
    d: bool,
    w: bool,
    p: Option<PathBuf>,
    r: bool,
    s: bool,
    n: bool,
    user: Option<String>,
    opengl_trace: bool,
    start_profiler: Option<PathBuf>,
    start: bool,
    startservice: bool,
    force_stop: Option<PathBuf>,
    kill: Option<PathBuf>,
    kill_all: bool,
    broadcast: Option<String>,
    instrument: bool,
    no_window_animation: bool,
    profile_start: Option<PathBuf>,
    profile_stop: bool,
    dumpheap: Option<PathBuf>,
    set_debug_app: Option<PathBuf>,
    persistent: bool,
    clear_debug_app: bool,
    monitor: bool,
    gdb: bool,
    screen_compat: Option<ScreenCompatibilityMode>,
    display_size: Option<String>,
    display_density: Option<String>,
    to_uri: bool,
    to_intent_uri: bool,
}

impl AdbShellAm {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn d(&mut self, d: bool) -> &mut Self {
        self.d = d;
        self
    }

    pub fn w(&mut self, w: bool) -> &mut Self {
        self.w = w;
        self
    }

    pub fn p(&mut self, p: &Path) -> &mut Self {
        self.p = Some(p.to_owned());
        self
    }

    pub fn r(&mut self, r: bool) -> &mut Self {
        self.r = r;
        self
    }

    pub fn s(&mut self, s: bool) -> &mut Self {
        self.s = s;
        self
    }

    pub fn n(&mut self, n: bool) -> &mut Self {
        self.n = n;
        self
    }

    pub fn user(&mut self, user: String) -> &mut Self {
        self.user = Some(user);
        self
    }

    pub fn opengl_trace(&mut self, opengl_trace: bool) -> &mut Self {
        self.opengl_trace = opengl_trace;
        self
    }

    pub fn start_profiler(&mut self, start_profiler: &Path) -> &mut Self {
        self.start_profiler = Some(start_profiler.to_owned());
        self
    }

    pub fn start(&mut self, start: bool) -> &mut Self {
        self.start = start;
        self
    }

    pub fn startservice(&mut self, startservice: bool) -> &mut Self {
        self.startservice = startservice;
        self
    }

    pub fn force_stop(&mut self, force_stop: &Path) -> &mut Self {
        self.force_stop = Some(force_stop.to_owned());
        self
    }

    pub fn kill(&mut self, kill: &Path) -> &mut Self {
        self.kill = Some(kill.to_owned());
        self
    }

    pub fn kill_all(&mut self, kill_all: bool) -> &mut Self {
        self.kill_all = kill_all;
        self
    }

    pub fn broadcast(&mut self, broadcast: String) -> &mut Self {
        self.broadcast = Some(broadcast);
        self
    }

    pub fn instrument(&mut self, instrument: bool) -> &mut Self {
        self.instrument = instrument;
        self
    }

    pub fn no_window_animation(&mut self, no_window_animation: bool) -> &mut Self {
        self.no_window_animation = no_window_animation;
        self
    }

    pub fn profile_start(&mut self, profile_start: &Path) -> &mut Self {
        self.profile_start = Some(profile_start.to_owned());
        self
    }

    pub fn profile_stop(&mut self, profile_stop: bool) -> &mut Self {
        self.profile_stop = profile_stop;
        self
    }

    pub fn dumpheap(&mut self, dumpheap: &Path) -> &mut Self {
        self.dumpheap = Some(dumpheap.to_owned());
        self
    }

    pub fn set_debug_app(&mut self, set_debug_app: &Path) -> &mut Self {
        self.set_debug_app = Some(set_debug_app.to_owned());
        self
    }

    pub fn persistent(&mut self, persistent: bool) -> &mut Self {
        self.persistent = persistent;
        self
    }

    pub fn clear_debug_app(&mut self, clear_debug_app: bool) -> &mut Self {
        self.clear_debug_app = clear_debug_app;
        self
    }

    pub fn monitor(&mut self, monitor: bool) -> &mut Self {
        self.monitor = monitor;
        self
    }

    pub fn gdb(&mut self, gdb: bool) -> &mut Self {
        self.gdb = gdb;
        self
    }

    pub fn screen_compat(&mut self, screen_compat: ScreenCompatibilityMode) -> &mut Self {
        self.screen_compat = Some(screen_compat);
        self
    }

    pub fn display_size(&mut self, display_size: String) -> &mut Self {
        self.display_size = Some(display_size);
        self
    }

    pub fn display_density(&mut self, display_density: String) -> &mut Self {
        self.display_density = Some(display_density);
        self
    }

    pub fn to_uri(&mut self, to_uri: bool) -> &mut Self {
        self.to_uri = to_uri;
        self
    }

    pub fn to_intent_uri(&mut self, to_intent_uri: bool) -> &mut Self {
        self.to_intent_uri = to_intent_uri;
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut am = Command::new("adb");
        am.arg("shell");
        am.arg("am");
        if self.d {
            am.arg("-D");
        }
        if self.w {
            am.arg("-W");
        }
        if let Some(p) = &self.p {
            am.arg("-P").arg(p);
        }
        if self.r {
            am.arg("-R");
        }
        if self.s {
            am.arg("-S");
        }
        if self.n {
            am.arg("-n");
        }
        if let Some(user) = &self.user {
            am.arg("-user").arg(user);
        }
        if self.opengl_trace {
            am.arg("--opengl-trace");
        }
        if let Some(start_profiler) = &self.start_profiler {
            am.arg("--start-profiler").arg(start_profiler);
        }
        if self.start {
            am.arg("start");
        }
        if self.startservice {
            am.arg("startservice");
        }
        if let Some(force_stop) = &self.force_stop {
            am.arg("force-stop").arg(force_stop);
        }
        if let Some(kill) = &self.kill {
            am.arg("kill").arg(kill);
        }
        if self.kill_all {
            am.arg("kill-all");
        }
        if let Some(broadcast) = &self.broadcast {
            am.arg("broadcast").arg(broadcast);
        }
        if self.instrument {
            am.arg("instrument");
        }
        if self.no_window_animation {
            am.arg("--no-window-animation");
        }
        if let Some(profile_start) = &self.profile_start {
            am.arg("profile start").arg(profile_start);
        }
        if self.profile_stop {
            am.arg("profile stop");
        }
        if let Some(dumpheap) = &self.dumpheap {
            am.arg("dumpheap").arg(dumpheap);
        }
        if let Some(set_debug_app) = &self.set_debug_app {
            am.arg("set-debug-app").arg(set_debug_app);
        }
        if self.persistent {
            am.arg("--persistent");
        }
        if self.clear_debug_app {
            am.arg("clear-debug-app");
        }
        if self.monitor {
            am.arg("monitor");
        }
        if self.gdb {
            am.arg("gdb");
        }
        if self.gdb {
            am.arg("--gdb");
        }
        if let Some(screen_compat) = &self.screen_compat {
            am.arg("screen-compat").arg(screen_compat.to_string());
        }
        if let Some(display_size) = &self.display_size {
            am.arg("display-size").arg(display_size);
        }
        if let Some(display_density) = &self.display_density {
            am.arg("display-density").arg(display_density);
        }
        if self.to_uri {
            am.arg("to-uri");
        }
        if self.to_intent_uri {
            am.arg("to-intent-uri");
        }
        am.output_err(true)?;
        Ok(())
    }
}
