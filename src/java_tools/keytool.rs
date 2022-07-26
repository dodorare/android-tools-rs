use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

/// ## Description
/// Keytool is a key and certificate management utility. It allows users to
/// administer their own public/private key pairs and associated certificates
/// for use in self-authentication (where the user authenticates himself/herself
///  to other users/services) or data integrity and authentication services,
/// using digital signatures. It also allows users to cache the public keys
/// (in the form of certificates) of their communicating peers.
///
/// A certificate is a digitally signed statement from one entity (person, company, etc.),
/// saying that the public key (and some other information) of some other entity has a
/// particular value. (See Certificates.) When data is digitally signed, the signature
/// can be verified to check the data integrity and authenticity. Integrity means that
/// the data has not been modified or tampered with, and authenticity means the data
/// indeed comes from whoever claims to have created and signed it.
///
/// keytool also enables users to administer secret keys used in symmetric
/// encryption/decryption (e.g. DES).
///
/// keytool stores the keys and certificates in a [`keystore`].
///
/// Contents options that can help to manage keystore of cryptographic keys
///
/// [keystore]::(https://docs.oracle.com/javase/7/docs/technotes/tools/windows/keytool.html#KeyStore)
#[derive(Clone, Default)]
pub struct Keytool {
    v: bool,
    keystore: Option<PathBuf>,
    alias: Option<String>,
    keypass: Option<String>,
    storepass: Option<String>,
    dname: Option<Vec<String>>,
    storetype: Option<StoreType>,
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
    keyalg: Option<KeyAlgorithm>,
    file: Option<PathBuf>,
    rfc: bool,
    jarfile: Option<PathBuf>,
    new_arg: Option<String>,
    sslserver: Option<String>,
    providerpath: Option<Vec<PathBuf>>,
    destalias: Option<String>,
}

impl Keytool {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// The keystore location.
    ///
    /// If the JKS [`storetype`] is used and a keystore file does not yet exist, then certain
    /// keytool commands may result in a new keystore file being created. For example, if
    /// keytool -genkeypair is invoked and the -keystore option is not specified, the
    /// default keystore file named .keystore in the user's home directory will be created
    /// if it does not already exist. Similarly, if the -keystore ks_file option is
    /// specified but ks_file does not exist, then it will be created.
    ///
    /// Note that the input stream from the -keystore option is passed to the `KeyStore.load`
    /// method. If `NONE` is specified as the URL, then a null stream is passed to the
    /// `KeyStore.load` method. NONE should be specified if the `KeyStore` is not file-based
    /// (for example, if it resides on a hardware token device).
    ///
    /// [storetype](https://docs.oracle.com/javase/7/docs/technotes/tools/windows/keytool.html#KeyStoreImplementation)
    pub fn keystore(&mut self, keystore: &Path) -> &mut Self {
        self.keystore = Some(keystore.to_owned());
        self
    }

    /// Alias name of the entry to process
    pub fn alias(&mut self, alias: &str) -> &mut Self {
        self.alias = Some(alias.to_string());
        self
    }

    /// Key password
    pub fn keypass(&mut self, keypass: &str) -> &mut Self {
        self.keypass = Some(keypass.to_string());
        self
    }

    /// The password that is used to protect the integrity of the keystore.
    ///
    /// If the modifier env or file is not specified, then the password has the value argument,
    /// which must be at least 6 characters long. Otherwise, the password is retrieved as
    /// follows:
    ///
    /// * `env:` Retrieve the password from the environment variable named argument.
    /// * `file:` Retrieve the password from the file named argument.
    ///
    /// ## Note
    /// All other options that require passwords, such as -keypass, -srckeypass, -destkeypass,
    /// -srcstorepass, and -deststorepass, accept the env and file modifiers. Remember to
    /// separate the password option and the modifier with a colon (:).
    ///
    /// The password must be provided to all commands that access the keystore contents. For
    /// such commands, when the -storepass option is not provided at the command line, the user
    /// is prompted for it.
    ///
    /// When retrieving information from the keystore, the password is optional. If no password
    /// is specified, then the integrity of the retrieved information cannot be verified and a
    /// warning is displayed.
    pub fn storepass(&mut self, storepass: &str) -> &mut Self {
        self.storepass = Some(storepass.to_string());
        self
    }

    /// Key algorithm name
    pub fn keyalg(&mut self, keyalg: KeyAlgorithm) -> &mut Self {
        self.keyalg = Some(keyalg);
        self
    }

