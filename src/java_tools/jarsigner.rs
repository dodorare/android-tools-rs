use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Signs and verifies Java Archive (JAR) files
#[derive(Clone, Default)]
pub struct Jarsigner {
    verify: bool,
    jar_file: PathBuf,
    alias: String,
    keystore: Option<PathBuf>,
    storepass: Option<String>,
    storetype: Option<String>,
    keypass: Option<String>,
    certchain: Option<PathBuf>,
    sigfile: Option<PathBuf>,
    signedjar: Option<PathBuf>,
    digestalg: Option<String>,
    sigalg: Option<String>,
    verbose: bool,
    certs: bool,
    rev_check: bool,
    tsa: Option<PathBuf>,
    tsacert: Option<String>,
    tsapolicyid: Option<String>,
    tsadigestalg: Option<String>,
    altsigner: Option<PathBuf>,
    altsignerpath: Option<Vec<PathBuf>>,
    internalsf: bool,
    sectionsonly: bool,
    protected: bool,
    provider_name: Option<String>,
    addprovider: Option<String>,
    provider_class: Option<String>,
    provider_arg: Option<PathBuf>,
    strict: bool,
    conf: Option<PathBuf>,
    h: bool,
    help: bool,
}

impl Jarsigner {
    /// ## JarFile
    /// The JAR file to be signed.
    ///
    /// If you also specified the -strict option, and the jarsigner command detected
    /// severe warnings, the message, "jar signed, with signer errors" is displayed.
    ///
    /// ## Alias
    /// The aliases are defined in the keystore specified by `-keystore` or the default
    /// keystore.
    pub fn new(jar_file: &Path, alias: &str) -> Self {
        Self {
            jar_file: jar_file.to_owned(),
            alias: alias.to_owned(),
            ..Default::default()
        }
    }

    /// Specifies the URL that tells the keystore location. This defaults to the file
    /// `.keystore` in the user's home directory, as determined by the `user.home` system
    /// property. A keystore is required when signing. You must explicitly specify a
    /// keystore when the default keystore does not exist or if you want to use one other
    /// than the default. A keystore is not required when verifying, but if one is
    /// specified or the default exists and the `-verbose` option was also specified, then
    /// additional information is output regarding whether or not any of the certificates
    /// used to verify the JAR file are contained in that keystore. The `-keystore` argument
    /// can be a file name and path specification rather than a URL, in which case it is
    /// treated the same as a file: URL, for example, the following are equivalent:
    ///
    /// ```sh
    /// * `-keystore filePathAndName`
    /// * `-keystore file:filePathAndName`
    /// ```
    ///
    /// If the Sun PKCS #11 provider was configured in the java.security security
    /// properties file (located in the JRE's `$JAVA_HOME/lib/security directory`), then the
    /// keytool and jarsigner tools can operate on the PKCS #11 token by specifying these
    /// options:
    ///
    /// ```sh
    /// * `-keystore NONE`
    /// * `-storetype PKCS11`
    /// ```
    ///
    /// For example, the following command lists the contents of the configured PKCS#11
    /// token:
    ///
    /// ```sh
    /// * `keytool -keystore NONE -storetype PKCS11 -list`
    /// ```
    pub fn keystore(&mut self, keystore: &Path) -> &mut Self {
        self.keystore = Some(keystore.to_owned());
        self
    }

    /// Specifies the password that is required to access the keystore. This is only
    /// needed when signing (not verifying) a JAR file. In that case, if a `-storepass`
    /// option is not provided at the command line, then the user is prompted for the
    /// password. If the modifier env or file is not specified, then the password has the
    /// value argument. Otherwise, the password is retrieved as follows:
    ///
    /// * `env:` Retrieve the password from the environment variable named argument
    /// * `file:` Retrieve the password from the file named argument
    pub fn storepass(&mut self, storepass: String) -> &mut Self {
        self.storepass = Some(storepass);
        self
    }

    /// Specifies the type of keystore to be instantiated. The default keystore type is
    /// the one that is specified as the value of the `keystore.type` property in the
    /// security properties file, which is returned by the static `getDefaultType` method
    /// in `java.security.KeyStore`. The PIN for a PCKS #11 token can also be
    /// specified with the `-storepass` option. If none is specified, then the `keytool`
    /// and `jarsigner` commands prompt for the token PIN. If the token has a protected
    /// authentication path (such as a dedicated PIN-pad or a biometric reader), then
    /// the `-protected` option must be specified and no password options can be
    /// specified   
    pub fn storetype(&mut self, storetype: String) -> &mut Self {
        self.storetype = Some(storetype);
        self
    }

