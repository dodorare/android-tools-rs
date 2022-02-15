#[derive(Clone, Default)]
 pub struct Dpm {
    name: Option<String>,
    user_id: Option<String>,
    set_active_admin: bool,
    set_profile_owner: bool,
    set_device_owner: bool,
    remove_active_admin: bool,
    clear_freeze_period_record: bool,
    force_network_logs: bool,
    force_security_logs: bool,
}

impl Dpm {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Sets component as active admin.
    ///
    /// Options are:
    /// * `--user` user_id: Specify the target user. You can also pass `--user
    /// current` to select the current user.
    pub fn set_active_admin(&mut self, set_active_admin: bool) -> &mut Self {
        self.set_active_admin = set_active_admin;
        self
    }

    /// Sets component as active admin and its package as profile owner for
    /// an existing user.
    ///
    /// Options are:
    /// * `--user user_id`: Specify the target user. You can also pass --user
    /// current to select the current user.
    /// * `--name name`: Specify the human-readable organization name.
    pub fn set_profile_owner(&mut self, set_profile_owner: bool) -> &mut Self {
        self.set_profile_owner = set_profile_owner;
        self
    }

    /// Sets component as active admin and its package as device owner.
    ///
    /// Options are:
    /// * `--user user_id`: Specify the target user. You can also pass --user
    /// current to select the current user.
    /// * `--name name`: Specify the human-readable organization name.
    pub fn set_device_owner(&mut self, set_device_owner: bool) -> &mut Self {
        self.set_device_owner = set_device_owner;
        self
    }

    /// Disables an active admin. The app must declare `android:testOnly` in the
    /// manifest. This command also removes device and profile owners.
    ///
    /// Options are:
    /// * `--user user_id`: Specify the target user. You can also pass --user
    /// current to select the current user.
    pub fn remove_active_admin(&mut self, remove_active_admin: bool) -> &mut Self {
        self.remove_active_admin = remove_active_admin;
        self
    }

    /// Clears the device's record of previously-set freeze periods for system OTA
    /// updates. This is useful to avoid the device's scheduling restrictions when
    /// developing apps that manage freeze-periods. See [`Manage system updates`].
    ///
    /// Supported on devices running Android 9.0 (API level 28) and higher.
    pub fn clear_freeze_period_record(&mut self, clear_freeze_period_record: bool) -> &mut Self {
        self.clear_freeze_period_record = clear_freeze_period_record;
        self
    }

    /// Forces the system to make any existing network logs ready for retrieval by
    /// a DPC. If there are connection or DNS logs available, the DPC receives the
    /// onNetworkLogsAvailable() callback. See Network activity logging.
    ///
    /// This command is rate-limited. Supported on devices running Android 9.0
    /// (API level 28) and higher.
    pub fn force_network_logs(&mut self, force_network_logs: bool) -> &mut Self {
        self.force_network_logs = force_network_logs;
        self
    }

    /// Forces the system to make any existing security logs available to the DPC.
    /// If there are logs available, the DPC receives the onSecurityLogsAvailable()
    /// callback. See Log enterprise device activity.
    ///
    /// This command is rate-limited. Supported on devices running Android 9.0
    /// (API level 28) and higher.
    pub fn force_security_logs(&mut self, force_security_logs: bool) -> &mut Self {
        self.force_security_logs = force_security_logs;
        self
    }
}