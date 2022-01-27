use std::path::{Path, PathBuf};

#[derive(Clone, Default)]
pub struct Emulator {
    list_avds: bool,
    sysdir: Option<PathBuf>,
    system: Option<PathBuf>,
    vendor: Option<PathBuf>,
    writable_system: bool,
    delay_adb: bool,
    datadir: Option<PathBuf>,
    kernel: Option<PathBuf>,
    ramdisk: Option<PathBuf>,
    image: Option<PathBuf>,
    initdata: Option<PathBuf>,
    data: Option<PathBuf>,
    encryption_key: Option<PathBuf>,
    logcat_output: Option<PathBuf>,
    partition_size: Option<u32>,
    cache: Option<PathBuf>,
    cache_size: Option<u32>,
    no_cache: bool,
    nocache: bool,
    sdcard: Option<PathBuf>,
    quit_after_boot: Option<u32>,
    qemu_top_dir: Option<PathBuf>,
    monitor_adb: bool,
    snapstorage: Option<PathBuf>,
    no_snapstorage: bool,
    snapshot: Option<String>,
    no_snapshot: bool,
    no_snapshot_save: bool,
    no_snapshot_load: bool,
    snapshot_list: bool,
    no_snapshot_update_time: bool,
    wipe_data: bool,
    avd: Option<String>,
    avd_arch: Option<String>, // check
    skindir: Option<PathBuf>,
    skin: Option<String>,
    no_skin: bool,
    noskin: bool,
    memory: Option<u32>,
    ui_only: Option<String>,
    id: Option<String>,
    cores: Option<u32>,
    accel: Option<String>,
    no_accel: bool, // TODO
    ranchu: bool, // TODO
    engine: Option<String>, // TODO
    netspeed: Option<u32>,
    netdelay: Option<String>, // TODO
    netfast: bool, // TODO
    code_profile: Option<String>,
    show_kernel: bool,
    shell: bool,
    no_jni: bool,
    nojni: bool,
    dalvik_vm_checkjni: bool,
    logcat: Option<String>, // TODO
    no_audio: bool,
    noaudio: bool, // TODO
    audio: Option<String>, // TODO
    radio: Option<String>, // TODO
    port: Option<String>, // TODO
    ports: Option<String>, // TODO
    onion: Option<PathBuf>, // TODO
    onion_alph: Option<String>, // TODO
    onion_rotation: Option<String>, // TODO
    dpi_device: Option<String>, // TODO
    scale: Option<String>, // TODO
    wifi_client_port: Option<String>, // TODO
    wifi_server_port: Option<String>, // TODO
    http_proxy: Option<String>, // TODO
    timezone: Option<String>, // TODO
    change_language: Option<String>, // TODO
    change_country: Option<String>, // TODO
    change_locale: Option<String>, // TODO
    dns_server: Option<String>, // TODO
    net_tap: Option<String>, // TODO
    net_tap_script_up: Option<String>, // TODO
    net_tap_script_down: Option<String>, // TODO
    cpu_delay: Option<String>, // TODO
    no_boot_anim: bool,
    no_window: bool,
    qt_hide_window: bool,
    no_sim: bool,
    lowram: bool,
    version: bool,
    no_passive_gps: bool,
    gnss_file_path: Option<PathBuf>,
    gnss_grpc_port: Option<String>, // TODO
    virtio_console: bool,
    read_only: bool,
    is_restart: Option<String>, // TODO
    report_console: Option<String>, // TODO
    gps: Option<String>, // TODO
    shell_serial: Option<String>, // TODO
    tcpdump: Option<PathBuf>,
    bootchart: Option<String>, // TODO
    charmap: Option<PathBuf>,
    studio_params: Option<PathBuf>,
    prop: Option<String>, // TODO
    shared_net_id: Option<String>, // TODO
    gpu: Option<String>, // TODO
    use_host_vulkan: bool,
    camera_back: bool, // TODO
    camera_front: bool, // TODO
    webcam_list: bool,
    virtualscene_poster: Option<String>, // TODO
    screen: bool,
    force_32bit: bool,
    selinux: Option<Selinux>,
    unix_pipe: Option<PathBuf>,
    fixed_scale: bool,
    wait_for_debugger: bool,
    skip_adb_auth: bool,
    metrics_to_console: bool,
    metrics_collection: bool,
    metrics_to_file: Option<PathBuf>,
    detect_image_hang: bool,
    feature: Option<String>, // TODO
    icc_profile: Option<PathBuf>,
    sim_access_rules_file: Option<PathBuf>,
    phone_number: Option<String>, // TODO
    acpi_config: Option<PathBuf>,
    fuchsia: bool,
    window_size: Option<String>, // TODO
    allow_host_audio: bool,
    restart_when_stalled: bool,
    perf_stat: Option<PathBuf>,
    share_vid: bool,
    grpc: Option<String>,  // TODO
    grpc_tls_key: Option<PathBuf>,
    grpc_tls_cer: Option<PathBuf>,
    grpc_tls_ca: Option<PathBuf>,
    grpc_use_token: bool,
    idle_grpc_timeout: Option<String>, // TODO
    waterfall: bool, // TODO
    multidisplay: Option<String>, // TODO
    google_maps_key: Option<String>, // TODO
    no_location_ui: Option<String>, // TODOs
    use_keycode_forwarding: bool, // TODO
    record_session: Option<PathBuf>,
    legacy_fake_camera: bool, // TODO
    no_camera_hq_edge: bool, // TODO
    no_direct_adb: bool,
    check_snapshot_loadable: Option<PathBuf>, // TODO
    no_hidpi_scaling: bool,
    no_mouse_reposition: bool,
    guest_angle: bool,
    qemu: bool,
    qemuh: bool,
    verbose: bool,
    debug: bool, // TODO
    debug_no: bool,
    help: bool,
    help_disk_images: bool,
    help_debug_tags: bool,
    help_char_devices: bool,
    help_environment: bool,
    help_virtual_device: bool,
    help_sdk_images: bool,
    help_build_images: bool,
    help_all: bool,
}