    /// Distinguished name
    pub fn dname(&mut self, dname: &[String]) -> &mut Self {
        self.dname = Some(dname.to_vec());
        self
    }

    /// This qualifier specifies the type of keystore to be instantiated.
    pub fn storetype(&mut self, storetype: StoreType) -> &mut Self {
        self.storetype = Some(storetype);
        self
    }

    /// Used to identify a cryptographic service provider's name when listed in the security
    /// properties file.
    pub fn providername(&mut self, providername: String) -> &mut Self {
        self.providername = Some(providername);
        self
    }

    /// Used to specify the name of a cryptographic service provider's master class file when
    /// the service provider is not listed in the security properties file.
    pub fn providerclass(&mut self, providerclass: String) -> &mut Self {
        self.providerclass = Some(providerclass);
        self
    }

    /// Used with the -providerclass option to represent an optional string input argument
    /// for the constructor of provider_class_name.
    pub fn providerarg(&mut self, providerarg: &Path) -> &mut Self {
        self.providerarg = Some(providerarg.to_owned());
        self
    }

    /// Either true or false. This value should be specified as true when a password must be
    /// specified by way of a protected authentication path such as a dedicated PIN
    /// reader.Because there are two keystores involved in the -importkeystore command, the
    /// following two options -srcprotected and -destprotected are provided for the source
    /// keystore and the destination keystore respectively.
    pub fn protected(&mut self, protected: bool) -> &mut Self {
        self.protected = protected;
        self
    }

    /// Denotes an X.509 certificate extension. The option can be used in -genkeypair and
    /// -gencert to embed extensions into the certificate generated, or in -certreq to show
    /// what extensions are requested in the certificate request. The option can appear
    /// multiple times. The name argument can be a supported extension name
    /// (see Named Extensions) or an arbitrary OID number. The value argument, when
    /// provided, denotes the argument for the extension. When value is omitted, that means
    /// that the default value of the extension or the extension requires no argument.
    /// The :critical modifier, when provided, means the extension's isCritical attribute
    /// is true; otherwise, it is false. You can use :c in place of :critical.
    pub fn ext(&mut self, ext: String) -> &mut Self {
        self.ext = Some(ext);
        self
    }

    /// Key bit size
    pub fn keysize(&mut self, keysize: u32) -> &mut Self {
        self.keysize = Some(keysize);
        self
    }

    /// Validity number of days
    pub fn validity(&mut self, validity: u32) -> &mut Self {
        self.validity = Some(validity);
        self
    }

    /// * {-rfc}: Output in RFC (Request For Comment) style
    /// * {-infile infile}: Input file name
    /// * {-outfile outfile}: Output file name
    /// * {-alias alias}: Alias name of the entry to process
    /// * {-sigalg sigalg}: Signature algorithm name
    /// * {-dname dname}: Distinguished name
    /// * {-startdate startdate}: Certificate validity start date and time
    /// * {-ext ext}*: X.509 extension
    /// * {-validity days}: Validity number of days
    /// * [-keypass arg]: Key password
    /// * {-keystore keystore}: Keystore name
    /// * [-storepass arg]: Keystore password
    /// * {-storetype type}: Keystore type
    /// * {-providername name}: Provider name
    /// * {-providerclass class [-providerarg arg]}: Add security provider by fully qualified class
    /// name with an optional configure argument. For example, if MyProvider is a legacy provider
    /// loaded via reflection,
    ///
    /// ```sh
    /// keytool -providerclass com.example.MyProvider ...
    /// ```
    ///
    /// * {-providerpath list}: Provider classpath
    /// * {-v}: Verbose output
    /// * {-protected}: Password provided through a protected mechanism
    ///
    /// Generates a certificate as a response to a certificate request file (which can be created by
    /// the keytool `-certreq` command). The command reads the request from infile (if omitted, from
    /// the standard input), signs it using alias's private key, and outputs the X.509 certificate
    /// into outfile (if omitted, to the standard output). When-rfc is specified, the output format
    /// is Base64-encoded PEM; otherwise, a binary DER is created.
    ///
    /// The sigalg value specifies the algorithm that should be used to sign the certificate. The
    /// startdate argument is the start time and date that the certificate is valid. The valDays
    /// argument tells the number of days for which the certificate should be considered valid.
    ///
    /// When dname is provided, it is used as the subject of the generated certificate. Otherwise,
    /// the one from the certificate request is used.
    ///
    /// The ext value shows what X.509 extensions will be embedded in the certificate. Read Common
    /// Options for the grammar of `-ext`.
    ///
    /// The `-gencert` option enables you to create certificate chains. The following example creates
    /// a certificate, e1, that contains three certificates in its certificate chain.
    pub fn gencert(&mut self, gencert: bool) -> &mut Self {
        self.gencert = gencert;
        self
    }

