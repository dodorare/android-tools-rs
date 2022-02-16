use crate::error::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Default)]
pub struct AdbTools {
    install: Option<PathBuf>,
    forward: Option<String>,
    connect: Option<String>,
    devices: bool,
    devices_l: bool,
    kill_server: bool,
    tcpip: Option<String>,
    help: bool,
    d: bool,
    e: bool,
    s: bool,
    a: bool,
    h: Option<String>,
    p: Option<String>,
    l: Option<String>,
    version: bool,
}

impl AdbTools {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Only push files that are newer on the host than the device
    pub fn install(&mut self, install: &Path) -> &mut Self {
        self.install = Some(install.to_owned());
        self
    }

    pub fn forward(&mut self, forward: String) -> &mut Self {
        self.forward = Some(forward);
        self
    }

    pub fn connect(&mut self, connect: String) -> &mut Self {
        self.connect = Some(connect);
        self
    }

    pub fn devices(&mut self, devices: bool) -> &mut Self {
        self.devices = devices;
        self
    }

    pub fn devices_l(&mut self, devices_l: bool) -> &mut Self {
        self.devices_l = devices_l;
        self
    }

    pub fn kill_server(&mut self, kill_server: bool) -> &mut Self {
        self.kill_server = kill_server;
        self
    }

    pub fn tcpip(&mut self, tcpip: String) -> &mut Self {
        self.tcpip = Some(tcpip);
        self
    }

    /// Show this help message
    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// Use USB device (error if multiple devices connected)
    pub fn d(&mut self, d: bool) -> &mut Self {
        self.d = d;
        self
    }

    /// Use TCP/IP device (error if multiple TCP/IP devices available)
    pub fn e(&mut self, e: bool) -> &mut Self {
        self.e = e;
        self
    }

    /// Use device with given serial (overrides $ANDROID_SERIAL)
    pub fn s(&mut self, s: bool) -> &mut Self {
        self.s = s;
        self
    }

    /// Listen on all network interfaces, not just localhost
    pub fn a(&mut self, a: bool) -> &mut Self {
        self.a = a;
        self
    }

    /// Name of adb server host [default=localhost]
    pub fn h(&mut self, h: String) -> &mut Self {
        self.h = Some(h);
        self
    }

    /// Port of adb server [default=5037]
    pub fn p(&mut self, p: String) -> &mut Self {
        self.p = Some(p);
        self
    }

    /// Listen on given socket for adb server [default=tcp:localhost:5037]
    pub fn l(&mut self, l: String) -> &mut Self {
        self.l = Some(l);
        self
    }

    pub fn version(&mut self, version: bool) -> &mut Self {
        self.version = version;
        self
    }
    pub fn run(&self) -> Result<()> {
        let mut adb = Command::new("adb");
        if self.devices {
            adb.arg("devices");
        }
        if let Some(install) = &self.install {
            adb.arg("install").arg(install);
        }
        if let Some(forward) = &self.forward {
            adb.arg("forward").arg(forward);
        }
        if let Some(connect) = &self.connect {
            adb.arg("connect").arg(connect);
        }
        if self.devices_l {
            adb.arg("devices -l");
        }
        if self.kill_server {
            adb.arg("kill-server");
        }
        if let Some(tcpip) = &self.tcpip {
            adb.arg("tcpip").arg(tcpip);
        }
        if self.help {
            adb.arg("--help");
        }
        if self.d {
            adb.arg("-d");
        }
        if self.e {
            adb.arg("-e");
        }
        if self.s {
            adb.arg("-s");
        }
        if self.a {
            adb.arg("-a");
        }
        if let Some(h) = &self.h {
            adb.arg("-H").arg(h);
        }
        if let Some(p) = &self.p {
            adb.arg("-P").arg(p);
        }
        if let Some(l) = &self.l {
            adb.arg("-L").arg(l);
        }
        if self.version {
            adb.arg("--version");
        }
        adb.output_err(true)?;
        Ok(())
    }
}
