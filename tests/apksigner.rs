use android_tools::java_tools::{android_dir, AabKey, Apksigner, KeyAlgorithm, Keytool};

#[test]
fn test_apksigner() {
    // Specifies path to resources
    let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Defines path to android manifest needed to aapt2 link
    let apk_path = user_dirs
        .join("tests")
        .join("resources")
        .join("android_app_bundle")
        .join("test.apk");

    // Removes old keystore if it exists
    let android_dir = android_dir().unwrap();
    let target = vec![android_dir.join("debug.keystore")];
    target.iter().for_each(|content| {
        if content.is_file() {
            std::fs::remove_file(&content).unwrap();
        }
    });

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
