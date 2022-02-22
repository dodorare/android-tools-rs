mod apksigner;
mod jarsigner;
mod keytool;

pub use apksigner::*;
pub use jarsigner::*;
pub use keytool::*;

use std::path::Path;

/// Tools that using to create keystore and sign JAR files with keystore
#[derive(Clone, Copy)]
pub struct JavaTools;

impl JavaTools {
    /// Invocates jarsigner to sign JAR file. You can use it to sign APK and AAB files too
    pub fn jarsigner(self, jar_file: &Path, alias: &str) -> JarSigner {
        JarSigner::new(jar_file, alias)
    }

    /// Invocates keytool options to create and manage keystore
    pub fn keytool(self) -> Keytool {
        Keytool::new()
    }
}
