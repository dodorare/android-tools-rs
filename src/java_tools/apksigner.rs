use crate::{error::*, sdk_path_from_env};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Default)]
pub struct Apksigner {
    out: Option<PathBuf>,
    min_sdk_version: Option<u32>,
    max_sdk_version: Option<u32>,
    v1_signing_enabled: bool,
    v2_signing_enabled: bool,
    v3_signing_enabled: bool,
    v4_signing_enabled: Option<V4SigningEnabled>,
    v: bool,
    sign: bool,
    verify: Option<PathBuf>,
    verbose: bool,
    next_signer: bool,
    v1_signer_name: Option<String>,
    ks: Option<PathBuf>,
    ks_key_alias: Option<String>,
    ks_pass: Option<String>,
    pass_encoding: Option<String>,
    key_pass: Option<String>,
    ks_type: Option<String>,
    ks_provider_name: Option<String>,
    ks_provider_class: Option<String>,
    ks_provider_arg: Option<String>,
    key: Option<PathBuf>,
    cert: Option<String>,
    print_certs: bool,
    werr: bool,
    help: bool,
    apk_path: Option<PathBuf>,
}

impl Apksigner {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// The location where you'd like to save the signed APK. If this option
    /// isn't provided explicitly, the APK package is signed in-place,
    /// overwriting the input APK file
    pub fn out(&mut self, out: &Path) -> &mut Self {
        self.out = Some(out.to_owned());
        self
    }

    /// The lowest Android framework API level that `apksigner` uses to confirm
    /// that the APK's signature will be verified. Higher values allow the
    /// tool to use stronger security parameters when signing the app but
    /// limit the APK's availability to devices running more recent versions
    /// of Android. By default, apksigner uses the value of the `minSdkVersion`
    /// attribute from the app's manifest file
    pub fn min_sdk_version(&mut self, min_sdk_version: u32) -> &mut Self {
        self.min_sdk_version = Some(min_sdk_version);
        self
    }

    /// The highest Android framework API level that apksigner uses to confirm
    /// that the APK's signature will be verified. By default, the tool uses
    /// the highest possible API level
    pub fn max_sdk_version(&mut self, min_sdk_version: u32) -> &mut Self {
        self.min_sdk_version = Some(min_sdk_version);
        self
    }

    /// Determines whether apksigner signs the given APK package using the
    /// traditional, JAR-based signing scheme. By default, the tool uses the
    /// values of `--min-sdk-version` and `--max-sdk-version` to decide when to
    /// apply this signature scheme
    pub fn v1_signing_enabled(&mut self, v1_signing_enabled: bool) -> &mut Self {
        self.v1_signing_enabled = v1_signing_enabled;
        self
    }

    /// Determines whether apksigner signs the given APK package using the APK
    /// Signature Scheme v2. By default, the tool uses the values of
    /// `--min-sdk-version` and `--max-sdk-version` to decide when to apply
    /// this signature scheme
    pub fn v2_signing_enabled(&mut self, v2_signing_enabled: bool) -> &mut Self {
        self.v2_signing_enabled = v2_signing_enabled;
        self
    }

    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// Determines whether apksigner signs the given APK package using the APK
    /// Signature Scheme v3. By default, the tool uses the values of
    /// `--min-sdk-version` and `--max-sdk-version` to decide when to apply this
    /// signature scheme
    pub fn v3_signing_enabled(&mut self, v3_signing_enabled: bool) -> &mut Self {
        self.v3_signing_enabled = v3_signing_enabled;
        self
    }

    /// Determines whether apksigner signs the given APK package using the APK
    /// Signature Scheme v4. This scheme produces a signature in an separate file
    /// (apk-name.apk.idsig). If true and the APK is not signed, then a v2 or v3
    /// signature is generated based on the values of `--min-sdk-version` and
    /// `--max-sdk-version`. The command then produces the .idsig file based on
    /// the content of the signed APK. Use only to generate only the v4 signature
    /// without modifying the APK and any signatures it had before the
    /// invocation; only fails if the APK doesn't have a v2 or v3 signature
    /// already, or if the signature used a different key than the one provided
    /// for the current invocation. By default, the tool uses the values of
    /// `--min-sdk-version` and `--max-sdk-version` to decide when to apply this
    /// signature scheme
    pub fn v4_signing_enabled(&mut self, v4_signing_enabled: V4SigningEnabled) -> &mut Self {
        self.v4_signing_enabled = Some(v4_signing_enabled);
        self
    }

