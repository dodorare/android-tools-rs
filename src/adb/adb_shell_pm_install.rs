use crate::error::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

use super::InstallLocation;

#[derive(Clone, Default)]
pub struct AdbShellPmInstall {
    path: Option<PathBuf>,
    r: bool,
    t: bool,
    i: Option<String>,
    install_location: Option<InstallLocation>,
    f: bool,
    d: bool,
    g: bool,
    fastdeploy: bool,
    incremental: bool,
    no_incremental: bool,
}

impl AdbShellPmInstall {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Downloads a package somewhere temporarily, then installs it
    pub fn path(&mut self, path: &Path) -> &mut Self {
        self.path = Some(path.to_owned());
        self
    }

    /// Reinstall an existing app, keeping its data
    pub fn r(&mut self, r: bool) -> &mut Self {
        self.r = r;
        self
    }

    /// Allow test APKs to be installed. Gradle generates a test APK when you
    /// have only run or debugged your app or have used the Android Studio
    /// Build > Build APK command. If the APK is built using a developer
    /// preview SDK (if the targetSdkVersion is a letter instead of a number)
    /// you must include the -t option with the install command if you are
    /// installing a test APK
    pub fn t(&mut self, t: bool) -> &mut Self {
        self.t = t;
        self
    }

    /// Print all information
    pub fn f(&mut self, f: bool) -> &mut Self {
        self.f = f;
        self
    }

    /// Specify the installer package name
    pub fn i(&mut self, i: String) -> &mut Self {
        self.i = Some(i);
        self
    }

    /// Sets the install location using one of the following values:
    ///
    /// * 0: Use the default install location
    /// * 1: Install on internal device storage
    /// * 2: Install on external media
    pub fn install_location(&mut self, install_location: InstallLocation) -> &mut Self {
        self.install_location = Some(install_location);
        self
    }

    /// Short summary
    pub fn d(&mut self, d: bool) -> &mut Self {
        self.d = d;
        self
    }

    /// Organize by group
    pub fn g(&mut self, g: bool) -> &mut Self {
        self.g = g;
        self
    }

    pub fn fastdeploy(&mut self, fastdeploy: bool) -> &mut Self {
        self.fastdeploy = fastdeploy;
        self
    }

    pub fn incremental(&mut self, incremental: bool) -> &mut Self {
        self.incremental = incremental;
        self
    }

    pub fn no_incremental(&mut self, no_incremental: bool) -> &mut Self {
        self.no_incremental = no_incremental;
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut install = Command::new("adb");
        install.arg("shell");
        install.arg("pm");
        install.arg("list packages");
        if self.f {
            install.arg("-f");
        }
        if self.r {
            install.arg("-r");
        }
        if self.t {
            install.arg("-t");
        }
        if self.d {
            install.arg("-d");
        }
        if self.g {
            install.arg("-g");
        }
        if self.fastdeploy {
            install.arg("--fastdeploy");
        }
        if self.incremental {
            install.arg("--incremental");
        }
        if self.no_incremental {
            install.arg("--no-incremental");
        }
        if let Some(install_location) = &self.install_location {
            install
                .arg("--install-location")
                .arg(install_location.to_string());
        }
        if let Some(path) = &self.path {
            install.arg(path);
        }
        install.output_err(true)?;
        Ok(())
    }
}