    /// * {-alias alias}: Alias name of the entry to process
    /// * {-keyalg alg}: Key algorithm name
    /// * {-keysize size}: Key bit size
    /// * {-groupname name}: Group name. For example, an Elliptic Curve name
    /// * {-sigalg alg}: Signature algorithm name
    /// * -destalias alias: Destination alias
    /// * [-dname name]: Distinguished name
    /// * {-startdate date}: Certificate validity start date and time
    /// * [-ext value]*: X.509 extension
    /// * {-validity days}: Validity number of days
    /// * [-keypass arg]: Key password
    /// * {-keystore keystore}: Keystore name
    /// * [-storepass arg]: Keystore password
    /// * {-storetype type}: Keystore type
    /// * {-providername name}: Provider name
    /// * {-providerclass class [-providerarg arg]}: Add security provider by fully
    /// qualified class name with an optional configure argument.
    /// * {-providerpath list}: Provider classpath
    /// * {-v}: Verbose output
    /// * {-protected}: Password provided through a protected mechanism
    ///
    /// Generates a key pair (a public key and associated private key). Wraps the public key
    /// into an X.509 v3 self-signed certificate, which is stored as a single-element
    /// certificate chain. This certificate chain and the private key are stored in a new
    /// keystore entry identified by alias.
    ///
    /// The keyalg value specifies the algorithm to be used to generate the key pair, and
    /// the keysize value specifies the size of each key to be generated. The sigalg value
    /// specifies the algorithm that should be used to sign the self-signed certificate.
    /// This algorithm must be compatible with the keyalg value.
    ///
    /// The groupname value specifies a named group when generating a key pair. The groupname
    /// option is preferred over the keysize option because there may be more than one curve
    /// of the same size. For example:
    ///
    /// ```sh
    ///  keytool -genkeypair -keyalg EC -groupname secp384r1
    /// ```
    ///
    /// The dname value specifies the X.500 Distinguished Name to be associated with the value
    /// of alias, and is used as the issuer and subject fields in the self-signed certificate.
    /// If no distinguished name is provided at the command line, then the user is prompted
    /// for one.
    ///
    /// The value of keypass is a password used to protect the private key of the generated key
    /// pair. If no password is provided, then the user is prompted for it. If you press the
    /// Return key at the prompt, then the key password is set to the same password as the
    /// keystore password. The keypass value must be at least 6 characters.
    ///
    /// The value of startdate specifies the issue time of the certificate, also known as the
    /// "Not Before" value of the X.509 certificate's Validity field.
    ///
    /// The option value can be set in one of these two forms:
    ///
    /// ```sh
    /// ([+-]nnn[ymdHMS])+
    /// [yyyy/mm/dd] [HH:MM:SS]
    /// ```
    ///
    /// When the option is not provided, the start date is the current time. The option can be
    /// provided at most once.
    ///
    /// The value of valDays specifies the number of days (starting at the date specified by
    /// `-startdate`, or the current date when `-startdate` is not specified) for which the
    /// certificate should be considered valid.
    ///
    /// This command was named -genkey in earlier releases. The old name is still supported in
    /// this release. The new name, -genkeypair, is preferred going forward.
    pub fn genkeypair(&mut self, genkeypair: bool) -> &mut Self {
        self.genkeypair = genkeypair;
        self
    }

    /// * {-alias alias}: Alias name of the entry to process
    /// * [-keypass arg] : Key password
    /// * {-keyalg alg}: Key algorithm name
    /// * {-keysize size}: Key bit size
    /// * {-keystore keystore}: Keystore name
    /// * [-storepass arg]: Keystore password
    /// * {-storetype type}: Keystore type
    /// * {-providername name}: Provider name
    /// * {-providerclass class [providerarg arg]}: Add security provider by fully qualified
    /// class name with an optional configure argument.
    /// * {-providerpath list}: Provider classpath
    /// * {-v}: Verbose output
    /// * {-protected}: Password provided through a protected mechanism
    ///
    /// Generates a secret key and stores it in a new KeyStore.SecretKeyEntry identified by
    /// alias.
    ///
    /// The value of keyalg specifies the algorithm to be used to generate the secret key,
    /// and the value of keysize specifies the size of the key to be generated. The keypass
    /// value is a password that protects the secret key. If no password is provided, then
    /// the user is prompted for it. If you press the Return key at the prompt, then the key
    /// password is set to the same password that is used for the keystore. The keypass value
    /// must be at least 6 characters.
    pub fn genseckey(&mut self, genseckey: bool) -> &mut Self {
        self.genseckey = genseckey;
        self
    }

