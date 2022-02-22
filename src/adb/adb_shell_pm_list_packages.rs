use crate::error::*;
use std::process::Command;

#[derive(Clone, Default)]
pub struct AdbShellPmListPackages {
    f: bool,
    d: bool,
    s: bool,
    e: bool,
    third_party_packages: bool,
    i: bool,
    u: bool,
    user_id: Option<String>,
}

impl AdbShellPmListPackages {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Filter to only show disabled packages
    pub fn d(&mut self, d: bool) -> &mut Self {
        self.d = d;
        self
    }

    /// See their associated file
    pub fn f(&mut self, f: bool) -> &mut Self {
        self.f = f;
        self
    }

    /// Filter to only show enabled packages
    pub fn e(&mut self, e: bool) -> &mut Self {
        self.e = e;
        self
    }

    /// Filter to only show third party packages
    pub fn third_party_packages(&mut self, third_party_packages: bool) -> &mut Self {
        self.third_party_packages = third_party_packages;
        self
    }

    /// Also include uninstalled packages
    pub fn u(&mut self, u: bool) -> &mut Self {
        self.u = u;
        self
    }

    /// Filter to only show system packages
    pub fn s(&mut self, s: bool) -> &mut Self {
        self.s = s;
        self
    }

    /// See the installer for the packages
    pub fn i(&mut self, i: bool) -> &mut Self {
        self.i = i;
        self
    }

    /// The user space to query
    pub fn user_id(&mut self, user_id: String) -> &mut Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut list_packages = Command::new("adb");
        list_packages.arg("shell");
        list_packages.arg("pm");
        list_packages.arg("list packages");
        if self.f {
            list_packages.arg("-f");
        }
        if self.d {
            list_packages.arg("-d");
        }
        if self.s {
            list_packages.arg("-s");
        }
        if self.e {
            list_packages.arg("-e");
        }
        if self.i {
            list_packages.arg("-i");
        }
        if self.s {
            list_packages.arg("-S");
        }
        if self.third_party_packages {
            list_packages.arg("-3");
        }
        if self.u {
            list_packages.arg("-u");
        }
        if let Some(user_id) = &self.user_id {
            list_packages.arg("--user").arg(user_id);
        }
        list_packages.output_err(true)?;
        Ok(())
    }
}
