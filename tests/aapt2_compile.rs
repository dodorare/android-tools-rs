use android_tools::aapt2::Aapt2;

#[test]
/// [`AAPT2`] supports compilation of all Android resource types, such as drawables and XML files.
/// [`AAPT2`] then parses the file and
/// generates an intermediate binary file with a `.flat` extension. Notice, that aapt2 compile
/// sensitive to path to your resources needed to compile. You need to put resources into specified directories.
/// Use article [Resource types](https://developer.android.com/guide/topics/resources/available-resources)
///
/// [AAPT2]: https://developer.android.com/studio/command-line/aapt2
fn test_compile_resources_with_aapt2() {
    // Creates a temporary directory that will be dropped after test finished and specifies resources from tests/resources.
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

    // Compiles files by iterate all resources found in target directory into files with `.flat` extension
    let compiled_res = Aapt2
        .compile_incremental(&res_path, &compiled_res_dir)
        .run()
        .unwrap();
    assert!(compiled_res.exists());
}