    /// * {-noprompt}: Do not prompt
    /// * {-trustcacerts}: Trust certificates from cacerts
    /// * {-protected}: Password is provided through protected mechanism
    /// * {-alias alias}: Alias name of the entry to process
    /// * {-file file}: Input file name
    /// * [-keypass arg]: Key password
    /// * {-keystore keystore}: Keystore name
    /// * [-storepass arg]: Keystore password
    /// * {-storetype type}: Keystore type
    /// * {-providername name}: Provider name
    /// * {-providerclass class [-providerarg arg]}: Add security provider by fully qualified
    /// class name with an optional configure argument.
    /// * {-providerpath list}: Provider classpath
    /// * {-v}: Verbose output
    ///
    /// Reads the certificate or certificate chain (where the latter is supplied in a PKCS#7
    /// formatted reply or a sequence of X.509 certificates) from the file cert_file, and
    /// stores it in the keystore entry identified by alias. If no file is specified, then
    /// the certificate or certificate chain is read from stdin.
    ///
    /// You import a certificate for two reasons: To add it to the list of trusted certificates,
    /// and to import a certificate reply received from a certificate authority (CA) as the
    /// result of submitting a Certificate Signing Request to that CA
    /// (see the -certreq option in Commands).
    ///
    /// Which type of import is intended is indicated by the value of the -alias option. If the
    /// alias does not point to a key entry, then the keytool command assumes you are adding a
    /// trusted certificate entry. In this case, the alias should not already exist in the
    /// keystore. If the alias does already exist, then the keytool command outputs an error
    /// because there is already a trusted certificate for that alias, and does not import the
    /// certificate. If the alias points to a key entry, then the keytool command assumes you
    /// are importing a certificate reply.
    pub fn importcert(&mut self, mportcert: bool) -> &mut Self {
        self.mportcert = mportcert;
        self
    }

    /// * {-alias alias}: Alias name of the entry to process
    /// * [-keypass arg]: Key password
    /// * {-keyalg alg}: Key algorithm name
    /// * {-keysize size}: Key bit size
    /// * {-keystore keystore}: Keystore name
    /// * [-storepass arg]: Keystore password
    /// * {-storetype type}: Keystore type
    /// * {-providername name}: Provider name
    /// * {-providerclass class [-providerarg arg]}: Add security provider by fully qualified
    /// class name with an optional configure argument.
    /// * {-providerpath list}: Provider classpath
    /// * {-v}: Verbose output
    /// * {-protected}: Password provided through a protected mechanism
    ///
    /// Imports a passphrase and stores it in a new KeyStore.SecretKeyEntry identified by alias.
    /// The passphrase may be supplied via the standard input stream; otherwise the user is
    /// prompted for it. keypass is a password used to protect the imported passphrase. If no
    /// password is provided, the user is prompted for it. If you press the Return key at the
    /// prompt, the key password is set to the same password as that used for the keystore.
    /// keypass must be at least 6 characters long.
    pub fn importpass(&mut self, importpass: bool) -> &mut Self {
        self.importpass = importpass;
        self
    }