    /// Specifies the password used to protect the private key of the keystore entry
    /// addressed by the alias specified on the command line. The password is required
    /// when using jarsigner to sign a JAR file. If no password is provided on the command
    /// line, and the required password is different from the store password, then the
    /// user is prompted for it
    ///
    /// If the modifier env or file is not specified, then the password has the value
    /// argument. Otherwise, the password is retrieved as follows:
    ///
    /// * `env:` Retrieve the password from the environment variable named argument
    /// * `file:` Retrieve the password from the file named argument
    ///
    /// ## Note
    /// The password should not be specified on the command line or in a script unless it
    /// is for testing purposes, or you are on a secure system
    pub fn keypass(&mut self, keypass: String) -> &mut Self {
        self.keypass = Some(keypass);
        self
    }

    /// Specifies the certificate chain to be used when the certificate chain associated
    /// with the private key of the keystore entry that is addressed by the alias
    /// specified on the command line is not complete. This can happen when the keystore
    /// is located on a hardware token where there is not enough capacity to hold  a
    /// complete certificate chain. The file can be a sequence of concatenated X.509
    /// certificates, or a single PKCS#7 formatted data block, either in binary encoding
    /// format or in printable encoding format (also known as Base64 encoding) as defined
    /// by the Internet RFC 1421 standard
    pub fn certchain(&mut self, certchain: &Path) -> &mut Self {
        self.certchain = Some(certchain.to_owned());
        self
    }

    /// Specifies the base file name to be used for the generated `.SF` and `.DSA` files. For
    /// example, if file is DUKESIGN, then the generated .SF and .DSA files are named
    /// `DUKESIGN.SF` and `DUKESIGN.DSA`, and placed in the META-INF directory of the signed
    /// JAR file
    ///
    /// The characters in the file must come from the set a-zA-Z0-9_-. Only
    /// letters, numbers, underscore, and hyphen characters are allowed. All lowercase
    /// characters are converted to uppercase for the .SF and .DSA file names
    ///
    /// If no -sigfile option appears on the command line, then the base file name for the
    /// `.SF` and `.DSA` files is the first 8 characters of the alias name specified on
    /// the command line, all converted to upper case. If the alias name has fewer
    /// than 8 characters, then the full alias name is used. If the alias name
    /// contains any characters that are not valid in a signature file name, then each
    /// such character is converted to an underscore (_) character to form the file
    /// name
    pub fn sigfile(&mut self, sigfile: &Path) -> &mut Self {
        self.sigfile = Some(sigfile.to_owned());
        self
    }

    /// Name of signed JAR file
    pub fn signedjar(&mut self, signedjar: &Path) -> &mut Self {
        self.signedjar = Some(signedjar.to_owned());
        self
    }

    /// Name of digest algorithm
    pub fn digestalg(&mut self, digestalg: String) -> &mut Self {
        self.digestalg = Some(digestalg);
        self
    }

    /// Specifies the name of the signature algorithm to use to sign the JAR file
    ///
    /// For a list of standard signature algorithm names, see "Appendix A: Standard Names"
    /// in the Java Cryptography Architecture (JCA) Reference Guide at http://docs.oracle.com/javase/8/docs/technotes/guides/security/crypto/CryptoSpec.html#AppA
    ///
    /// This algorithm must be compatible with the private key used to sign the JAR file.
    /// If this option is not specified, then `SHA1withDSA`, `SHA256withRSA`, or
    /// `SHA256withECDSA` are used depending on the type of private key. There must either
    /// be a statically installed provider supplying an implementation of the specified
    /// algorithm or the user must specify one with the `-providerClass` option; otherwise,
    /// the command will not succeed
    pub fn sigalg(&mut self, sigalg: String) -> &mut Self {
        self.sigalg = Some(sigalg);
        self
    }

