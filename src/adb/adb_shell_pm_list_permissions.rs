use crate::error::*;
use std::process::Command;

#[derive(Clone, Default)]
pub struct AdbShellPmListPermissions {
    f: bool,
    d: bool,
    s: bool,
    u: bool,
    g: bool,
}

impl AdbShellPmListPermissions {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Only list dangerous permissions
    pub fn d(&mut self, d: bool) -> &mut Self {
        self.d = d;
        self
    }

    /// Print all information
    pub fn f(&mut self, f: bool) -> &mut Self {
        self.f = f;
        self
    }

    /// List only the permissions users will see
    pub fn u(&mut self, u: bool) -> &mut Self {
        self.u = u;
        self
    }

    /// Short summary
    pub fn s(&mut self, s: bool) -> &mut Self {
        self.s = s;
        self
    }

    /// Organize by group
    pub fn g(&mut self, g: bool) -> &mut Self {
        self.g = g;
        self
    }


    pub fn run(&self) -> Result<()> {
        let mut list_permissions = Command::new("adb");
        list_permissions.arg("shell");
        list_permissions.arg("pm");
        list_permissions.arg("list packages");
        if self.f {
            list_permissions.arg("-f");
        }
        if self.d {
            list_permissions.arg("-d");
        }
        if self.s {
            list_permissions.arg("-s");
        }
        if self.g {
            list_permissions.arg("-g");
        }
        if self.u {
            list_permissions.arg("-u");
        }
        list_permissions.output_err(true)?;
        Ok(())
    }
}
