use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Contents options that can help to manage keystore of cryptographic keys
#[derive(Clone, Default)]
pub struct Keytool {
    genkey: bool,
    v: bool,
    keystore: Option<PathBuf>,
    alias: Option<String>,
    keypass: Option<String>,
    storepass: Option<String>,
    dname: Option<Vec<String>>,
    storetype: Option<Storetype>,
    providername: Option<String>,
    providerclass: Option<String>,
    providerarg: Option<PathBuf>,
    protected: bool,
    ext: Option<String>,
    keysize: Option<u32>,
    validity: Option<u32>,
    gencert: bool,
    genkeypair: bool,
    genseckey: bool,
    mportcert: bool,
    importpass: bool,
    importkeystore: bool,
    printcertreq: bool,
    certreq: bool,
    exportcert: bool,
    list: bool,
    printcert: bool,
    printcrl: bool,
    storepasswd: bool,
    keypasswd: bool,
    delete: bool,
    changealias: bool,
    help: bool,
    keyalg: Option<Keyalg>,
}

impl Keytool {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn genkey(&mut self, genkey: bool) -> &mut Self {
        self.genkey = genkey;
        self
    }

    pub fn v(&mut self, v: bool) -> &mut Self {
        self.v = v;
        self
    }

    pub fn keystore(&mut self, keystore: &Path) -> &mut Self {
        self.keystore = Some(keystore.to_owned());
        self
    }

    pub fn alias(&mut self, alias: &str) -> &mut Self {
        self.alias = Some(alias.to_string());
        self
    }

    pub fn keypass(&mut self, keypass: &str) -> &mut Self {
        self.keypass = Some(keypass.to_string());
        self
    }

    pub fn storepass(&mut self, storepass: &str) -> &mut Self {
        self.storepass = Some(storepass.to_string());
        self
    }

    pub fn keyalg(&mut self, keyalg: Keyalg) -> &mut Self {
        self.keyalg = Some(keyalg);
        self
    }

    pub fn dname(&mut self, dname: &[String]) -> &mut Self {
        self.dname = Some(dname.to_vec());
        self
    }

    pub fn storetype(&mut self, storetype: Storetype) -> &mut Self {
        self.storetype = Some(storetype);
        self
    }

    pub fn providername(&mut self, providername: String) -> &mut Self {
        self.providername = Some(providername);
        self
    }

    pub fn providerclass(&mut self, providerclass: String) -> &mut Self {
        self.providerclass = Some(providerclass);
        self
    }

    pub fn providerarg(&mut self, providerarg: &Path) -> &mut Self {
        self.providerarg = Some(providerarg.to_owned());
        self
    }

    pub fn protected(&mut self, protected: bool) -> &mut Self {
        self.protected = protected;
        self
    }

    pub fn ext(&mut self, ext: String) -> &mut Self {
        self.ext = Some(ext);
        self
    }

    pub fn keysize(&mut self, keysize: u32) -> &mut Self {
        self.keysize = Some(keysize);
        self
    }

    pub fn validity(&mut self, validity: u32) -> &mut Self {
        self.validity = Some(validity);
        self
    }

    pub fn gencert(&mut self, gencert: bool) -> &mut Self {
        self.gencert = gencert;
        self
    }

    pub fn genkeypair(&mut self, genkeypair: bool) -> &mut Self {
        self.genkeypair = genkeypair;
        self
    }

    pub fn genseckey(&mut self, genseckey: bool) -> &mut Self {
        self.genseckey = genseckey;
        self
    }

    pub fn mportcert(&mut self, mportcert: bool) -> &mut Self {
        self.mportcert = mportcert;
        self
    }

    pub fn importpass(&mut self, importpass: bool) -> &mut Self {
        self.importpass = importpass;
        self
    }

    pub fn importkeystore(&mut self, importkeystore: bool) -> &mut Self {
        self.importkeystore = importkeystore;
        self
    }
    pub fn printcertreq(&mut self, printcertreq: bool) -> &mut Self {
        self.printcertreq = printcertreq;
        self
    }

    pub fn certreq(&mut self, certreq: bool) -> &mut Self {
        self.certreq = certreq;
        self
    }

    pub fn exportcert(&mut self, exportcert: bool) -> &mut Self {
        self.exportcert = exportcert;
        self
    }

