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
#[cfg(feature = "bundletool")]
pub mod bundletool;
#[cfg(feature = "java-tools")]
pub mod java_tools;
