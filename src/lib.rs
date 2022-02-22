use error::AndroidError;
use std::path::PathBuf;

/// On Windows adds `.exe` to given string.
macro_rules! bin {
    ($bin:expr) => {{
        #[cfg(not(target_os = "windows"))]
        let bin = $bin;
        #[cfg(target_os = "windows")]
        let bin = concat!($bin, ".exe");
        bin
    }};
}

pub mod error;

#[cfg(feature = "aapt2")]
pub mod aapt2;
#[cfg(feature = "adb")]
pub mod adb;
#[cfg(feature = "bundletool")]
pub mod bundletool;
#[cfg(feature = "emulator")]
pub mod emulator;
#[cfg(feature = "java-tools")]
pub mod java_tools;

/// Return SDK path from found environment variable
pub fn sdk_path_from_env() -> crate::error::Result<PathBuf> {
    let sdk_path = {
        let sdk_path = std::env::var("ANDROID_SDK_ROOT")
            .ok()
            .or_else(|| std::env::var("ANDROID_SDK_PATH").ok())
            .or_else(|| std::env::var("ANDROID_HOME").ok());
        std::path::PathBuf::from(sdk_path.ok_or(AndroidError::AndroidSdkNotFound)?)
    };
    Ok(sdk_path)
}