    /// Use the verbose output mod
    pub fn v(&mut self, v: bool) -> &mut Self {
        self.v = v;
        self
    }

    /// Use the verbose output mod
    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose = verbose;
        self
    }

    /// Used for specifying different [`general options`] for each signer
    pub fn next_signer(&mut self, next_signer: bool) -> &mut Self {
        self.next_signer = next_signer;
        self
    }

    /// The base name for the files that comprise the JAR-based signature for
    /// the current signer. By default, apksigner uses the key alias of the
    /// KeyStore or the basename of the key file for this signer
    pub fn v1_signer_name(&mut self, v1_signer_name: String) -> &mut Self {
        self.v1_signer_name = Some(v1_signer_name);
        self
    }

    /// The signer's private key and certificate chain reside in the given
    /// Java-based KeyStore file. If the filename is set to "NONE", the
    /// KeyStore containing the key and certificate doesn't need a file
    /// specified, which is the case for some PKCS #11 KeyStores
    pub fn ks(&mut self, ks: &Path) -> &mut Self {
        self.ks = Some(ks.to_owned());
        self
    }

    /// The name of the alias that represents the signer's private key and
    /// certificate data within the KeyStore. If the KeyStore associated
    /// with the signer contains multiple keys, you must specify this option.
    pub fn ks_key_alias(&mut self, ks_key_alias: String) -> &mut Self {
        self.ks_key_alias = Some(ks_key_alias);
        self
    }

    /// The password for the KeyStore that contains the signer's private key
    /// and certificate. You must provide a password to open a KeyStore.
    /// he apksigner tool supports the following formats:
    ///
    /// * `pass:`<password> – Password provided inline with the rest of the
    /// apksigner sign command
    /// * `env:`<name> – Password is stored in the given environment variable
    /// * `file:`<filename> – Password is stored as a single line in the given
    /// file
    /// * `stdin` – Password is provided as a single line in the standard input
    /// stream. This is the default behavior for --ks-pass
    ///
    /// ## Note
    /// If you include multiple passwords in the same file, specify them on
    /// separate lines. The apksigner tool associates passwords with an APK's
    /// signers based on the order in which you specify the signers. If you've
    /// provided two passwords for a signer, apksigner interprets the first
    /// password as the KeyStore password and the second one as the key password
    pub fn ks_pass(&mut self, ks_pass: String) -> &mut Self {
        self.ks_pass = Some(ks_pass);
        self
    }

    /// Includes the specified character encodings (such as, ibm437 or utf-8)
    /// when trying to handle passwords containing non-ASCII characters
    ///
    /// Keytool often encrypts keystores by converting the password using the
    /// console's default charset. By default, apksigner tries to decrypt using
    /// several forms of the password: the Unicode form, the form encoded using
    /// the JVM default charset, and, on Java 8 and older, the form encoded
    /// using the console's default charset. On Java 9, apksigner cannot detect
    /// the console's charset. So, you may need to specify --pass-encoding when
    /// a non-ASCII password is used. You may also need to specify this option
    /// with keystores that keytool created on a different OS or in a different
    /// locale
    pub fn pass_encoding(&mut self, pass_encoding: String) -> &mut Self {
        self.pass_encoding = Some(pass_encoding);
        self
    }

    /// The password for the signer's private key, which is needed if the
    /// private key is password-protected. The apksigner tool supports the
    /// following formats:
    ///
    /// * `pass:`<password> – Password provided inline with the rest of the
    /// apksigner sign command
    /// * `env:`<name> – Password is stored in the given environment variable
    /// * `file:`<filename> – Password is stored as a single line in the given
    /// file
    /// * `stdin` – Password is provided as a single line in the standard input
    /// stream. This is the default behavior for `--key-pass`
    ///
    /// ## Note
    /// If you include multiple passwords in the same file, specify them on
    /// separate lines. The apksigner tool associates passwords with an APK's
    /// signers based on the order in which you specify the signers. If you've
    /// provided two passwords for a signer, apksigner interprets the first
    /// password as the KeyStore password and the second one as the key password.
    pub fn key_pass(&mut self, key_pass: String) -> &mut Self {
        self.key_pass = Some(key_pass);
        self
    }

    /// The type or algorithm associated with the KeyStore that contains the
    /// signer's private key and certificate. By default, apksigner uses the
    /// type defined as the keystore.type constant in the Security properties
    /// file
    pub fn ks_type(&mut self, ks_type: String) -> &mut Self {
        self.ks_type = Some(ks_type);
        self
    }

    /// The name of the JCA Provider to use when requesting the signer's
    /// KeyStore implementation. By default, apksigner uses the
    /// highest-priority provider
    pub fn ks_provider_name(&mut self, ks_provider_name: String) -> &mut Self {
        self.ks_provider_name = Some(ks_provider_name);
        self
    }

    /// The fully-qualified class name of the JCA Provider to use when
    /// requesting the signer's KeyStore implementation. This option serves
    /// as an alternative for `--ks-provider-name`. By default, apksigner uses
    /// the provider specified with the `--ks-provider-name option`
    pub fn ks_provider_class(&mut self, ks_provider_class: String) -> &mut Self {
        self.ks_provider_class = Some(ks_provider_class);
        self
    }

    /// A string value to pass in as the argument for the constructor of the
    /// JCA Provider class; the class itself is defined with the
    /// `--ks-provider-class` option. By default, apksigner uses the class's
    /// 0-argument constructor
    pub fn ks_provider_arg(&mut self, ks_provider_arg: String) -> &mut Self {
        self.ks_provider_arg = Some(ks_provider_arg);
        self
    }

    /// The name of the file that contains the signer's private key. This file
    /// must use the PKCS #8 DER format. If the key is password-protected,
    /// `apksigner` prompts for the password using standard input unless you
    /// specify a different kind of input format using the `--key-pass` option
    pub fn key(&mut self, key: &Path) -> &mut Self {
        self.key = Some(key.to_owned());
        self
    }

    /// The name of the file that contains the signer's certificate chain. This
    /// file must use the X.509 PEM or DER format
    pub fn cert(&mut self, cert: String) -> &mut Self {
        self.cert = Some(cert);
        self
    }

    /// Show information about the APK's signing certificates
    pub fn print_certs(&mut self, print_certs: bool) -> &mut Self {
        self.print_certs = print_certs;
        self
    }

    /// Treat warnings as errors
    pub fn werr(&mut self, werr: bool) -> &mut Self {
        self.werr = werr;
        self
    }

    /// Sign an APK
    ///
    /// The syntax for signing an APK using the apksigner tool is as follows:
    ///
    /// ```xml
    /// apksigner sign --ks keystore.jks |
    /// --key key.pk8 --cert cert.x509.pem
    /// [signer_options] app-name.apk
    /// ```
    ///
    /// When you sign an APK using the apksigner tool, you must provide the
    /// signer's private key and certificate. You can include this information
    /// in two different ways:
    /// * Specify a KeyStore file using the --ks option
    /// * Specify the private key file and certificate file separately using the
    /// `--key` and `--cert` options, respectively. The private key file must use
    /// the PKCS #8 format, and the certificate file must use the X.509 format.
    pub fn sign(&mut self, sign: bool) -> &mut Self {
        self.sign = sign;
        self
    }

    pub fn verify(&mut self, verify: &Path) -> &mut Self {
        self.verify = Some(verify.to_owned());
        self
    }

    /// Apk path
    pub fn apk_path(&mut self, apk_path: &Path) -> &mut Self {
        self.apk_path = Some(apk_path.to_owned());
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut apksigner = apksigner_tool()?;
        if self.sign {
            apksigner.arg("sign");
        }
        if let Some(out) = &self.out {
            apksigner.arg("--out").arg(out);
        }
        if let Some(min_sdk_version) = &self.min_sdk_version {
            apksigner
                .arg("--min-sdk-version")
                .arg(min_sdk_version.to_string());
        }
        if let Some(max_sdk_version) = &self.max_sdk_version {
            apksigner
                .arg("--max-sdk-version")
                .arg(max_sdk_version.to_string());
        }
        if self.v1_signing_enabled {
            apksigner.arg("--v1-signing-enabled");
        }
        if self.v2_signing_enabled {
            apksigner.arg("--v2-signing-enabled");
        }
        if self.v3_signing_enabled {
            apksigner.arg("--v3-signing-enabled");
        }
        if let Some(v4_signing_enabled) = &self.v4_signing_enabled {
            apksigner
                .arg("--v4-signing-enabled")
                .arg(v4_signing_enabled.to_string());
        }
        if self.v {
            apksigner.arg("-v");
        }
        if self.verbose {
            apksigner.arg("--verbose");
        }
        if self.next_signer {
            apksigner.arg("--next-signer");
        }
        if let Some(v1_signer_name) = &self.v1_signer_name {
            apksigner.arg("--v1-signer-name").arg(v1_signer_name);
        }
        if let Some(ks) = &self.ks {
            apksigner.arg("--ks").arg(ks);
        }
        if let Some(apk_path) = &self.apk_path {
            apksigner.arg(apk_path);
        }
        if let Some(ks_key_alias) = &self.ks_key_alias {
            apksigner.arg("--ks-key-alias").arg(ks_key_alias);
        }
        if let Some(ks_pass) = &self.ks_pass {
            apksigner.arg("--ks-pass").arg(ks_pass);
        }
        if let Some(pass_encoding) = &self.pass_encoding {
            apksigner.arg("--pass-encoding").arg(pass_encoding);
        }
        if let Some(key_pass) = &self.key_pass {
            apksigner.arg("--key-pass").arg(key_pass);
        }
        if let Some(ks_type) = &self.ks_type {
            apksigner.arg("--ks-type").arg(ks_type);
        }
        if let Some(ks_provider_name) = &self.ks_provider_name {
            apksigner.arg("--ks-provider-name").arg(ks_provider_name);
        }
        if let Some(ks_provider_class) = &self.ks_provider_class {
            apksigner.arg("--ks-provider-class").arg(ks_provider_class);
        }
        if let Some(ks_provider_arg) = &self.ks_provider_arg {
            apksigner.arg("--ks-provider-arg").arg(ks_provider_arg);
        }
        if let Some(verify) = &self.verify {
            apksigner.arg("verify").arg(verify);
        }
        if let Some(key) = &self.key {
            apksigner.arg("--key").arg(key);
        }
        if let Some(cert) = &self.cert {
            apksigner.arg("--cert").arg(cert);
        }
        if self.print_certs {
            apksigner.arg("--print-certs");
        }
        if self.werr {
            apksigner.arg("-Werr");
        }
        if self.help {
            apksigner.arg("--help");
        }
        println!("output: {:?}", apksigner);
        apksigner.output_err(true)?;
        Ok(())
    }
}

#[derive(Clone)]
pub enum V4SigningEnabled {
    False,
    True,
    Only,
}

impl std::fmt::Display for V4SigningEnabled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::False => write!(f, "false"),
            Self::True => write!(f, "true"),
            Self::Only => write!(f, "only"),
        }
    }
}

pub fn apksigner_tool() -> Result<Command> {
    if let Ok(apksigner) = which::which(bat!("apksigner")) {
        return Ok(Command::new(apksigner));
    }
    let sdk_path = sdk_path_from_env()?;
    let build_tools = sdk_path.join("build-tools");
    let target_sdk_version = std::fs::read_dir(&build_tools)
        .map_err(|_| Error::PathNotFound(build_tools.clone()))?
        .filter_map(|path| path.ok())
        .filter(|path| path.path().is_dir())
        .filter_map(|path| path.file_name().into_string().ok())
        .filter(|name| name.chars().next().unwrap().is_digit(10))
        .max()
        .ok_or(AndroidError::BuildToolsNotFound)?;
    let apksigner_bat = build_tools.join(target_sdk_version).join(bat!("apksigner"));
    Ok(Command::new(apksigner_bat))
}
