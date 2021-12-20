//! Android Asset Packaging Tool 2.0 (AAPT2).
//! https://developer.android.com/studio/command-line/aapt2
//! https://android.googlesource.com/platform/frameworks/base/+/master/tools/aapt2
//!
//! The main idea behind `AAPT2`, apart from new features, is that it divides
//! the 'package' step into two: 'compile' and 'link'. It improves performance,
//! since if only one file changes, you only need to recompile that one file and
//! link all the intermediate files with the 'link' command.

mod compile;
mod convert;
mod daemon;
mod diff;
mod dump;
mod link;
mod optimize;
mod version;

pub use compile::*;
pub use convert::*;
pub use daemon::*;
pub use diff::*;
pub use dump::*;
pub use link::*;
pub use optimize::*;
pub use version::*;

use self::{daemon::Aapt2Daemon, diff::Aapt2Diff, version::Aapt2Version};
use crate::error::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

/// [`AAPT2`](https://developer.android.com/studio/command-line/aapt2)
/// (Android Asset Packaging Tool) is a build tool that Android Studio
/// and Android Gradle Plugin use to compile and package your app’s resources.
/// [`AAPT2`] parses, indexes, and compiles the resources into a binary format
/// that is optimized for the Android platform
#[derive(Clone, Copy)]
pub struct Aapt2;

impl Aapt2 {
    /// Compiles resources incrementally from given resource path
    pub fn compile_incremental(self, res_path: &Path, compiled_res: &Path) -> Aapt2Compile {
        Aapt2Compile::new(res_path, compiled_res)
    }

    /// Compiles resources from given resource dir
    pub fn compile_dir(self, res_dir: &Path, compiled_res: &Path) -> Aapt2Compile {
        Aapt2Compile::new_from_res_dir(res_dir, compiled_res)
    }

    /// Compiles resources from given resource zip
    pub fn compile_zip(self, res_zip: &Path, compiled_res: &Path) -> Aapt2Compile {
        Aapt2Compile::new_from_res_zip(res_zip, compiled_res)
    }

    /// Links given list of resources into an APK
    pub fn link_inputs(self, inputs: &[PathBuf], output_apk: &Path, manifest: &Path) -> Aapt2Link {
        Aapt2Link::new(inputs, output_apk, manifest)
    }

    /// Links resources from given /compiled_res folder into an APK
    pub fn link_compiled_res(
        self,
        compiled_res: Option<PathBuf>,
        output_apk: &Path,
        manifest: &Path,
    ) -> Aapt2Link {
        Aapt2Link::new_from_compiled_res(compiled_res, output_apk, manifest)
    }

    /// Used for printing information about the APK you generated using the link command
    pub fn dump(self, subcommand: SubCommand, filename_apk: &Path) -> Aapt2Dump {
        Aapt2Dump::new(subcommand, filename_apk)
    }

    /// Prints the differences in resources of two APKs
    pub fn diff(self, file: &[PathBuf]) -> Aapt2Diff {
        Aapt2Diff::new(file)
    }

    /// Preforms resource optimizations on an APK
    pub fn optimize(self, output_apk: &Path, output_dir: &Path) -> Aapt2Optimize {
        Aapt2Optimize::new(output_apk, output_dir)
    }

    /// Converts an apk between binary and proto formats
    pub fn convert(self, o: &Path) -> Aapt2Convert {
        Aapt2Convert::new(o)
    }

    /// Prints the version of aapt2
    pub fn version(self, version: String) -> Aapt2Version {
        Aapt2Version::new(version)
    }

    /// Runs aapt in daemon mode. Each subsequent line is a single parameter to the
    /// command. The end of an invocation is signaled by providing an empty line
    pub fn daemon(self, trace_folder: &Path) -> Aapt2Daemon {
        Aapt2Daemon::new(trace_folder)
    }
}

pub fn aapt2_tool() -> Result<Command> {
    if let Ok(aapt2) = which::which(bin!("aapt2")) {
        return Ok(Command::new(aapt2));
    } else if let Ok(aapt2) = std::env::var("ANDROID_SDK_ROOT") {
        let aapt2 = PathBuf::from(aapt2);
        let build_tools = aapt2.join("build-tools");
        let target_sdk_version = std::fs::read_dir(&build_tools)
            .map_err(|_| Error::PathNotFound(build_tools.clone()))?
            .filter_map(|path| path.ok())
            .filter(|path| path.path().is_dir())
            .filter_map(|path| path.file_name().into_string().ok())
            .filter(|name| name.chars().next().unwrap().is_digit(10))
            .max()
            .ok_or(AndroidError::BuildToolsNotFound)?;
        let aapt2_exe = build_tools.join(target_sdk_version).join(bin!("aapt2"));
        return Ok(Command::new(aapt2_exe));
    }
    Err(Error::CmdNotFound("aapt2".to_string()))
}
