use crate::error::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Default)]
pub struct Shell {
    pwd: bool,
    netstat: bool,
    service_list: bool,
    ps: bool,
    wm_size: Option<String>,
    ls: bool,
    ls_s: bool,
    ls_r: bool,
    install: Option<PathBuf>,
    install_r: Option<PathBuf>,
    uninstall: Option<String>,
    permissions_groups: bool,
    list_permissions_g_r: bool,
    dump: Option<String>,
    path: Option<PathBuf>,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Print current working directory
    pub fn pwd(&mut self, pwd: bool) -> &mut Self {
        self.pwd = pwd;
        self
    }

    /// List TCP connectivity
    pub fn netstat(&mut self, netstat: bool) -> &mut Self {
        self.netstat = netstat;
        self
    }

    /// List all services
    pub fn service_list(&mut self, service_list: bool) -> &mut Self {
        self.service_list = service_list;
        self
    }

    /// Print process status
    pub fn ps(&mut self, ps: bool) -> &mut Self {
        self.ps = ps;
        self
    }

    /// Displays the current screen resolution
    pub fn wm_size(&mut self, wm_size: String) -> &mut Self {
        self.wm_size = Some(wm_size.to_owned());
        self
    }

    /// List directory contents
    pub fn ls(&mut self, ls: bool) -> &mut Self {
        self.ls = ls;
        self
    }

    /// Print size of each file
    pub fn ls_s(&mut self, ls_s: bool) -> &mut Self {
        self.ls_s = ls_s;
        self
    }

    /// Install app or install app from phone path
    pub fn install(&mut self, install: &Path) -> &mut Self {
        self.install = Some(install.to_owned());
        self
    }

    /// Install app from phone path
    pub fn install_r(&mut self, install_r: &Path) -> &mut Self {
        self.install_r = Some(install_r.to_owned());
        self
    }

    /// Remove the app
    pub fn uninstall(&mut self, uninstall: String) -> &mut Self {
        self.uninstall = Some(uninstall.to_owned());
        self
    }

    /// List permission groups definitions
    pub fn permissions_groups(&mut self, permissions_groups: bool) -> &mut Self {
        self.permissions_groups = permissions_groups;
        self
    }

    /// List permissions details
    pub fn list_permissions_g_r(&mut self, list_permissions_g_r: bool) -> &mut Self {
        self.list_permissions_g_r = list_permissions_g_r;
        self
    }

    /// List info on one package
    pub fn dump(&mut self, dump: String) -> &mut Self {
        self.dump = Some(dump.to_owned());
        self
    }

    /// Path to the apk file
    pub fn path(&mut self, path: &Path) -> &mut Self {
        self.path = Some(path.to_owned());
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut shell = Command::new("adb");
        shell.arg("shell");
        if self.pwd {
            shell.arg("pwd");
        }
        if self.netstat {
            shell.arg("netstat");
        }
        if self.service_list {
            shell.arg("service list");
        }
        if self.ps {
            shell.arg("ps");
        }
        if let Some(wm_size) = &self.wm_size {
            shell.arg("wm size").arg(wm_size);
        }
        if self.ls {
            shell.arg("ls");
        }
        if self.ls_s {
            shell.arg("ls -s");
        }
        if self.ls_r {
            shell.arg("ls -R");
        }
        if let Some(install) = &self.install {
            shell.arg("install").arg(install);
        }
        if let Some(install_r) = &self.install_r {
            shell.arg("install -r").arg(install_r);
        }
        if let Some(uninstall) = &self.uninstall {
            shell.arg("uninstall").arg(uninstall);
        }
        if self.permissions_groups {
            shell.arg("permissions groups");
        }
        if self.list_permissions_g_r {
            shell.arg("list permissions -g -r");
        }
        if let Some(dump) = &self.dump {
            shell.arg("dump").arg(dump);
        }
        if let Some(path) = &self.path {
            shell.arg("path").arg(path);
        }
        shell.output_err(true)?;
        Ok(())
    }
}
