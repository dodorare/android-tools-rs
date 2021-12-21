use android_tools::java_tools::{android_dir, AabKey, JarSigner, KeyAlgorithm, Keytool};

#[test]
/// The [`jarsigner`] tool has two purposes:
/// * To sign Java Archive (JAR) files.
/// * To verify the signatures and integrity of signed JAR files.
///
/// The JAR feature enables the packaging of class files, images, sounds, and other digital data
/// in a single file for faster and easier distribution. A tool named jar enables developers to
/// produce JAR files. (Technically, any zip file can also be considered a JAR file, although when
/// created by the jar command or processed by the jarsigner command, JAR files also contain a
/// META-INF/MANIFEST.MF file.)
///
/// [jarsigner]: https://docs.oracle.com/javase/7/docs/technotes/tools/windows/jarsigner.html
fn test_sign_application_with_jarsigner() {
    // Creates a temporary directory that will be dropped after test finished
    let tempdir = tempfile::tempdir().unwrap();
    let aab_build_dir = tempdir.path();

    // Generates minimal unsigned AAB that will be signed later
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

    // Signs AAB with key
    let signed_aab = JarSigner::new(&cloned_aab, &key.key_alias)
        .verbose(true)
        .sigalg("SHA256withRSA".to_string())
        .digestalg("SHA-256".to_string())
        .keystore(&key.key_path)
        .storepass(key.key_pass.to_string())
        .run()
        .unwrap();
    assert!(signed_aab.exists());
}
