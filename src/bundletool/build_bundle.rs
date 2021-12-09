use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// ## Build your app bundle using bundletool
/// To build your app bundle, you use the `bundletool build-bundle` command, as shown
/// below:
///
/// ```xml
/// bundletool build-bundle --modules=base.zip --output=mybundle.aab
/// ```
///
/// ## Note
/// If you plan to publish the app bundle, you need to sign it using [`jarsigner`]. You
/// can not use apksigner to sign your app bundle.
///
/// [`jarsigner`]::https://docs.oracle.com/javase/8/docs/technotes/tools/windows/jarsigner.html
#[derive(Debug, PartialEq, PartialOrd)]
pub struct BuildBundle {
    modules: Vec<PathBuf>,
    output: PathBuf,
    config: Option<PathBuf>,
    metadata_file: Option<PathBuf>,
}

impl BuildBundle {
    /// Specifies the list of module ZIP files `bundletool` should use to build your app
    /// bundle.
    ///
    /// Specifies the path and filename for the output `*.aab` file.
    pub fn new(modules: &[PathBuf], output: &Path) -> Self {
        Self {
            modules: modules.to_vec(),
            output: output.to_owned(),
            config: None,
            metadata_file: None,
        }
    }

    /// Specifies the path to an optional configuration file you can use to customize the
    /// build process. To learn more, see the section about [`customizing downstream APK
    /// generation`].
    ///
    /// [`customizing downstream APK generation`]::https://developer.android.com/studio/build/building-cmdline#bundleconfig
    pub fn config(&mut self, config: &Path) -> &mut Self {
        self.config = Some(config.to_owned());
        self
    }

    /// Instructs bundletool to package an optional metadata file inside your app bundle.
    /// You can use this file to include data, such as ProGuard mappings or the complete
    /// list of your app's DEX files, that may be useful to other steps in your toolchain
    /// or an app store.
    ///
    /// `target-bundle-path` specifies a path relative to the root of the app bundle where
    /// you would like the metadata file to be packaged, and `local-file-path` specifies
    /// the path to the local metadata file itself.
    pub fn metadata_file(&mut self, metadata_file: &Path) -> &mut Self {
        self.metadata_file = Some(metadata_file.to_owned());
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut build_bundle = Command::new("java");
        build_bundle.arg("-jar");
        if let Ok(bundletool_path) = std::env::var("BUNDLETOOL_PATH") {
            build_bundle.arg(bundletool_path);
        } else {
            return Err(Error::BundletoolNotFound);
        }
        build_bundle.arg("build-bundle");
        build_bundle.arg("--modules");
        build_bundle.arg(
            self.modules
                .iter()
                .map(|v| v.to_string_lossy().to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
        build_bundle.arg("--output").arg(&self.output);
        if let Some(config) = &self.config {
            build_bundle.arg("--config").arg(config);
        }
        if let Some(metadata_file) = &self.metadata_file {
            build_bundle.arg("--metadata-file").arg(metadata_file);
        }
        build_bundle.output_err(true)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::{aapt2::Aapt2, bundletool::BuildBundle};
    use zip::ZipWriter;
    use zip_extensions::write::ZipWriterExtensions;

    #[test]
    fn test_build_android_app_bundle() {
        // Creates a temporary directory and specify resources
        let tempfile = tempfile::tempdir().unwrap();
        let build_dir = tempfile.path().to_path_buf();

        // Specifies path to needed resources
        let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let res_path = user_dirs
            .join("examples")
            .join("macroquad-3d")
            .join("res")
            .join("android")
            .join("mipmap-hdpi");

        // Compiles resources for aapt2 link
        let aapt2_compile =
            Aapt2.compile_incremental(dunce::simplified(&res_path), dunce::simplified(&build_dir));
        let compiled_res = aapt2_compile.run().unwrap();
        println!("{:?}", compiled_res);

        println!("compiled_res {:?}", compiled_res);

        // Defines path to android manifest
        let manifest_example = user_dirs
            .join("src")
            .join("examples")
            .join("macroquad-3d")
            .join("manifest")
            .join("AndroidManifest.xml");
        let manifest_path = build_dir.join("AndroidManifest.xml");
        std::fs::copy(manifest_example, &manifest_path).unwrap();
        assert!(manifest_path.exists());

        // Assigns configuration to generate APK
        let package_name = "test";
        let target_sdk_version = 30;
        let apk_path = build_dir.join("test.apk");

        // Defines path to  Android SDk and android.jar
        let sdk_path = {
            let sdk_path = std::env::var("ANDROID_SDK_ROOT")
                .ok()
                .or_else(|| std::env::var("ANDROID_SDK_PATH").ok())
                .or_else(|| std::env::var("ANDROID_HOME").ok());
            std::path::PathBuf::from(sdk_path.expect("Android SDK was not found"))
        };
        let platforms_path = sdk_path.join("platforms");
        let platform_dir = platforms_path.join(format!("android-{}", target_sdk_version));
        if !platform_dir.exists() {
            panic!("Platform not found");
        }
        let android_jar = platform_dir.join("android.jar");
        if !android_jar.exists() {
            panic!("Android.jar not found");
        }

        // Links compiled res with specified manifest file and generates an APK
        let gen_apk = Aapt2
            .link_compiled_res(Some(compiled_res), &apk_path, &manifest_path)
            .android_jar(android_jar)
            .verbose(true)
            .proto_format(true)
            .auto_add_overlay(true)
            .run()
            .unwrap();

        // Defines apk path from build directory
        let output_dir = build_dir.join("extracted_files");
        if !output_dir.exists() {
            std::fs::create_dir_all(&output_dir).unwrap();
        }

        // Extracts files from APK to defined output directory and prepares files to generate archive
        let filename = std::path::Path::new(&gen_apk);
        let file = std::fs::File::open(&filename).unwrap();
        let mut apk = zip::ZipArchive::new(file).unwrap();
        apk.extract(&output_dir).unwrap();
        let path = output_dir.join("AndroidManifest.xml");
        let manifest_path = output_dir.join("manifest");
        if !manifest_path.exists() {
            std::fs::create_dir_all(&manifest_path).unwrap();
        }
        let mut options = fs_extra::file::CopyOptions::new();
        options.overwrite = true;
        fs_extra::file::move_file(&path, &manifest_path.join("AndroidManifest.xml"), &options)
            .unwrap();

        // Generates zip archive from extracted files
        let zip_path = build_dir.join("extracted_files.zip");
        let file = std::fs::File::create(&zip_path).unwrap();
        let mut zip = ZipWriter::new(file);
        zip.create_from_directory(&output_dir.to_path_buf())
            .unwrap();
        println!("output_dir {:?}", output_dir);
        println!("zip_path {:?}", zip_path);
        let aab = build_dir.join(format!("{}_unsigned.aab", package_name));

        // Builds app bundle
        BuildBundle::new(&[zip_path], &aab).run().unwrap();
    }
}
