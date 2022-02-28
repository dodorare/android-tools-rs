use android_tools::{java_tools::{Apksigner, AabKey, Keytool, KeyAlgorithm}, aapt2::Aapt2, sdk_path_from_env};

#[test]
fn test_apksigner() {
    // Creates a temporary directory that will be dropped after test finished
    let tempfile = tempfile::tempdir().unwrap();
    let tempdir = tempfile.path().to_path_buf();

    // Specifies path to resources
    let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let res_path = user_dirs
        .join("tests")
        .join("resources")
        .join("res")
        .join("android")
        .join("mipmap-hdpi");

    // Compiles resources
    let compiled_res_path = tempdir.join("compiled_res");
    if !compiled_res_path.exists() {
        std::fs::create_dir_all(&compiled_res_path).unwrap();
    }
    let aapt2_compile = Aapt2.compile_incremental(
        dunce::simplified(&res_path),
        dunce::simplified(&compiled_res_path),
    );
    let compiled_res = aapt2_compile.run().unwrap();

    // Defines path to android manifest needed to aapt2 link
    let manifest_path = user_dirs
        .join("tests")
        .join("resources")
        .join("manifest")
        .join("AndroidManifest.xml");
    assert!(manifest_path.exists());

    // Defines apk path and name
    let target_sdk_version = 30;
    let apk_path = tempdir.join("test.apk");

    // Defines path to Android SDK tools
    let sdk_path = sdk_path_from_env().unwrap();
    let platforms_path = sdk_path.join("platforms");
    let platform_dir = platforms_path.join(format!("android-{}", target_sdk_version));
    if !platform_dir.exists() {
        panic!("Platform not found");
    }
    let android_jar = platform_dir.join("android.jar");
    if !android_jar.exists() {
        panic!("Android.jar not found");
    }

    // Links compiled resources in `.flat` extension with specified manifest file and generates an APK
    let mut aapt2_link = Aapt2.link_compiled_res(Some(compiled_res), &apk_path, &manifest_path);
    aapt2_link.android_jar(android_jar).verbose(true);
    aapt2_link.run().unwrap();

    // Creates new keystore to sign aab
    let key = AabKey::new_default().unwrap();
    Keytool::new()
        .genkeypair(true)
        .v(true)
        .keystore(&key.key_path)
        .alias(&key.key_alias)
        .keypass(&key.key_pass)
        .storepass(&key.key_pass)
        .dname(&["CN=Android Debug,O=Android,C=US".to_owned()])
        .keyalg(KeyAlgorithm::RSA)
        .keysize(2048)
        .validity(10000)
        .run()
        .unwrap();

    Apksigner::new()
    .sign(true)
    .ks(&key.key_path)
    .ks_pass(format!("pass:{}", &key.key_pass))
    .apk_path(&apk_path)
    .run()
    .unwrap();
}
