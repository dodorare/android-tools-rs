use android_tools::bundletool::{BuildApks, GetSizeTotal};

#[test]
/// When `bundletool` generates APKs from your app bundle, it includes them in a container
/// called an APK set archive, which uses the `.apks` file extension. To generate an APK
/// set for all device configurations your app supports from your app bundle, use the bundletool
/// [`build-apks`] command
///
/// [build-apks]: https://developer.android.com/studio/command-line/bundletool#generate_apks
fn test_build_apks_from_aab() {
    // Creates a temporary directory that will be dropped after test finished
    let tempfile = tempfile::tempdir().unwrap();
    let build_dir = tempfile.path().to_path_buf();

    // Assigns configuration to generate AAB
    let package_name = "test";
    assert!(build_dir.exists());

    // Specifies path to minimal unsigned AAB
    let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let aab_path = user_dirs
        .join("tests")
        .join("resources")
        .join("android_app_bundle")
        .join("minimal_unsigned.aab");

    // Builds apks file from generated AAB
    let apks_path = build_dir.join(format!("{}.apks", package_name));
    let apks = BuildApks::new(&aab_path, &apks_path).run().unwrap();
    GetSizeTotal::new(&apks).run().unwrap();
}
