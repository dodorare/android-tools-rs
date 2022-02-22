use android_tools::adb::AdbTools;

#[test]
fn test_adb() {
    AdbTools::new().version(true).help(true).run().unwrap();
}