    /// * {-srckeystore keystore}: Source keystore name
    /// * {-destkeystore keystore}: Destination keystore name
    /// * {-srcstoretype type}: Source keystore type
    /// * {-deststoretype type}: Destination keystore type
    /// * [-srcstorepass arg]: Source keystore password
    /// * [-deststorepass arg]: Destination keystore password
    /// * {-srcprotected Source keystore password protected
    /// * {-srcprovidername name}: Source keystore provider name
    /// * {-destprotected}: Destination keystore password protected
    /// * {-destprovidername name}: Destination keystore provider name
    /// * {-srcalias alias}: Source alias
    /// * {-destalias alias}: Destination alias
    /// * [-srckeypass arg]: Source key password
    /// * [-destkeypass arg]: Destination key password
    /// * {-noprompt}: Do not prompt
    /// * {-providerclass class [-providerarg arg]}: Add security provider by fully qualified
    /// class name with an optional configure argument
    /// * {-providerpath list}: Provider classpath
    /// * {-v}: Verbose output
    ///
    /// Imports a single entry or all entries from a source keystore to a destination keystore.
    ///
    /// When the `-srcalias` option is provided, the command imports the single entry identified
    /// by the alias to the destination keystore. If a destination alias is not provided with
    /// destalias, then srcalias is used as the destination alias. If the source entry is
    /// protected by a password, then srckeypass is used to recover the entry. If srckeypass
    /// is not provided, then the keytool command attempts to use srcstorepass to recover the
    /// entry. If srcstorepass is either not provided or is incorrect, then the user is
    /// prompted for a password. The destination entry is protected with destkeypass. If
    /// destkeypass is not provided, then the destination entry is protected with the source
    /// entry password. For example, most third-party tools require storepass and keypass in
    /// a PKCS #12 keystore to be the same. In order to create a PKCS #12 keystore for these
    /// tools, always specify a -destkeypass to be the same as -deststorepass.
    ///
    /// If the `-srcalias` option is not provided, then all entries in the source keystore are
    /// imported into the destination keystore. Each destination entry is stored under the
    /// alias from the source entry. If the source entry is protected by a password, then
    /// srcstorepass is used to recover the entry. If srcstorepass is either not provided or
    /// is incorrect, then the user is prompted for a password. If a source keystore entry
    /// type is not supported in the destination keystore, or if an error occurs while storing
    /// an entry into the destination keystore, then the user is prompted whether to skip the
    /// entry and continue or to quit. The destination entry is protected with the source
    /// entry password.
    ///
    /// If the destination alias already exists in the destination keystore, then the user is
    /// prompted to either overwrite the entry or to create a new entry under a different
    /// alias name.
    ///
    /// If the -noprompt option is provided, then the user is not prompted for a new destination
    /// alias. Existing entries are overwritten with the destination alias name. Entries that
    /// cannot be imported are skipped and a warning is displayed.
    pub fn importkeystore(&mut self, importkeystore: bool) -> &mut Self {
        self.importkeystore = importkeystore;
        self
    }

    /// * {-file file}: Input file name
    /// * {-v}: Verbose output
    ///
    /// Prints the content of a PKCS #10 format certificate request, which can be generated by
    /// the keytool -certreq command. The command reads the request from file. If there is no
    /// file, then the request is read from the standard input.Prints the content of a PKCS #10
    /// format certificate request, which can be generated by the keytool -certreq command. The
    /// command reads the request from file. If there is no file, then the request is read from
    /// the standard input.
    pub fn printcertreq(&mut self, printcertreq: bool) -> &mut Self {
        self.printcertreq = printcertreq;
        self
    }

    /// * {-alias alias}: Alias name of the entry to process
    /// * {-sigalg alg}: Signature algorithm name
    /// * {-file file}: Output file name
    /// * [-keypass arg]: Key password
    /// * {-keystore keystore}: Keystore name
    /// * {-dname name}: Distinguished name
    /// * [-storepass arg]: Keystore password
    /// * {-storetype type}: Keystore type
    /// * {-providername name}: Provider name
    /// * {-providerclass class [-providerarg arg]}: Add security provider by fully qualified
    /// class name with an optional configure argument.
    /// * {-providerpath list}: Provider classpath
    /// * {-v}: Verbose output
    /// * {-protected }: Password provided through a protected mechanism
    ///
    /// Generates a Certificate Signing Request (CSR) using the PKCS #10 format.
    ///
    /// A CSR is intended to be sent to a certificate authority (CA). The CA authenticates the
    /// certificate requestor (usually off-line) and will return a certificate or certificate
    /// chain, used to replace the existing certificate chain (which initially consists of a
    /// self-signed certificate) in the keystore.
    ///
    /// The private key associated with alias is used to create the PKCS #10 certificate request.
    /// To access the private key, the correct password must be provided. If keypass is not
    /// provided at the command line and is different from the password used to protect the
    /// integrity of the keystore, then the user is prompted for it. If dname is provided, then
    /// it is used as the subject in the CSR. Otherwise, the X.500 Distinguished Name associated
    /// with alias is used.
    ///
    /// The sigalg value specifies the algorithm that should be used to sign the CSR.
    ///
    /// The CSR is stored in the file certreq_file. If no file is specified, then the CSR is
    /// output to stdout.
    ///
    /// Use the importcert command to import the response from the CA.
    pub fn certreq(&mut self, certreq: bool) -> &mut Self {
        self.certreq = certreq;
        self
    }