    /// When the `-verbose` option appears on the command line, it indicates verbose mode,
    /// which causes jarsigner to output extra information about the progress of the JAR
    /// signing or verification
    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose = verbose;
        self
    }

    /// If the `-certs` option appears on the command line with the `-verify` and `-verbose`
    /// options, then the output includes certificate information for each signer of the
    /// JAR file. This information includes the name of the type of certificate (stored in
    /// the `.DSA` file) that certifies the signer's public key, and if the certificate is
    /// an X.509 certificate (an instance of the `java.security.cert.X509Certificate`), then
    /// the distinguished name of the signer
    ///
    /// The keystore is also examined. If no keystore value is specified on the command
    /// line, then the default keystore file (if any) is checked. If the public key
    /// certificate for a signer matches an entry in the keystore, then the alias name for
    /// the keystore entry for that signer is displayed in parentheses
    pub fn certs(&mut self, certs: bool) -> &mut Self {
        self.certs = certs;
        self
    }

    /// Enable certificate revocation check
    pub fn rev_check(&mut self, rev_check: bool) -> &mut Self {
        self.rev_check = rev_check;
        self
    }

    /// If [-tsa](http://example.tsa.url) appears on the command line when signing a JAR file
    /// then a time stamp is generated for the signature. The URL,
    /// identifies the location of the Time Stamping Authority (TSA) and overrides any URL
    /// found with the -tsacert option. The `-tsa` option does not require the TSA public
    /// key certificate to be present in the keystore
    ///
    /// To generate the time stamp, jarsigner communicates with the TSA with the
    /// Time-Stamp Protocol (TSP) defined in RFC 3161. When successful, the time stamp
    /// token returned by the TSA is stored with the signature in the signature block
    /// file
    pub fn tsa(&mut self, tsa: &Path) -> &mut Self {
        self.tsa = Some(tsa.to_owned());
        self
    }

    /// When `-tsacert` alias appears on the command line when signing a JAR file, a time
    /// stamp is generated for the signature. The alias identifies the TSA public key
    /// certificate in the keystore that is in effect. The entry's certificate is examined
    /// for a Subject Information Access extension that contains a URL identifying the
    /// location of the TSA
    ///
    /// The TSA public key certificate must be present in the keystore when using the
    /// `-tsacert` option
    pub fn tsacert(&mut self, tsacert: String) -> &mut Self {
        self.tsacert = Some(tsacert);
        self
    }

    /// TSAPolicyID for Timestamping Authority
    pub fn tsapolicyid(&mut self, tsapolicyid: String) -> &mut Self {
        self.tsapolicyid = Some(tsapolicyid);
        self
    }

    /// Algorithm of digest data in timestamping request
    pub fn tsadigestalg(&mut self, tsadigestalg: String) -> &mut Self {
        self.tsadigestalg = Some(tsadigestalg);
        self
    }

    /// Class name of an alternative signing mechanism (This option is deprecated and will
    /// be removed in a future release.)
    pub fn altsigner(&mut self, altsigner: &Path) -> &mut Self {
        self.altsigner = Some(altsigner.to_owned());
        self
    }

    /// Location of an alternative signing mechanism (This option is deprecated and will
    /// be removed in a future release.)
    pub fn altsignerpath(&mut self, altsignerpath: &[PathBuf]) -> &mut Self {
        self.altsignerpath = Some(altsignerpath.to_owned());
        self
    }

    /// Include the `.SF` file inside the signature block
    pub fn internalsf(&mut self, internalsf: bool) -> &mut Self {
        self.internalsf = internalsf;
        self
    }

    /// Don't compute hash of entire manifest
    pub fn sectionsonly(&mut self, sectionsonly: bool) -> &mut Self {
        self.sectionsonly = sectionsonly;
        self
    }

    /// Keystore has protected authentication path
    pub fn protected(&mut self, protected: bool) -> &mut Self {
        self.protected = protected;
        self
    }

    /// Provider name
    pub fn provider_name(&mut self, provider_name: String) -> &mut Self {
        self.provider_name = Some(provider_name);
        self
    }

    /// Add security provider by name (e.g. SunPKCS11)
    /// add security provider by fully-qualified class name
    pub fn addprovider(&mut self, addprovider: String) -> &mut Self {
        self.addprovider = Some(addprovider);
        self
    }

    /// Configure argument for -addprovider
    pub fn provider_class(&mut self, provider_class: String) -> &mut Self {
        self.provider_class = Some(provider_class);
        self
    }

    /// Configure argument for `-providerClass`
    pub fn provider_arg(&mut self, provider_arg: &Path) -> &mut Self {
        self.provider_arg = Some(provider_arg.to_owned());
        self
    }

    /// Treat warnings as errors
    pub fn strict(&mut self, strict: bool) -> &mut Self {
        self.strict = strict;
        self
    }

    /// Specify a pre-configured options file
    pub fn conf(&mut self, conf: &Path) -> &mut Self {
        self.conf = Some(conf.to_owned());
        self
    }

    /// The `-verify` option can take zero or more keystore alias names after the JAR file
    /// name. When the `-verify` option is specified, the jarsigner command checks that the
    /// certificate used to verify each signed entry in the JAR file matches one of the
    /// keystore aliases. The aliases are defined in the keystore specified by `-keystore`
    /// or the default keystore.
    ///
    /// If you also specified the `-strict` option, and the jarsigner command detected
    /// severe warnings, the message, "jar verified, with signer errors" is displayed
    pub fn verify(&mut self, verify: bool) -> &mut Self {
        self.verify = verify;
        self
    }

    /// Print this help message
    pub fn h(&mut self, h: bool) -> &mut Self {
        self.h = h;
        self
    }

    /// Print this help message
    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// Runs jarsigner commands and signa JAR file with arguments
    pub fn run(&self) -> Result<PathBuf> {
        let mut jarsigner = jarsigner_tool()?;
        if self.verify {
            jarsigner.arg("-verify");
        }
        jarsigner.arg(&self.jar_file);
        jarsigner.arg(&self.alias);
        if let Some(keystore) = &self.keystore {
            jarsigner.arg("-keystore").arg(keystore);
        }
        if let Some(storepass) = &self.storepass {
            jarsigner.arg("-storepass").arg(storepass);
        }
        if let Some(storetype) = &self.storetype {
            jarsigner.arg("-storetype").arg(storetype);
        }
        if let Some(keypass) = &self.keypass {
            jarsigner.arg("-keypass").arg(keypass);
        }
        if let Some(certchain) = &self.certchain {
            jarsigner.arg("-certchain").arg(certchain);
        }
        if let Some(sigfile) = &self.sigfile {
            jarsigner.arg("-sigfile").arg(sigfile);
        }
        if let Some(signedjar) = &self.signedjar {
            jarsigner.arg("-signedjar").arg(signedjar);
        }
        if let Some(digestalg) = &self.digestalg {
            jarsigner.arg("-digestalg").arg(digestalg);
        }
        if let Some(sigalg) = &self.sigalg {
            jarsigner.arg("-sigalg").arg(sigalg);
        }
        if self.verbose {
            jarsigner.arg("-verbose");
        }
        if self.certs {
            jarsigner.arg("-certs");
        }
        if self.rev_check {
            jarsigner.arg("-revCheck");
        }
        if let Some(tsa) = &self.tsa {
            jarsigner.arg("-tsa").arg(tsa);
        }
        if let Some(tsacert) = &self.tsacert {
            jarsigner.arg("-tsacert").arg(tsacert);
        }
        if let Some(tsapolicyid) = &self.tsapolicyid {
            jarsigner.arg("-tsapolicyid").arg(tsapolicyid);
        }
        if let Some(tsadigestalg) = &self.tsadigestalg {
            jarsigner.arg("-tsadigestalg").arg(tsadigestalg);
        }
        if let Some(altsigner) = &self.altsigner {
            jarsigner.arg("-altsigner").arg(altsigner);
        }
        if let Some(altsignerpath) = &self.altsignerpath {
            jarsigner.arg("-altsigner").arg(
                altsignerpath
                    .iter()
                    .map(|v| v.to_string_lossy().to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if self.internalsf {
            jarsigner.arg("-internalsf");
        }
        if self.sectionsonly {
            jarsigner.arg("-sectionsonly");
        }
        if self.protected {
            jarsigner.arg("-protected");
        }
        if let Some(provider_name) = &self.provider_name {
            jarsigner.arg("-providerName").arg(provider_name);
        }
        if let Some(addprovider) = &self.addprovider {
            jarsigner.arg("-addprovider").arg(addprovider);
        }
        if let Some(provider_class) = &self.provider_class {
            jarsigner.arg("-providerClass").arg(provider_class);
        }
        if let Some(provider_arg) = &self.provider_arg {
            jarsigner.arg("-providerArg").arg(provider_arg);
        }
        if self.strict {
            jarsigner.arg("-strict");
        }
        if let Some(conf) = &self.conf {
            jarsigner.arg("-conf").arg(conf);
        }
        if self.h {
            jarsigner.arg("-h");
        }
        if self.help {
            jarsigner.arg("-help");
        }
        jarsigner.output_err(true)?;
        Ok(self.jar_file.clone())
    }
}

/// Signs and verifies `.aab` and Java Archive (JAR) files
fn jarsigner_tool() -> Result<Command> {
    if let Ok(jarsigner) = which::which(bin!("jarsigner")) {
        return Ok(Command::new(jarsigner));
    }
    if let Ok(java) = std::env::var("JAVA_HOME") {
        let keytool = PathBuf::from(java).join("bin").join(bin!("jarsigner.exe"));
        if keytool.exists() {
            return Ok(Command::new(keytool));
        }
    }
    Err(Error::CmdNotFound("jarsigner".to_string()))
}
