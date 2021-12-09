mod jarsigner;
mod keytool;

pub use jarsigner::*;
pub use keytool::*;

use std::path::Path;

#[derive(Clone, Copy)]
pub struct JavaTools;

impl JavaTools {
    pub fn jarsigner(self, jar_file: &Path, alias: &str) -> Jarsigner {
        Jarsigner::new(jar_file, alias)
    }
}
