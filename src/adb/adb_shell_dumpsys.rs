use crate::error::*;
use std::process::Command;

#[derive(Clone, Default)]
pub struct AdbShellDumpsys {
    activity: bool,
    iphonesybinfo: bool,
    battery_set_level: Option<String>,
    battery_set_status: Option<String>,
    battery_reset: bool,
    battery_set_usb: Option<String>,
}

impl AdbShellDumpsys {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn activity(&mut self, activity: bool) -> &mut Self {
        self.activity = activity;
        self
    }

    pub fn iphonesybinfo(&mut self, iphonesybinfo: bool) -> &mut Self {
        self.iphonesybinfo = iphonesybinfo;
        self
    }

    pub fn battery_set_level(&mut self, battery_set_level: String) -> &mut Self {
        self.battery_set_level = Some(battery_set_level.to_owned());
        self
    }

    pub fn battery_set_status(&mut self, battery_set_status: String) -> &mut Self {
        self.battery_set_status = Some(battery_set_status.to_owned());
        self
    }

    pub fn battery_reset(&mut self, battery_reset: bool) -> &mut Self {
        self.battery_reset = battery_reset;
        self
    }

    pub fn battery_set_usb(&mut self, battery_set_usb: String) -> &mut Self {
        self.battery_set_usb = Some(battery_set_usb.to_owned());
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut dumpsys = Command::new("adb");
        dumpsys.arg("shell");
        dumpsys.arg("dumpsys");
        if self.activity {
            dumpsys.arg("activity");
        }
        if self.iphonesybinfo {
            dumpsys.arg("iphonesybinfo");
        }
        if let Some(battery_set_level) = &self.battery_set_level {
            dumpsys.arg("battery set level").arg(battery_set_level);
        }
        if let Some(battery_set_status) = &self.battery_set_status {
            dumpsys.arg("battery set status").arg(battery_set_status);
        }
        if self.battery_reset {
            dumpsys.arg("battery reset");
        }
        if let Some(battery_set_usb) = &self.battery_set_usb {
            dumpsys.arg("battery set usb").arg(battery_set_usb);
        }
        dumpsys.output_err(true)?;
        Ok(())
    }
}
