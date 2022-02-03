use android_tools::emulator::EmulatorTools;

#[test]
fn test_emulator() {
    EmulatorTools::new()
        .version(true)
        .help_all(true)
        .run()
        .unwrap();
}
