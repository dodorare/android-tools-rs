use std::path::PathBuf;

use error::AndroidError;

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
        std::path::PathBuf::from(
            sdk_path.unwrap_or(sdk_install_path()?.to_str().unwrap().to_string()),
        )
    };
    Ok(sdk_path)
}

/// Default installation path
pub fn sdk_install_path() -> crate::error::Result<PathBuf> {
    let home_dir_path = dirs::home_dir().ok_or(AndroidError::HomeDirectoryUnableToAccess)?;
    #[cfg(target_os = "windows")]
    let path = std::path::Path::new("Local").join("Android").join("Sdk");
    #[cfg(target_os = "macos")]
    let path = std::path::Path::new("Library").join("Android").join("sdk");
    #[cfg(target_os = "linux")]
    let path = std::path::Path::new("Android").join("sdk");

    #[cfg(target_os = "windows")]
    let app_data = std::path::Path::new("AppData");
    #[cfg(target_os = "windows")]
    let sdk_path = home_dir_path.join(app_data).join(path);

    #[cfg(not(target_os = "windows"))]
    let sdk_path = home_dir_path.join(path);

    if !sdk_path.exists() {
        return Err(AndroidError::AndroidSdkNotFound)?;
    }
    Ok(sdk_path)
}

pub fn find_max_version(target_dir: &std::path::Path) -> crate::error::Result<String> {
    let max_version = std::fs::read_dir(&target_dir)?
        .filter_map(|path| path.ok())
        .filter(|path| path.path().is_dir())
        .filter_map(|path| path.file_name().into_string().ok())
        .filter(|name| name.chars().next().unwrap().is_ascii_digit())
        .max()
        .ok_or(AndroidError::AndroidToolIsNotFound)?;
    Ok(max_version)
}