    pub fn list(&mut self, list: bool) -> &mut Self {
        self.list = list;
        self
    }

    pub fn printcert(&mut self, printcert: bool) -> &mut Self {
        self.printcert = printcert;
        self
    }

    pub fn printcrl(&mut self, printcrl: bool) -> &mut Self {
        self.printcrl = printcrl;
        self
    }

    pub fn storepasswd(&mut self, storepasswd: bool) -> &mut Self {
        self.storepasswd = storepasswd;
        self
    }

    pub fn keypasswd(&mut self, keypasswd: bool) -> &mut Self {
        self.keypasswd = keypasswd;
        self
    }

    pub fn delete(&mut self, delete: bool) -> &mut Self {
        self.delete = delete;
        self
    }

    pub fn changealias(&mut self, changealias: bool) -> &mut Self {
        self.changealias = changealias;
        self
    }

    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// Runs keytool commands
    pub fn run(&self) -> Result<()> {
        let mut keytool = keytool()?;
        if self.genkey {
            keytool.arg("-genkey");
        }
        if self.v {
            keytool.arg("-v");
        }
        if let Some(keystore) = &self.keystore {
            keytool.arg("-keystore").arg(keystore);
        }
        if let Some(alias) = &self.alias {
            keytool.arg("-alias").arg(alias);
        }
        if let Some(keypass) = &self.keypass {
            keytool.arg("-keypass").arg(keypass);
        }
        if let Some(storepass) = &self.storepass {
            keytool.arg("-storepass").arg(storepass);
        }
        if let Some(dname) = &self.dname {
            keytool.arg("-dname").arg(
                dname
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(storetype) = &self.storetype {
            keytool.arg("-storetype").arg(storetype.to_string());
        }
        if let Some(keyalg) = &self.keyalg {
            keytool.arg("-keyalg").arg(keyalg.to_string());
        }
        if let Some(providername) = &self.providername {
            keytool.arg("-providername").arg(providername);
        }
        if let Some(providerclass) = &self.providerclass {
            keytool.arg("-providerclass").arg(providerclass);
        }
        if let Some(providerarg) = &self.providerarg {
            keytool.arg("-providerarg").arg(providerarg);
        }
        if self.protected {
            keytool.arg("-protected");
        }
        if let Some(ext) = &self.ext {
            keytool.arg("-ext").arg(ext);
        }
        if let Some(keysize) = &self.keysize {
            keytool.arg("-keysize").arg(keysize.to_string());
        }
        if let Some(validity) = &self.validity {
            keytool.arg("-validity").arg(validity.to_string());
        }
        if self.gencert {
            keytool.arg("-gencert");
        }
        if self.genkeypair {
            keytool.arg("-genkeypair");
        }
        if self.genseckey {
            keytool.arg("-genseckey");
        }
        if self.mportcert {
            keytool.arg("-mportcert");
        }
        if self.importpass {
            keytool.arg("-importpass");
        }
        if self.importkeystore {
            keytool.arg("-importkeystore");
        }
        if self.printcertreq {
            keytool.arg("-printcertreq");
        }
        if self.certreq {
            keytool.arg("-certreq");
        }
        if self.exportcert {
            keytool.arg("-exportcert");
        }
        if self.list {
            keytool.arg("-list");
        }
        if self.printcert {
            keytool.arg("-printcert");
        }
        if self.printcrl {
            keytool.arg("-printcrl");
        }
        if self.storepasswd {
            keytool.arg("-storepasswd");
        }
        if self.keypasswd {
            keytool.arg("-keypasswd");
        }
        if self.delete {
            keytool.arg("-delete");
        }
        if self.changealias {
            keytool.arg("-changealias");
        }
        if self.help {
            keytool.arg("-help");
        }
        keytool.output_err(true)?;
        Ok(())
    }
}

/// The `keytool` command is a key and certificate management utility. It enables users to
/// administer their own public/private key pairs and associated certificates for use in
/// self-authentication (where the user authenticates himself or herself to other users
/// and services) or data integrity and authentication services, using digital signatures.
/// The `keytool` command also enables users to cache the public keys (in the form of
/// certificates) of their communicating peers
pub fn keytool() -> Result<Command> {
    if let Ok(keytool) = which::which(bin!("keytool")) {
        return Ok(Command::new(keytool));
    }
    if let Ok(java) = std::env::var("JAVA_HOME") {
        let keytool = PathBuf::from(java).join("bin").join(bin!("keytool"));
        if keytool.exists() {
            return Ok(Command::new(keytool));
        }
    }
    Err(Error::CmdNotFound("keytool".to_string()))
}

#[derive(Clone)]
pub enum Storetype {
    JKS,
    /// This keystore implementation employs a much stronger protection of private
    /// keys (using password-based encryption with Triple DES) than JKS. You can
    /// upgrade your keystore of type "JKS" to type "JCEKS" by changing the password
    /// of a private-key entry in your keystore.
    JCEKS,
    /// There is a difference between PKCS12 type keystore created on the keytool
    /// provided in the IBM JVM and the keytool provided in an Oracle JVM. The keytool
    /// in an IBM JVM uses a PKCS12 keystore to store both key entries and certificate
    /// entries.The keytool in an Oracle JVM uses a PKCS12 keystore to store key entries.
    /// The keytool program in IBM's JVM can read the keystore created by the keytool
    /// program provided by an Oracle JVM, but not the other way around.
    PKCS12,
    /// This is a second version of `PKCS12` type `keystore`. It can be read by the keytool
    /// program in an `Oracle JVM`.
    PKCS12S2,
    /// This is a RACF® keyring keystore. This type is available only on z/OS® systems
    /// with RACF installed. When using JCERACFKS keystore, you must always specify the
    /// `-keystore` option. This is no default value.
    JCERACFKS,
}

#[derive(Clone)]
pub enum Keyalg {
    RSA,
    /// This keystore implementation employs a much stronger protection of private
    /// keys (using password-based encryption with Triple DES) than JKS. You can
    /// upgrade your keystore of type "JKS" to type "JCEKS" by changing the password
    /// of a private-key entry in your keystore.
    DSA,
    /// There is a difference between PKCS12 type keystore created on the keytool
    /// provided in the IBM JVM and the keytool provided in an Oracle JVM. The keytool
    /// in an IBM JVM uses a PKCS12 keystore to store both key entries and certificate
    /// entries.The keytool in an Oracle JVM uses a PKCS12 keystore to store key entries.
    /// The keytool program in IBM's JVM can read the keystore created by the keytool
    /// program provided by an Oracle JVM, but not the other way around.
    EC,
    /// This is a second version of `PKCS12` type `keystore`. It can be read by the keytool
    /// program in an `Oracle JVM`.
    DES,
    /// This is a RACF® keyring keystore. This type is available only on z/OS® systems
    /// with RACF installed. When using JCERACFKS keystore, you must always specify the
    /// `-keystore` option. This is no default value.
    DESede,
}

impl std::fmt::Display for Storetype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::JKS => write!(f, "JKS"),
            Self::JCEKS => write!(f, "JCEKS"),
            Self::PKCS12 => write!(f, "PKCS12"),
            Self::PKCS12S2 => write!(f, "PKCS12S2"),
            Self::JCERACFKS => write!(f, "JCERACFKS"),
        }
    }
}

impl std::fmt::Display for Keyalg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::RSA => write!(f, "RSA"),
            Self::DSA => write!(f, "DSA"),
            Self::EC => write!(f, "EC"),
            Self::DES => write!(f, "DES"),
            Self::DESede => write!(f, "DESede"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AabKey {
    pub key_path: PathBuf,
    pub key_pass: String,
    pub key_alias: String,
}

impl Default for AabKey {
    fn default() -> Self {
        let key_path = android_dir().unwrap().join("aab.keystore");
        let key_pass = "android".to_string();
        let key_alias = "androidaabkey".to_string();
        Self {
            key_path,
            key_pass,
            key_alias,
        }
    }
}

/// Returns the path to `android` directory created in the user's home directory
pub fn android_dir() -> Result<PathBuf> {
    let android_dir = dirs::home_dir()
        .ok_or_else(|| Error::PathNotFound(PathBuf::from("$HOME")))?
        .join(".android");
    std::fs::create_dir_all(&android_dir)?;
    Ok(android_dir)
}
