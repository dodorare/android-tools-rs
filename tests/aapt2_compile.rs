use android_tools::aapt2::Aapt2;

#[test]
/// Testing [`AAPT2`] compile. This tool needed to compile resources.
/// [`AAPT2`] supports compilation of all Android resource types, such as drawables and XML files.
/// [`AAPT2`](https://developer.android.com/studio/command-line/aapt2) then parses the file and
/// generates an intermediate binary file with a `.flat` extension.
fn test_compile_resources_with_aapt2() {
    // Creates a temporary directory and specify resources.
    let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let res_path = user_dirs
        .join("tests")
        .join("resources")
        .join("res")
        .join("android")
        .join("mipmap-hdpi");
    let tempfile = tempfile::tempdir().unwrap();
    let compiled_res_dir = tempfile.path().to_path_buf();
    assert!(compiled_res_dir.exists());

    // Compiles resources
    let compiled_res = Aapt2
        .compile_incremental(&res_path, &compiled_res_dir)
        .run()
        .unwrap();
    assert!(compiled_res.exists());
}
