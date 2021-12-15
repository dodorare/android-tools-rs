use android_tools::java_tools::{android_dir, gen_key, AabKey};

#[test]
/// The [`keytool`] command is a key and certificate management utility. It enables users to administer
/// their own public/private key pairs and associated certificates for use in self-authentication
/// (where the user authenticates himself or herself to other users and services) or data integrity
/// and authentication services, using digital signatures.
///
/// [keytool]: https://docs.oracle.com/javase/9/tools/keytool.htm#JSWOR-GUID-5990A2E4-78E3-47B7-AE75-6D1826259549
fn test_create_keystore_with_keytool() {
    // Creates a temporary directory that will be dropped after test finished
    let tempdir = tempfile::tempdir().unwrap();
    let aab_build_dir = tempdir.path();

    // Generates minimal unsigned AAB
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

    // Creates new keystore from keytool
    let key = AabKey::default();
    gen_key(key).unwrap();
}
