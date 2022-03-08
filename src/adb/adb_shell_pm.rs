use crate::error::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

use super::InstallLocation;

/// # Call package manager (pm)
///
/// Within an adb shell, you can issue commands with the package manager
/// (pm) tool to perform actions and queries on app packages installed
/// on the device. While in a shell, the syntax is:
///
/// ```xml
/// pm command
/// ```
///
/// You can also issue a package manager command directly from adb without
/// entering a remote shell.
///
/// For example:
///
/// ```xml
/// adb shell pm uninstall com.example.MyApp
/// ```
#[derive(Clone, Default)]
pub struct AdbShellPm {
    list_permission_groups: Option<String>,
    list_instrumentation: bool,
    list_features: bool,
    list_libraries: bool,
    list_users: bool,
    path_package: Option<PathBuf>,
    uninstall: Option<PathBuf>,
    clear: Option<PathBuf>,
    enable: Option<PathBuf>,
    disable: Option<PathBuf>,
    disable_user: Option<PathBuf>,
    grant: Option<String>,
    revoke: Option<String>,
    set_install_location: Option<InstallLocation>,
    get_install_location: Option<InstallLocation>,
    set_permission_enforced: bool,
    trim_caches: Option<PathBuf>,
    create_user: Option<String>,
    remove_user: Option<String>,
    get_max_users: bool,
    f: bool,
    k: bool,
    fastdeploy: bool,
    user: Option<String>,
}

impl AdbShellPm {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Prints all known permission groups.
    pub fn list_permission_groups(&mut self, list_permission_groups: String) -> &mut Self {
        self.list_permission_groups = Some(list_permission_groups);
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
        self.grant = Some(grant);
        self
    }

    /// Revoke a permission from an app. On devices running Android 6.0 (API level 23)
    /// and higher, the permission can be any permission declared in the app manifest.
    /// On devices running Android 5.1 (API level 22) and lower, must be an optional
    /// permission defined by the app.
    pub fn revoke(&mut self, revoke: String) -> &mut Self {
        self.revoke = Some(revoke);
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
    pub fn set_install_location(&mut self, set_install_location: InstallLocation) -> &mut Self {
        self.set_install_location = Some(set_install_location);
        self
    }

    /// Returns the current install location. Return values:
    /// * `0`: Auto: Let system decide the best location.
    /// * `1`: Internal: install on internal device storage.
    /// * `2`: External: on external media.
    pub fn get_install_location(&mut self, get_install_location: InstallLocation) -> &mut Self {
        self.get_install_location = Some(get_install_location);
        self
    }

    /// Specifies whether the given permission should be enforced.
    pub fn set_permission_enforced(&mut self, set_permission_enforced: bool) -> &mut Self {
        self.set_permission_enforced = set_permission_enforced;
        self
    }

    /// Trim cache files to reach the given free space.
    pub fn trim_caches(&mut self, trim_caches: &Path) -> &mut Self {
        self.trim_caches = Some(trim_caches.to_owned());
        self
    }

    /// Create a new user with the given user_name, printing the new user identifier
    /// of the user.
    pub fn create_user(&mut self, create_user: String) -> &mut Self {
        self.create_user = Some(create_user);
        self
    }

    /// Remove the user with the given user_id, deleting all data associated with
    /// that user
    pub fn remove_user(&mut self, remove_user: String) -> &mut Self {
        self.remove_user = Some(remove_user);
        self
    }

    /// Prints the maximum number of users supported by the device.
    pub fn get_max_users(&mut self, get_max_users: bool) -> &mut Self {
        self.get_max_users = get_max_users;
        self
    }

    pub fn f(&mut self, f: bool) -> &mut Self {
        self.f = f;
        self
    }

    pub fn k(&mut self, k: bool) -> &mut Self {
        self.k = k;
        self
    }

    pub fn fastdeploy(&mut self, fastdeploy: bool) -> &mut Self {
        self.fastdeploy = fastdeploy;
        self
    }

    pub fn user(&mut self, user: String) -> &mut Self {
        self.user = Some(user);
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut pm = Command::new("adb");
        pm.arg("shell");
        pm.arg("pm");
        if let Some(list_permission_groups) = &self.list_permission_groups {
            pm.arg("list permission groups").arg(list_permission_groups);
        }
        if self.list_instrumentation {
            pm.arg("list instrumentation");
        }
        if self.list_features {
            pm.arg("list features");
        }
        if self.list_libraries {
            pm.arg("list libraries");
        }
        if self.list_users {
            pm.arg("list users");
        }
        if let Some(path_package) = &self.path_package {
            pm.arg("path").arg(path_package);
        }
        if let Some(uninstall) = &self.uninstall {
            pm.arg("uninstall").arg(uninstall);
        }
        if let Some(clear) = &self.clear {
            pm.arg("clear").arg(clear);
        }
        if let Some(enable) = &self.enable {
            pm.arg("enable").arg(enable);
        }
        if let Some(disable) = &self.disable {
            pm.arg("disable").arg(disable);
        }
        if let Some(disable_user) = &self.disable_user {
            pm.arg("disable-user").arg(disable_user);
        }
        if let Some(grant) = &self.grant {
            pm.arg("grant").arg(grant);
        }
        if let Some(revoke) = &self.revoke {
            pm.arg("revoke").arg(revoke);
        }
        if let Some(set_install_location) = &self.set_install_location {
            pm.arg("set-install-location")
                .arg(set_install_location.to_string());
        }
        if let Some(get_install_location) = &self.get_install_location {
            pm.arg("get-install-location")
                .arg(get_install_location.to_string());
        }
        if self.set_permission_enforced {
            pm.arg("set-permission-enforced");
        }
        if let Some(trim_caches) = &self.trim_caches {
            pm.arg("trim-caches").arg(trim_caches);
        }
        if let Some(create_user) = &self.create_user {
            pm.arg("create-user").arg(create_user);
        }
        if let Some(remove_user) = &self.remove_user {
            pm.arg("remove-user").arg(remove_user);
        }
        if self.get_max_users {
            pm.arg("get-max-users");
        }
        if self.f {
            pm.arg("-f");
        }
        if self.k {
            pm.arg("-k");
        }
        if self.fastdeploy {
            pm.arg("--fastdeploy");
        }
        if let Some(user) = &self.user {
            pm.arg("--user").arg(user);
        }
        pm.output_err(true)?;
        Ok(())
    }
}
