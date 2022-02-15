use std::path::{PathBuf, Path};

#[derive(Clone, Default)]
 pub struct Pm {
    list_packages: Option<String>,
    list_permission_groups: Option<String>,
    list_permission: bool,
    list_instrumentation: bool,
    list_features: bool,
    list_libraries: bool,
    list_users: bool,
    path_package: Option<PathBuf>,
    install_path: Option<PathBuf>,
    uninstall: Option<PathBuf>,
    clear: Option<PathBuf>,
    enable: Option<PathBuf>,
    disable: Option<PathBuf>,
    disable_user: Option<PathBuf>,
    grant: Option<String>,
    revoke: Option<String>,
    set_install_location: Option<PathBuf>,
    get_install_location: Option<PathBuf>,
    set_permission_enforced: bool,
    trim_caches: bool,
    create_user: Option<String>,
    remove_user: Option<String>,
    get_max_users: bool,
}

impl Pm {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Prints all packages, optionally only those whose package name contains
    /// the text in filter.
    ///
    /// Options:
    ///
    /// * `-f`: See their associated file.
    /// * `-d`: Filter to only show disabled packages.
    /// * `-e`: Filter to only show enabled packages.
    /// * `-s`: Filter to only show system packages.
    /// * `-3`: Filter to only show third party packages.
    /// * `-i`: See the installer for the packages.
    /// * `-u`: Also include uninstalled packages.
    /// * `--user user_id`: The user space to query.
    pub fn list_packages(&mut self, list_packages: String) -> &mut Self {
        self.list_packages = Some(list_packages.to_owned());
        self
    }

    /// Prints all known permission groups.
    pub fn list_permission_groups(&mut self, list_permission_groups: String) -> &mut Self {
        self.list_permission_groups = Some(list_permission_groups.to_owned());
        self
    }

    /// Prints all known permissions, optionally only those in group.
    ///
    /// Options:
    ///
    /// * `-g`: Organize by group.
    /// * `-f`: Print all information.
    /// * `-s`: Short summary.
    /// * `-d`: Only list dangerous permissions.
    /// * `-u`: List only the permissions users will see.
    pub fn list_permission(&mut self, list_permission: bool) -> &mut Self {
        self.list_permission = list_permission;
        self
    }

    /// List all test packages.
    ///
    /// Options:
    ///
    /// * `-f`: List the APK file for the test package.
    /// * `target_package`: List test packages for only this app.
    pub fn list_instrumentation(&mut self, list_instrumentation: bool) -> &mut Self {
        self.list_instrumentation = list_instrumentation;
        self
    }

    /// Prints all features of the system.
    pub fn list_features(&mut self, list_features: bool) -> &mut Self {
        self.list_features = list_features;
        self
    }

    /// Prints all the libraries supported by the current device.
    pub fn list_libraries(&mut self, list_libraries: bool) -> &mut Self {
        self.list_libraries = list_libraries;
        self
    }

    /// Prints all users on the system.
    pub fn list_users(&mut self, list_users: bool) -> &mut Self {
        self.list_users = list_users;
        self
    }

    /// Print the path to the APK of the given package.
    pub fn path_package(&mut self, path_package: &Path) -> &mut Self {
        self.path_package = Some(path_package.to_owned());
        self
    }

    /// Installs a package (specified by path) to the system.
    ///
    /// Options:
    /// * `-r`: Reinstall an existing app, keeping its data.
    /// * `-t`: Allow test APKs to be installed. Gradle generates a test APK when
    /// you have only run or debugged your app or have used the Android Studio
    /// Build > Build APK command. If the APK is built using a developer preview
    /// SDK (if the targetSdkVersion is a letter instead of a number), you must
    /// include the -t option with the install command if you are installing a
    /// test APK.
    /// * `-i installer_package_name`: Specify the installer package name.
    /// * `--install-location location`: Sets the install location using one of
    /// the following values:
    ///     * `0`: Use the default install location
    ///     * `1`: Install on internal device storage
    ///     * `2`: Install on external media
    /// * `-f`: Install package on the internal system memory.
    /// * `-d`: Allow version code downgrade.
    /// * `-g`: Grant all permissions listed in the app manifest.
    /// * `--fastdeploy`: Quickly update an installed package by only updating the
    /// parts of the APK that changed.
    /// * `--incremental`: Installs enough of the APK to launch the app while
    /// streaming the remaining data in the background. To use this feature, you
    /// must sign the APK, create an [`APK Signature Scheme v4 file`], and place this
    /// file in the same directory as the APK. This feature is only supported on
    /// certain devices. This option forces adb to use the feature or fail if it
    /// is not supported (with verbose information on why it failed). Append the
    /// `--wait` option to wait until the APK is fully installed before granting
    /// access to the APK.
    pub fn install_path(&mut self, install_path: &Path) -> &mut Self {
        self.install_path = Some(install_path.to_owned());
        self
    }

    /// Removes a package from the system.
    ///
    /// Options:
    /// * `-k`: Keep the data and cache directories around after package removal.
    pub fn uninstall(&mut self, uninstall: &Path) -> &mut Self {
        self.uninstall = Some(uninstall.to_owned());
        self
    }

    /// Deletes all data associated with a package.
    pub fn clear(&mut self, clear: &Path) -> &mut Self {
        self.clear = Some(clear.to_owned());
        self
    }

    /// Enable the given package or component (written as "package/class").
    pub fn enable(&mut self, enable: &Path) -> &mut Self {
        self.enable = Some(enable.to_owned());
        self
    }

    /// Disable the given package or component (written as "package/class").
    pub fn disable(&mut self, disable: &Path) -> &mut Self {
        self.disable = Some(disable.to_owned());
        self
    }

    /// Options:
    ///
    /// `--user user_id`: The user to disable.
    pub fn disable_user(&mut self, disable_user: &Path) -> &mut Self {
        self.disable_user = Some(disable_user.to_owned());
        self
    }

    /// Grant a permission to an app. On devices running Android 6.0 (API level 23)
    /// and higher, the permission can be any permission declared in the app
    /// manifest. On devices running Android 5.1 (API level 22) and lower, must be
    /// an optional permission defined by the app.
    pub fn grant(&mut self, grant: String) -> &mut Self {
        self.grant = Some(grant.to_owned());
        self
    }

    /// Revoke a permission from an app. On devices running Android 6.0 (API level 23)
    /// and higher, the permission can be any permission declared in the app manifest.
    /// On devices running Android 5.1 (API level 22) and lower, must be an optional
    /// permission defined by the app.
    pub fn revoke(&mut self, revoke: String) -> &mut Self {
        self.revoke = Some(revoke.to_owned());
        self
    }

    pub fn get_install_location(&mut self, get_install_location: &Path) -> &mut Self {
        self.get_install_location = Some(get_install_location.to_owned());
        self
    }

    /// Changes the default install location. Location values:
    /// * `0`: Auto: Let system decide the best location.
    /// * `1`: Internal: install on internal device storage.
    /// * `2`: External: on external media.
    ///
    /// ## Note
    /// This is only intended for debugging; using this can cause apps to break and
    /// other undesireable behavior.
    pub fn get_install_location(&mut self, get_install_location: &Path) -> &mut Self {
        self.get_install_location = Some(get_install_location.to_owned());
        self
    }

    pub fn get_install_location(&mut self, get_install_location: bool) -> &mut Self {
        self.get_install_location = get_install_location;
        self
    }
}