use crate::error::*;
use std::process::Command;

/// ## Dumpsys
/// 
/// dumpsys is a tool that runs on Android devices and provides information about
/// system services. You can call dumpsys from the command line using the Android
/// Debug Bridge (ADB) to get diagnostic output for all system services running
/// on a connected device. This output is typically more verbose than you may
/// want, so use the command line options described below to get output for only
/// the system services you're interested in. This page also describes how to use
/// dumpsys to accomplish common tasks, such as inspecting input, RAM, battery,
/// or network diagnostics.
#[derive(Clone, Default)]
pub struct AdbShellDumpsys {
    t: Option<String>,
    help: bool,
    l: bool,
    skip: Option<Vec<String>>,
    service: Option<String>,
    c: bool,
    h: bool,
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

    /// Specifies the timeout period in seconds. When not specified, the default 
    /// value is 10 seconds.
    pub fn t(&mut self, t: String) -> &mut Self {
        self.t = Some(t);
        self
    }

    /// Prints out help text for the dumpsys tool.
    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// Outputs a complete list of system services that you can use with dumpsys.
    pub fn l(&mut self, l: bool) -> &mut Self {
        self.l = l;
        self
    }

    /// Specifies the services that you do not want to include in the output.
    pub fn skip(&mut self, skip: Vec<String>) -> &mut Self {
        self.skip = Some(skip);
        self
    }

    /// Specifies the service that you want to output. Some services may allow you
    /// to pass optional arguments. You can learn about these optional arguments
    /// by passing the -h option with the service, as shown below:
    ///
    /// ```xml
    /// adb shell dumpsys procstats -h
    /// ```
    pub fn service(&mut self, service: String) -> &mut Self {
        self.service = Some(service);
        self
    }

    /// When specifying certain services, append this option to output data in a
    /// machine-friendly format.
    pub fn c(&mut self, c: bool) -> &mut Self {
        self.c = c;
        self
    }

    /// For certain services, append this option to see help text and additional
    /// options for that service.
    pub fn h(&mut self, h: bool) -> &mut Self {
        self.h = h;
        self
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
        self.battery_set_level = Some(battery_set_level);
        self
    }

    pub fn battery_set_status(&mut self, battery_set_status: String) -> &mut Self {
        self.battery_set_status = Some(battery_set_status);
        self
    }

    pub fn battery_reset(&mut self, battery_reset: bool) -> &mut Self {
        self.battery_reset = battery_reset;
        self
    }

    pub fn battery_set_usb(&mut self, battery_set_usb: String) -> &mut Self {
        self.battery_set_usb = Some(battery_set_usb);
        self
    }

    /// Runs `adb shell dumpsys` commands
    pub fn run(&self) -> Result<()> {
        let mut dumpsys = Command::new("adb");
        dumpsys.arg("shell");
        dumpsys.arg("dumpsys");
        if let Some(t) = &self.t {
            dumpsys.arg("-t").arg(t);
        }
        if self.help {
            dumpsys.arg("--help");
        }
        if self.l {
            dumpsys.arg("-l");
        }
        if let Some(skip) = &self.skip {
            dumpsys.arg("--skip").arg(
                skip
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(service) = &self.service {
            dumpsys.arg(service);
        }
        if self.c {
            dumpsys.arg("-c");
        }
        if self.h {
            dumpsys.arg("-h");
        }
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
