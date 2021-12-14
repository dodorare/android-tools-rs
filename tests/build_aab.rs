use android_tools::{aapt2::Aapt2, bundletool::BuildBundle};
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
        .join("tests")
        .join("resources")
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
        .join("tests")
        .join("resources")
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
    fs_extra::file::move_file(&path, &manifest_path.join("AndroidManifest.xml"), &options).unwrap();

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
