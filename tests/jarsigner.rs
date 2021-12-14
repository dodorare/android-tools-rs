use android_tools::java_tools::{android_dir, gen_key, AabKey, Jarsigner};

#[test]
fn test_sign_application_with_jarsigner() {
    // Creates a temporary directory
    let tempdir = tempfile::tempdir().unwrap();
    let aab_build_dir = tempdir.path();

    // Generates minimal unsigned aab
    let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let aab_path = user_dirs
        .join("tests")
        .join("resources")
        .join("android_app_bundle")
        .join("minimal_unsigned.aab");
    let cloned_aab = aab_build_dir.join("minimal_unsigned.aab");
    if aab_path.exists() {
        std::fs::copy(&aab_path, &cloned_aab).unwrap();
    }

    // Removes old keystore if it exists
    let android_dir = android_dir().unwrap();
    let target = vec![android_dir.join("aab.keystore")];
    target.iter().for_each(|content| {
        if content.is_file() {
            std::fs::remove_file(&content).unwrap();
        }
    });

    // Creates new keystore to sign aab
    let key = AabKey::default();
    gen_key(key.clone()).unwrap();

    // Signs aab with key
    let signed_aab = Jarsigner::new(&cloned_aab, &key.key_alias)
        .verbose(true)
        .sigalg("SHA256withRSA".to_string())
        .digestalg("SHA-256".to_string())
        .keystore(&key.key_path)
        .storepass(key.key_pass.to_string())
        .run()
        .unwrap();
    assert!(signed_aab.exists());
}
