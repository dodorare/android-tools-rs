use super::bundletool;
use crate::error::*;
use std::path::{Path, PathBuf};

/// ## Generate and use device specification JSON files
///
/// `Bundletool` is capable of generating an APK set that targets a device configuration
/// specified by a JSON file. To first generate a JSON file for a connected device, run
/// the following command:
///
/// ```sh
/// `bundletool get-device-spec --output=/tmp/device-spec.json`
/// ```
///
/// `bundletool` creates a JSON file for your device in the directory the tool is located.
/// You can then pass it to `bundletool` to generate a set of APKs that target only the
/// configuration described in that JSON file as follows:
///
/// ```sh
/// `bundletool build-apks --device-spec=/MyApp/pixel2.json`
/// `--bundle=/MyApp/my_app.aab --output=/MyApp/my_app.apks`
/// ```
#[derive(Debug)]
pub struct GetDeviceSpec {
    output: PathBuf,
}
impl GetDeviceSpec {
    /// Connect your device or use emulator to get device specification in provided path using `bundletool`
    pub fn new(output: &Path) -> Self {
        Self {
            output: output.to_owned(),
        }
    }

    /// Runs `bundletool` commands to get device specification
    pub fn run(&self) -> Result<()> {
        let mut get_device_spec = bundletool()?;
        get_device_spec.arg("get-device-spec");
        get_device_spec.arg("--output").arg(&self.output);
        get_device_spec.output_err(true)?;
        Ok(())
    }
}
