use android_tools::bundletool::{BuildApks, GetSizeTotal};

#[test]
fn test_build_apks_from_aab() {
    // Creates a temporary directory
    let tempfile = tempfile::tempdir().unwrap();
    let build_dir = tempfile.path().to_path_buf();

    // Assigns configuratin to generate aab
    let package_name = "test";
    assert!(build_dir.exists());

    // Specifies path to minimal unsigned aab
    let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let aab_path = user_dirs
        .join("tests")
        .join("resources")
        .join("android_app_bundle")
        .join("minimal_unsigned.aab");

    // Test build_apks
    let apks_path = build_dir.join(format!("{}.apks", package_name));
    let apks = BuildApks::new(&aab_path, &apks_path).run().unwrap();
    GetSizeTotal::new(&apks).run().unwrap();
}