    /// * {-rfc}: Output in RFC style
    /// * {-alias alias}: Alias name of the entry to process
    /// * {-file file}: Output file name
    /// * {-keystore keystore}: Keystore name
    /// * [-storepass arg]: Keystore password
    /// * {-storetype type}: Keystore type
    /// * {-providername name}: Provider name
    /// * {-providerclass class [-providerarg arg]}: Add security provider by fully qualified
    /// * class name with an optional configure argument.
    /// * {-providerpath list}: Provider classpath
    /// * {-v}: Verbose output
    /// * {-protected}: Password provided through a protected mechanism
    ///
    /// Reads from the keystore the certificate associated with alias and stores it in the
    /// cert_file file. When no file is specified, the certificate is output to stdout.
    ///
    /// The certificate is by default output in binary encoding. If the -rfc option is
    /// specified, then the output in the printable encoding format defined by the [`Internet
    /// RFC 1421 Certificate Encoding Standard`].
    ///
    /// If alias refers to a trusted certificate, then that certificate is output. Otherwise,
    /// alias refers to a key entry with an associated certificate chain. In that case, the
    /// first certificate in the chain is returned. This certificate authenticates the public
    /// key of the entity addressed by alias.
    ///
    /// This command was named -export in earlier releases. The old name is still supported
    /// in this release. The new name, -exportcert, is preferred going forward.
    ///
    /// [Internet RFC 1421 Certificate Encoding Standard]::(https://docs.oracle.com/javase/8/docs/technotes/tools/windows/keytool.html#CHDGGFEG)
    pub fn exportcert(&mut self, exportcert: bool) -> &mut Self {
        self.exportcert = exportcert;
        self
    }

    /// * {-rfc}: Output in RFC style
    /// * {-alias alias}: Alias name of the entry to process
    /// * {-keystore keystore}: Keystore name
    /// * [-storepass arg]: Keystore password
    /// * {-storetype type}: Keystore type
    /// * {-providername name}: Provider name
    /// * {-providerclass class [-providerarg arg] }: Add security provider by fully qualified
    /// class name with an optional configure argument.
    /// * {-providerpath list}: Provider classpath
    /// * {-v}: Verbose output
    /// * {-protected}: Password provided through a protected mechanism
    ///
    /// Prints to stdout the contents of the keystore entry identified by alias. If no alias
    /// is specified, then the contents of the entire keystore are printed.
    ///
    /// This command by default prints the SHA256 fingerprint of a certificate. If the `-v`
    /// option is specified, then the certificate is printed in human-readable format, with
    /// additional information such as the owner, issuer, serial number, and any extensions.
    /// If the -rfc option is specified, then the certificate contents are printed using the
    /// printable encoding format, as defined by the [`Internet RFC 1421 Certificate Encoding
    /// Standard`].
    ///
    /// You cannot specify both `-v` and `-rfc`.
    ///
    /// [Internet RFC 1421 Certificate Encoding Standard]::(https://docs.oracle.com/javase/8/docs/technotes/tools/windows/keytool.html#CHDGGFEG)
    pub fn list(&mut self, list: bool) -> &mut Self {
        self.list = list;
        self
    }

    /// * {-rfc}: Output in RFC style
    /// * {-file cert_file}: Input file name
    /// * {-sslserver server[:port]}: Secure Sockets Layer (SSL) server host and port
    /// * {-jarfile JAR_file}: Signed .jar file
    /// * {-v}: Verbose output
    ///
    /// Reads the certificate from the file cert_file, the SSL server located at host:port,
    /// or the signed JAR file JAR_file (with the -jarfile option) and prints its contents
    /// in a human-readable format. When no port is specified, the standard HTTPS port 443
    /// is assumed. Note that -sslserver and -file options cannot be provided at the same
    /// time. Otherwise, an error is reported. If neither option is specified, then the
    /// certificate is read from stdin.
    ///
    /// When -rfc is specified, the keytool command prints the certificate in PEM mode as
    /// defined by the Internet RFC 1421 Certificate Encoding standard. See [`Internet RFC
    /// 1421 Certificate Encoding Standard`].
    ///
    /// If the certificate is read from a file or stdin, then it might be either binary
    /// encoded or in printable encoding format, as defined by the RFC 1421 Certificate
    /// Encoding standard.
    ///
    /// If the SSL server is behind a firewall, then the -J-Dhttps.proxyHost=proxyhost and
    /// -J-Dhttps.proxyPort=proxyport options can be specified on the command line for proxy
    /// tunneling. See Java Secure Socket Extension (JSSE) Reference Guide at
    /// http://docs.oracle.com/javase/8/docs/technotes/guides/security/jsse/JSSERefGuide.html
    ///
    /// ## Note
    /// This option can be used independently of a keystore.
    ///
    /// [Internet RFC 1421 Certificate Encoding Standard]::(https://docs.oracle.com/javase/8/docs/technotes/tools/windows/keytool.html#CHDGGFEG)
    pub fn printcert(&mut self, printcert: bool) -> &mut Self {
        self.printcert = printcert;
        self
    }

