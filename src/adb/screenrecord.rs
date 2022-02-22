use crate::error::*;
use std::process::Command;

#[derive(Clone, Default)]
pub struct Screenrecord {
    help: bool,
    rotate: bool,
    verbose: bool,
    size: Option<String>,
    bit_rate: Option<String>,
    time_limit: Option<String>,
}

impl Screenrecord {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Displays command syntax and options
    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// Rotates the output 90 degrees. This feature is experimental
    pub fn rotate(&mut self, rotate: bool) -> &mut Self {
        self.rotate = rotate;
        self
    }

    /// Displays log information on the command-line screen. If you do not set
    /// this option, the utility does not display any information while running.
    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose = verbose;
        self
    }

    /// Sets the video size: 1280x720. The default value is the device's native
    /// display resolution (if supported), 1280x720 if not. For best results, use
    /// a size supported by your device's Advanced Video Coding (AVC) encoder.
    pub fn size(&mut self, size: String) -> &mut Self {
        self.size = Some(size);
        self
    }

    /// Sets the video bit rate for the video, in megabits per second. The default
    /// value is 4Mbps. You can increase the bit rate to improve video quality,
    /// but doing so results in larger movie files. The following example sets the
    /// recording bit rate to 6Mbps:
    ///
    /// `screenrecord --bit-rate 6000000 /sdcard/demo.mp4`
    pub fn bit_rate(&mut self, bit_rate: String) -> &mut Self {
        self.bit_rate = Some(bit_rate);
        self
    }

    /// Sets the maximum recording time, in seconds. The default and maximum value
    /// is 180 (3 minutes).
    pub fn time_limit(&mut self, time_limit: String) -> &mut Self {
        self.time_limit = Some(time_limit);
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut screenrecord = Command::new("adb");
        screenrecord.arg("screenrecord");
        if self.help {
            screenrecord.arg("--help");
        }
        if self.rotate {
            screenrecord.arg("--rotate");
        }
        if self.verbose {
            screenrecord.arg("--verbose");
        }
        if let Some(size) = &self.size {
            screenrecord.arg("--size").arg(size);
        }
        if let Some(bit_rate) = &self.bit_rate {
            screenrecord.arg("--bit-rate").arg(bit_rate);
        }
        if let Some(time_limit) = &self.time_limit {
            screenrecord.arg("--time-limit").arg(time_limit);
        }
        Ok(())
    }
}