impl Emulator {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// List available AVDs
    pub fn list_avds(&mut self, list_avds: bool) -> &mut Self {
        self.list_avds = list_avds;
        self
    }

    /// Search for system disk images in <dir>
    pub fn sysdir(&mut self, sysdir: &Path) -> &mut Self {
        self.sysdir = Some(sysdir.to_owned());
        self
    }

    /// Read initial system image from <file>
    pub fn system(&mut self, system: &Path) -> &mut Self {
        self.system = Some(system.to_owned());
        self
    }

    /// Read initial vendor image from <file>
    pub fn vendor(&mut self, vendor: &Path) -> &mut Self {
        self.vendor = Some(vendor.to_owned());
        self
    }

    /// Make system & vendor image writable after `adb remount`
    pub fn writable_system(&mut self, writable_system: bool) -> &mut Self {
        self.writable_system = writable_system;
        self
    }

    /// Delay adb communication till boot completes
    pub fn delay_adb(&mut self, delay_adb: bool) -> &mut Self {
        self.delay_adb = delay_adb;
        self
    }

    /// Write user data into <dir>
    pub fn datadir(&mut self, datadir: &Path) -> &mut Self {
        self.datadir = Some(datadir.to_owned());
        self
    }

    /// Use specific emulated kernel
    pub fn kernel(&mut self, kernel: &Path) -> &mut Self {
        self.kernel = Some(kernel.to_owned());
        self
    }

    /// Ramdisk image (default <system>/ramdisk.img)
    pub fn ramdisk(&mut self, ramdisk: &Path) -> &mut Self {
        self.ramdisk = Some(ramdisk.to_owned());
        self
    }

    /// Obsolete, use -system <file> instead
    pub fn image(&mut self, image: &Path) -> &mut Self {
        self.image = Some(image.to_owned());
        self
    }

    /// Same as `-init-data <file>`
    pub fn initdata(&mut self, initdata: &Path) -> &mut Self {
        self.initdata = Some(initdata.to_owned());
        self
    }

    /// Data image (default <datadir>/userdata-qemu.img)
    pub fn data(&mut self, data: &Path) -> &mut Self {
        self.data = Some(data.to_owned());
        self
    }

    /// Read initial encryption key image from <file>
    pub fn encryption_key(&mut self, encryption_key: &Path) -> &mut Self {
        self.encryption_key = Some(encryption_key.to_owned());
        self
    }

    /// Output file of logcat(default none)
    pub fn logcat_output(&mut self, logcat_output: &Path) -> &mut Self {
        self.logcat_output = Some(logcat_output.to_owned());
        self
    }

    /// System/data partition size in MBs
    pub fn partition_size(&mut self, partition_size: u32) -> &mut Self {
        self.partition_size = Some(partition_size);
        self
    }

    /// Cache partition image (default is temporary file)
    pub fn cache(&mut self, cache: &Path) -> &mut Self {
        self.cache = Some(cache.to_owned());
        self
    }

    /// Cache partition size in MBs
    pub fn cache_size(&mut self, cache_size: u32) -> &mut Self {
        self.cache_size = Some(cache_size);
        self
    }

    /// Disable the cache partition
    pub fn no_cache(&mut self, no_cache: bool) -> &mut Self {
        self.no_cache = no_cache;
        self
    }

    /// Same as -no-cache
    pub fn nocache(&mut self, nocache: bool) -> &mut Self {
        self.nocache = nocache;
        self
    }

    /// SD card image (default <datadir>/sdcard.img)
    pub fn sdcard(&mut self, sdcard: &Path) -> &mut Self {
        self.sdcard = Some(sdcard.to_owned());
        self
    }