    /// * -file crl: Input file name
    /// * {-v}: Verbose output
    ///
    /// Reads the Certificate Revocation List (CRL) from the file crl. A CRL is a list of
    /// digital certificates that were revoked by the CA that issued them. The CA
    /// generates the crl file.
    ///
    /// ## Note
    /// This option can be used independently of a keystore.
    pub fn printcrl(&mut self, printcrl: bool) -> &mut Self {
        self.printcrl = printcrl;
        self
    }

    /// * [-new arg]: New password
    /// * {-keystore keystore}: Keystore name
    /// * [-storepass arg]: Keystore password
    /// * {-storetype type}: Keystore type
    /// * {-providername name}: Provider name
    /// * {-providerclass class [-providerarg arg]}: Add security provider by fully
    /// qualified class name with an optional configure argument.
    /// * {-providerpath list}: Provider classpath
    /// * {-v}: Verbose output]
    ///
    /// Changes the password used to protect the integrity of the keystore contents. The
    /// new password is new_storepass, which must be at least 6 characters.
    pub fn storepasswd(&mut self, storepasswd: bool) -> &mut Self {
        self.storepasswd = storepasswd;
        self
    }

    /// * {-alias alias}: Alias name of the entry to process
    /// * {-keypass old_keypass}: Key password
    /// * {-new new_keypass}: New password
    /// * {-keystore keystore}: Keystore name
    /// * {-storepass arg}: Keystore password
    /// * {-storetype type}: Keystore type
    /// * {-providername name}: Provider name
    /// * {-providerclass class [-providerarg arg]}: Add security provider by fully
    /// qualified class name with an optional configure argument.
    /// * {-providerpath list}: Provider classpath
    /// * {-v}: Verbose output
    ///
    /// Changes the password under which the private/secret key identified by alias
    /// is protected, from old_keypass to new_keypass, which must be at least 6
    /// characters.
    ///
    /// If the `-keypass` option is not provided at the command line, and the key password
    /// is different from the keystore password, then the user is prompted for it.
    ///
    /// If the -new option is not provided at the command line, then the user is prompted
    /// for it
    pub fn keypasswd(&mut self, keypasswd: bool) -> &mut Self {
        self.keypasswd = keypasswd;
        self
    }

    /// * [-alias alias]: Alias name of the entry to process
    /// * {-keystore keystore}: Keystore name
    /// * [-storepass arg]: Keystore password
    /// * {-storetype type}: Keystore type
    /// * {-providername name}: Provider name
    /// * {-providerclass class [-providerarg arg]}: Add security provider by fully qualified
    /// class name with an optional configure argument.
    /// * {-providerpath list}: Provider classpath
    /// * {-v}: Verbose output
    /// * {-protected}: Password provided through a protected mechanism
    ///
    /// Deletes from the keystore the entry identified by alias. The user is prompted for
    /// the alias, when no alias is provided at the command line.
    pub fn delete(&mut self, delete: bool) -> &mut Self {
        self.delete = delete;
        self
    }

    /// * {-alias alias}: Alias name of the entry to process
    /// * [-destalias alias]: Destination alias
    /// * [-keypass arg]: Key password
    /// * {-keystore keystore}: Keystore name
    /// * [-storepass arg]: Keystore password
    /// * {-storetype type}: Keystore type
    /// * {-providername name}: Provider name
    /// * {-providerclass class [-providerarg arg]}: Add security provider by fully qualifie
    /// class name with an optional configure argument.
    /// * {-providerpath list}: Provider classpath
    /// * {-v}: Verbose output
    /// * {-protected}: Password provided through a protected mechanism
    ///
    /// Move an existing keystore entry from the specified alias to a new alias, destalias.
    /// If no destination alias is provided, then the command prompts for one. If the original
    /// entry is protected with an entry password, then the password can be supplied with the
    /// -keypass option. If no key password is provided, then the storepass (if provided) is
    /// attempted first. If the attempt fails, then the user is prompted for a password.
    pub fn changealias(&mut self, changealias: bool) -> &mut Self {
        self.changealias = changealias;
        self
    }

