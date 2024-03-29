mod build_apks;
mod build_bundle;
mod extract_apks;
mod get_device_spec;
mod get_size_total;
mod install_apks;

pub use build_apks::*;
pub use build_bundle::*;
pub use extract_apks::*;
pub use get_device_spec::*;
pub use get_size_total::*;
pub use install_apks::*;

use crate::{bundletool, error::*};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

const BUNDLETOOL_VERSION: &str = "1.8.2";

/// ## Bundletool
/// `bundletool` is the underlying tool that Android Studio, the Android Gradle plugin,
/// and Google Play use to build an Android App Bundle, and convert an app bundle into
/// the various APKs that are deployed to devices. `Bundletool` is also available to you
/// as a command line tool, so you can build app bundles yourself and recreate
/// Google Play's server-side build of your app's APKs.
///
///
/// ## Download bundletool
/// If you haven't already done so, download bundletool from the [`GitHub repository`].
///
///
/// ## Install bundletool
/// In variable environments needs to create new variable `BUNDLETOOL_PATH` and add
/// path to the `bundletool`
///
/// [`GitHub repository`](https://github.com/google/bundletool/releases)
#[derive(Clone, Copy)]
pub struct Bundletool;

impl Bundletool {
    /// Generate an APK set for all device configurations your app supports from your app
    /// bundle
    pub fn build_apks(self, bundle: &Path, output: &Path) -> BuildApks {
        BuildApks::new(bundle, output)
    }

    /// Generate AAB file from generated zip modules to specified path.
    /// Notice, that zip module must contents files in protobuf format
    pub fn build_bundle(self, modules: &[PathBuf], output: &Path) -> BuildBundle {
        BuildBundle::new(modules, output)
    }

    /// To measure the estimated download sizes of APKs in an APK set as they would be
    /// served compressed over-the-wire, use the get-size total
    pub fn get_size_total(self, apks: &Path) -> GetSizeTotal {
        GetSizeTotal::new(apks)
    }

    /// Extract device-specific APKs from an existing APK set
    /// If you have an existing APK set and you want to extract from it a subset of APKs
    /// that target a specific device configuration, you can use the extract-apks
    /// command and specify a device specification JSON
    pub fn extract_apks(self, apks: &Path, output_dir: &Path, device_spec: &Path) -> ExtractApks {
        ExtractApks::new(apks, output_dir, device_spec)
    }

    /// Use the install-apks command and specify the path of the APK set to deploy your
    /// app from an APK set
    pub fn install_apks(self, apks: PathBuf) -> InstallApks {
        InstallApks::new(&apks)
    }

    /// Generate and use device specification JSON files.
    /// Bundletool is capable of generating an APK set that targets a device configuration
    /// specified by a JSON file. To first generate a JSON file for a connected
    /// device, run the command
    pub fn get_device_spec(self, output: &Path) -> GetDeviceSpec {
        GetDeviceSpec::new(output)
    }
}

/// Find `bundletool` jar file in home directory then set environment variable and initialize it
pub fn bundletool() -> Result<Command> {
    let mut bundletool_init = Command::new("java");
    bundletool_init.arg("-jar");
    if let Ok(bundletool_path) = std::env::var("BUNDLETOOL_PATH") {
        bundletool_init.arg(bundletool_path);
    } else {
        let env_version =
            std::env::var("BUNDLETOOL_VERSION").unwrap_or_else(|_| BUNDLETOOL_VERSION.to_string());
        let bundletool_file = format!("bundletool-all-{}.jar", env_version);
        let bundletool_file_path = dirs::home_dir()
            .ok_or(Error::UnableToAccessHomeDirectory)?
            .join(bundletool_file);
        if bundletool_file_path.exists() {
            bundletool_init.arg(bundletool_file_path);
        } else {
            return Err(Error::BundletoolNotFound);
        }
    }
    Ok(bundletool_init)
}