    /// Qeuit emulator after guest boots completely, or after timeout in seconds
    pub fn quit_after_boot(&mut self, quit_after_boot: u32) -> &mut Self {
        self.quit_after_boot = Some(quit_after_boot);
        self
    }

    /// Use the emulator in the specified dir (relative or absolute path)v
    pub fn qemu_top_dir(&mut self, qemu_top_dir: &Path) -> &mut Self {
        self.qemu_top_dir = Some(qemu_top_dir.to_owned());
        self
    }

    /// Monitor the adb messages between guest and host, default not
    pub fn monitor_adb(&mut self, monitor_adb: bool) -> &mut Self {
        self.monitor_adb = monitor_adb;
        self
    }

    /// File that contains all state snapshots (default <datadir>/snapshots.img)
    pub fn snapstorage(&mut self, snapstorage: &Path) -> &mut Self {
        self.snapstorage = Some(snapstorage.to_owned());
        self
    }

    /// Do not mount a snapshot storage file (this disables all snapshot functionality)
    pub fn no_snapstorage(&mut self, no_snapstorage: bool) -> &mut Self {
        self.no_snapstorage = no_snapstorage;
        self
    }

    /// Name of snapshot within storage file for auto-start and auto-save (default 'default-boot')
    pub fn snapshot(&mut self, snapshot: String) -> &mut Self {
        self.snapshot = Some(snapshot);
        self
    }

    /// Perform a full boot and do not auto-save, but qemu vmload and vmsave operate on snapstorage
    pub fn no_snapshot(&mut self, no_snapshot: bool) -> &mut Self {
        self.no_snapshot = no_snapshot;
        self
    }

    /// Do not auto-save to snapshot on exit: abandon changed state
    pub fn no_snapshot_save(&mut self, no_snapshot_save: bool) -> &mut Self {
        self.no_snapshot_save = no_snapshot_save;
        self
    }

    /// Do not auto-start from snapshot: perform a full boot
    pub fn no_snapshot_load(&mut self, no_snapshot_load: bool) -> &mut Self {
        self.no_snapshot_load = no_snapshot_load;
        self
    }

    /// Show a list of available snapshots
    pub fn snapshot_list(&mut self, snapshot_list: bool) -> &mut Self {
        self.snapshot_list = snapshot_list;
        self
    }

    /// Do not try to correct snapshot time on restore
    pub fn no_snapshot_update_time(&mut self, no_snapshot_update_time: bool) -> &mut Self {
        self.no_snapshot_update_time = no_snapshot_update_time;
        self
    }

    /// Reset the user data image (copy it from initdata)
    pub fn wipe_data(&mut self, wipe_data: bool) -> &mut Self {
        self.wipe_data = wipe_data;
        self
    }

    /// Use a specific android virtual device
    pub fn avd(&mut self, avd: String) -> &mut Self {
        self.avd = Some(avd);
        self
    }

    /// Use a specific target architecture
    pub fn avd_arch(&mut self, avd_arch: String) -> &mut Self {
        self.avd_arch = Some(avd_arch);
        self
    }

    /// Search skins in <dir> (default <system>/skins)
    pub fn skindir(&mut self, skindir: &Path) -> &mut Self {
        self.skindir = Some(skindir.to_owned());
        self
    }

    /// Select a given skin
    pub fn skin(&mut self, skin: String) -> &mut Self {
        self.skin = Some(skin);
        self
    }

    /// Deprecated: create an AVD with no skin instead
    pub fn no_skin(&mut self, no_skin: bool) -> &mut Self {
        self.no_skin = no_skin;
        self
    }

    /// Same as -no-skin
    pub fn noskin(&mut self, noskin: bool) -> &mut Self {
        self.noskin = noskin;
        self
    }

    /// Physical RAM size in MBs
    pub fn memory(&mut self, memory: u32) -> &mut Self {
        self.memory = Some(memory);
        self
    }

    /// Run only the UI feature requested
    pub fn ui_only(&mut self, ui_only: String) -> &mut Self {
        self.ui_only = Some(ui_only);
        self
    }

    /// Assign an id to this virtual device (separate from the avd name)
    pub fn id(&mut self, id: String) -> &mut Self {
        self.id = Some(id);
        self
    }

    /// Set number of CPU cores to emulator
    pub fn cores(&mut self, cores: u32) -> &mut Self {
        self.cores = Some(cores);
        self
    }

    /// Configure emulation acceleration
    pub fn accel(&mut self, accel: String) -> &mut Self {
        self.accel = Some(accel);
        self
    }

    /// Same as '-accel off'
    pub fn no_accel(&mut self, no_accel: bool) -> &mut Self {
        self.no_accel = no_accel;
        self
    }

    /// Use new emulator backend instead of the classic one
    pub fn ranchu(&mut self, ranchu: bool) -> &mut Self {
        self.ranchu = ranchu;
        self
    }

    pub fn engine(&mut self, engine: String) -> &mut Self {
        self.engine = Some(engine);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selinux {
    Disabled,
    Permissive,
}