    /// Lists the basic commands and their options.
    ///
    /// For more information about a specific command, enter the following, where command_name
    /// is the name of the command: keytool `-command_name` `-help`.
    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// Output file name
    pub fn file(&mut self, file: &Path) -> &mut Self {
        self.file = Some(file.to_owned());
        self
    }

    /// Signed .jar file
    pub fn jarfile(&mut self, jarfile: &Path) -> &mut Self {
        self.jarfile = Some(jarfile.to_owned());
        self
    }

    /// New password
    pub fn new_arg(&mut self, new_arg: String) -> &mut Self {
        self.new_arg = Some(new_arg);
        self
    }

    /// Secure Sockets Layer (SSL) server host and port
    pub fn sslserver(&mut self, sslserver: String) -> &mut Self {
        self.sslserver = Some(sslserver);
        self
    }

    /// Output in RFC style
    pub fn rfc(&mut self, rfc: bool) -> &mut Self {
        self.rfc = rfc;
        self
    }

    /// Verbose output
    pub fn v(&mut self, v: bool) -> &mut Self {
        self.v = v;
        self
    }

    /// Provider classpath
    pub fn providerpath(&mut self, providerpath: &[PathBuf]) -> &mut Self {
        self.providerpath = Some(providerpath.to_vec());
        self
    }

    /// Destination alias
    pub fn destalias(&mut self, destalias: String) -> &mut Self {
        self.destalias = Some(destalias);
        self
    }

    /// Runs keytool commands
    pub fn run(&self) -> Result<Option<Key>> {
        let mut key = Some(Key::new_default()?);
        let mut keytool = keytool()?;
        if self.v {
            keytool.arg("-v");
        }
        if let Some(keystore) = &self.keystore {
            keytool.arg("-keystore").arg(keystore);
            if let Some(key) = &mut key {
                key.key_path = keystore.to_owned();
            }
        }
        if let Some(alias) = &self.alias {
            keytool.arg("-alias").arg(alias);
            if let Some(key) = &mut key {
                key.key_alias = alias.to_owned();
            }
        }
        if let Some(keypass) = &self.keypass {
            keytool.arg("-keypass").arg(keypass);
            if let Some(key) = &mut key {
                key.key_pass = keypass.to_owned();
            }
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
            key = None;
        }
        if let Some(file) = &self.file {
            keytool.arg("-file").arg(file);
        }
        if let Some(jarfile) = &self.jarfile {
            keytool.arg("-jarfile").arg(jarfile);
        }
        if let Some(new_arg) = &self.new_arg {
            keytool.arg("-new").arg(new_arg);
        }
        if let Some(sslserver) = &self.sslserver {
            keytool.arg("-sslserver").arg(sslserver);
        }
        if let Some(destalias) = &self.destalias {
            keytool.arg("-destalias").arg(destalias);
        }
        if self.rfc {
            keytool.arg("-rfc");
        }
        if let Some(providerpath) = &self.providerpath {
            keytool.arg("-providerpath").arg(
                providerpath
                    .iter()
                    .map(|v| v.to_string_lossy().to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        keytool.output_err(true)?;
        Ok(key)
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
pub enum StoreType {
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
pub enum KeyAlgorithm {
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

impl std::fmt::Display for StoreType {
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

impl std::fmt::Display for KeyAlgorithm {
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
pub struct Key {
    pub key_path: PathBuf,
    pub key_pass: String,
    pub key_alias: String,
}

impl Key {
    pub fn new_default() -> Result<Self> {
        let key_path = android_dir()?.join("aab.keystore");
        let key_pass = "android".to_string();
        let key_alias = "androidaabkey".to_string();
        Ok(Self {
            key_path,
            key_pass,
            key_alias,
        })
    }
}

/// Returns or crates it if needed the path to `.android` in the user's home directory.
pub fn android_dir() -> Result<PathBuf> {
    let android_dir = dirs::home_dir()
        .ok_or_else(|| Error::PathNotFound(PathBuf::from("$HOME")))?
        .join(".android");
    std::fs::create_dir_all(&android_dir)?;
    Ok(android_dir)
}
