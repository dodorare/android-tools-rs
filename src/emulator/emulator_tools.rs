use crate::emulator::*;
use crate::error::*;
use std::{
    path::{Path, PathBuf},
};

#[derive(Clone, Default)]
pub struct EmulatorTools {
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
    accel: Option<AccelMode>,
    no_accel: bool,
    ranchu: bool,
    engine: Option<Engine>,
    netspeed: Option<Netspeed>,
    netdelay: Option<String>,
    netfast: bool,
    code_profile: Option<String>,
    show_kernel: bool,
    shell: bool,
    no_jni: bool,
    nojni: bool,
    dalvik_vm_checkjni: bool,
    logcat: Option<String>,
    no_audio: bool,
    noaudio: bool,
    audio: Option<String>,
    radio: Option<String>,
    port: Option<String>,
    ports: Option<Vec<String>>,
    onion: Option<PathBuf>,
    onion_alph: Option<String>,
    onion_rotation: Option<String>,
    dpi_device: Option<String>,
    scale: Option<String>,
    wifi_client_port: Option<String>,
    wifi_server_port: Option<String>,
    http_proxy: Option<String>,
    timezone: Option<String>,
    change_language: Option<String>,
    change_country: Option<String>,
    change_locale: Option<String>,
    dns_server: Option<String>,
    net_tap: Option<String>,
    net_tap_script_up: Option<String>,
    net_tap_script_down: Option<String>,
    cpu_delay: Option<String>,
    no_boot_anim: bool,
    no_window: bool,
    qt_hide_window: bool,
    no_sim: bool,
    lowram: bool,
    version: bool,
    no_passive_gps: bool,
    gnss_file_path: Option<PathBuf>,
    gnss_grpc_port: Option<String>,
    virtio_console: bool,
    read_only: bool,
    is_restart: Option<String>,
    report_console: Option<String>,
    gps: Option<String>,
    shell_serial: Option<String>,
    tcpdump: Option<PathBuf>,
    bootchart: Option<String>,
    charmap: Option<PathBuf>,
    studio_params: Option<PathBuf>,
    prop: Option<String>,
    shared_net_id: Option<String>,
    gpu: Option<String>,
    use_host_vulkan: bool,
    camera_back: Option<CameraMode>,
    camera_front: Option<CameraMode>,
    webcam_list: bool,
    virtualscene_poster: Option<String>,
    screen: Option<ScreenMode>,
    force_32bit: bool,
    selinux: Option<SELinux>,
    unix_pipe: Option<PathBuf>,
    fixed_scale: bool,
    wait_for_debugger: bool,
    skip_adb_auth: bool,
    metrics_to_console: bool,
    metrics_collection: bool,
    metrics_to_file: Option<PathBuf>,
    detect_image_hang: bool,
    feature: Option<String>,
    icc_profile: Option<PathBuf>,
    sim_access_rules_file: Option<PathBuf>,
    phone_number: Option<String>,
    acpi_config: Option<PathBuf>,
    fuchsia: bool,
    window_size: Option<String>,
    allow_host_audio: bool,
    restart_when_stalled: bool,
    perf_stat: Option<PathBuf>,
    share_vid: bool,
    grpc: Option<String>,
    grpc_tls_key: Option<PathBuf>,
    grpc_tls_cer: Option<PathBuf>,
    grpc_tls_ca: Option<PathBuf>,
    grpc_use_token: bool,
    idle_grpc_timeout: Option<u32>,
    waterfall: Option<String>,
    multidisplay: Option<String>,
    google_maps_key: Option<String>,
    no_location_ui: bool,
    use_keycode_forwarding: bool,
    record_session: Option<PathBuf>,
    legacy_fake_camera: bool,
    no_camera_hq_edge: bool,
    no_direct_adb: bool,
    check_snapshot_loadable: Option<String>,
    no_hidpi_scaling: bool,
    no_mouse_reposition: bool,
    guest_angle: bool,
    qemu: bool,
    verbose: bool,
    debug: Option<DebugTags>,
    debug_no: Option<DebugTags>,
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

impl EmulatorTools {
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
    pub fn accel(&mut self, accel: AccelMode) -> &mut Self {
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

    /// Select engine. auto|classic|qemu2
    pub fn engine(&mut self, engine: Engine) -> &mut Self {
        self.engine = Some(engine);
        self
    }

    /// Maximum network download/upload speeds
    pub fn netspeed(&mut self, netspeed: Netspeed) -> &mut Self {
        self.netspeed = Some(netspeed);
        self
    }

    /// Network latency emulation
    pub fn netdelay(&mut self, netdelay: String) -> &mut Self {
        self.netdelay = Some(netdelay);
        self
    }

    /// Disable network shaping
    pub fn netfast(&mut self, netfast: bool) -> &mut Self {
        self.netfast = netfast;
        self
    }

    /// Enable code profiling
    pub fn code_profile(&mut self, code_profile: String) -> &mut Self {
        self.code_profile = Some(code_profile);
        self
    }

    /// Display kernel messages
    pub fn show_kernel(&mut self, show_kernel: bool) -> &mut Self {
        self.show_kernel = show_kernel;
        self
    }

    /// Enable root shell on current terminal
    pub fn shell(&mut self, shell: bool) -> &mut Self {
        self.shell = shell;
        self
    }

    /// Deprecated, see dalvik_vm_checkjni
    pub fn no_jni(&mut self, no_jni: bool) -> &mut Self {
        self.no_jni = no_jni;
        self
    }

    /// Deprecated, see dalvik_vm_checkjni
    pub fn nojni(&mut self, nojni: bool) -> &mut Self {
        self.nojni = nojni;
        self
    }

    /// Enable dalvik.vm.checkjni
    pub fn dalvik_vm_checkjni(&mut self, dalvik_vm_checkjni: bool) -> &mut Self {
        self.dalvik_vm_checkjni = dalvik_vm_checkjni;
        self
    }

    /// Enable logcat output with given tags
    pub fn logcat(&mut self, logcat: String) -> &mut Self {
        self.logcat = Some(logcat);
        self
    }

    /// Disable audio support
    pub fn no_audio(&mut self, no_audio: bool) -> &mut Self {
        self.no_audio = no_audio;
        self
    }

    /// Same as -no-audio
    pub fn noaudio(&mut self, noaudio: bool) -> &mut Self {
        self.noaudio = noaudio;
        self
    }

    /// Use specific audio backend
    pub fn audio(&mut self, audio: String) -> &mut Self {
        self.audio = Some(audio);
        self
    }

    /// Redirect radio modem interface to character device
    pub fn radio(&mut self, radio: String) -> &mut Self {
        self.radio = Some(radio);
        self
    }

    /// TCP port that will be used for the console
    pub fn port(&mut self, port: String) -> &mut Self {
        self.port = Some(port);
        self
    }

    /// TCP ports used for the console and adb bridge
    pub fn ports(&mut self, ports: Vec<String>) -> &mut Self {
        self.ports = Some(ports);
        self
    }

    /// Use overlay PNG image over screen
    pub fn onion(&mut self, onion: &Path) -> &mut Self {
        self.onion = Some(onion.to_owned());
        self
    }

    /// Specify onion-skin translucency
    pub fn onion_alph(&mut self, onion_alph: String) -> &mut Self {
        self.onion_alph = Some(onion_alph);
        self
    }

    /// specify onion-skin rotation
    pub fn onion_rotation(&mut self, onion_rotation: String) -> &mut Self {
        self.onion_rotation = Some(onion_rotation);
        self
    }

    /// Specify device's resolution in dpi (default DEFAULT_DEVICE_DPI)
    pub fn dpi_device(&mut self, dpi_device: String) -> &mut Self {
        self.dpi_device = Some(dpi_device);
        self
    }

    /// Scale emulator window (deprecated)
    pub fn scale(&mut self, scale: String) -> &mut Self {
        self.scale = Some(scale);
        self
    }

    /// Connect to other emulator for WiFi forwarding
    pub fn wifi_client_port(&mut self, wifi_client_port: String) -> &mut Self {
        self.wifi_client_port = Some(wifi_client_port);
        self
    }

    /// Listen to other emulator for WiFi forwarding
    pub fn wifi_server_port(&mut self, wifi_server_port: String) -> &mut Self {
        self.wifi_server_port = Some(wifi_server_port);
        self
    }

    /// Make TCP connections through a HTTP/HTTPS proxy
    pub fn http_proxy(&mut self, http_proxy: String) -> &mut Self {
        self.http_proxy = Some(http_proxy);
        self
    }

    /// Use this timezone instead of the host's default
    pub fn timezone(&mut self, timezone: String) -> &mut Self {
        self.timezone = Some(timezone);
        self
    }

    /// Use this language instead of the current one. Restarts the framework
    pub fn change_language(&mut self, change_language: String) -> &mut Self {
        self.change_language = Some(change_language);
        self
    }

    /// Use this country instead of the current one. Restarts the framework
    pub fn change_country(&mut self, change_country: String) -> &mut Self {
        self.change_country = Some(change_country);
        self
    }

    /// Use this locale instead of the current one. Restarts the framework
    pub fn change_locale(&mut self, change_locale: String) -> &mut Self {
        self.change_locale = Some(change_locale);
        self
    }

    /// Use this DNS server(s) in the emulated system
    pub fn dns_server(&mut self, dns_server: String) -> &mut Self {
        self.dns_server = Some(dns_server);
        self
    }

    /// Use this TAP interface for networking
    pub fn net_tap(&mut self, net_tap: String) -> &mut Self {
        self.net_tap = Some(net_tap);
        self
    }

    /// Script to run when the TAP interface goes up
    pub fn net_tap_script_up(&mut self, net_tap_script_up: String) -> &mut Self {
        self.net_tap_script_up = Some(net_tap_script_up);
        self
    }

    /// Script to run when the TAP interface goes down
    pub fn net_tap_script_down(&mut self, net_tap_script_down: String) -> &mut Self {
        self.net_tap_script_down = Some(net_tap_script_down);
        self
    }

    /// Throttle CPU emulation
    pub fn cpu_delay(&mut self, cpu_delay: String) -> &mut Self {
        self.cpu_delay = Some(cpu_delay);
        self
    }

    /// Disable animation for faster boot
    pub fn no_boot_anim(&mut self, no_boot_anim: bool) -> &mut Self {
        self.no_boot_anim = no_boot_anim;
        self
    }

    /// Disable graphical window display
    pub fn no_window(&mut self, no_window: bool) -> &mut Self {
        self.no_window = no_window;
        self
    }

    /// Start QT window but hide window display
    pub fn qt_hide_window(&mut self, qt_hide_window: bool) -> &mut Self {
        self.qt_hide_window = qt_hide_window;
        self
    }

    /// Device has no SIM card
    pub fn no_sim(&mut self, no_sim: bool) -> &mut Self {
        self.no_sim = no_sim;
        self
    }

    /// Device is a low ram device
    pub fn lowram(&mut self, lowram: bool) -> &mut Self {
        self.lowram = lowram;
        self
    }

    /// Display emulator version number
    pub fn version(&mut self, version: bool) -> &mut Self {
        self.version = version;
        self
    }

    /// Disable passive gps updates
    pub fn no_passive_gps(&mut self, no_passive_gps: bool) -> &mut Self {
        self.no_passive_gps = no_passive_gps;
        self
    }

    /// Use the specified filepath to read gnss data
    pub fn gnss_file_path(&mut self, gnss_file_path: &Path) -> &mut Self {
        self.gnss_file_path = Some(gnss_file_path.to_owned());
        self
    }

    /// Use the specified port number to start grpc service to receive gnss data
    pub fn gnss_grpc_port(&mut self, gnss_grpc_port: String) -> &mut Self {
        self.gnss_grpc_port = Some(gnss_grpc_port);
        self
    }

    /// Using virtio console as console
    pub fn virtio_console(&mut self, virtio_console: bool) -> &mut Self {
        self.virtio_console = virtio_console;
        self
    }

    /// Allow running multiple instances of emulators on the same AVD, but cannot
    /// save snapshot
    pub fn read_only(&mut self, read_only: bool) -> &mut Self {
        self.read_only = read_only;
        self
    }

    /// Specifies that this emulator was a restart, and to wait out <restart-pid>
    /// before proceeding
    pub fn is_restart(&mut self, is_restart: String) -> &mut Self {
        self.is_restart = Some(is_restart);
        self
    }

    /// Report console port to remote socket
    pub fn report_console(&mut self, report_console: String) -> &mut Self {
        self.report_console = Some(report_console);
        self
    }

    /// Redirect NMEA GPS to character device
    pub fn gps(&mut self, gps: String) -> &mut Self {
        self.gps = Some(gps);
        self
    }

    /// Specific character device for root shell
    pub fn shell_serial(&mut self, shell_serial: String) -> &mut Self {
        self.shell_serial = Some(shell_serial);
        self
    }

    /// Capture network packets to file
    pub fn tcpdump(&mut self, tcpdump: &Path) -> &mut Self {
        self.tcpdump = Some(tcpdump.to_owned());
        self
    }

    /// Enable bootcharting
    pub fn bootchart(&mut self, bootchart: String) -> &mut Self {
        self.bootchart = Some(bootchart);
        self
    }

    /// Use specific key character map
    pub fn charmap(&mut self, charmap: &Path) -> &mut Self {
        self.charmap = Some(charmap.to_owned());
        self
    }

    /// Used by Android Studio to provide parameters
    pub fn studio_params(&mut self, studio_params: &Path) -> &mut Self {
        self.studio_params = Some(studio_params.to_owned());
        self
    }

    /// Set system property on boot
    pub fn prop(&mut self, prop: String) -> &mut Self {
        self.prop = Some(prop);
        self
    }

    /// Join the shared network, using IP address 10.1.2.<number>
    pub fn shared_net_id(&mut self, shared_net_id: String) -> &mut Self {
        self.shared_net_id = Some(shared_net_id);
        self
    }

    /// Set hardware OpenGLES emulation mode
    pub fn gpu(&mut self, gpu: String) -> &mut Self {
        self.gpu = Some(gpu);
        self
    }

    /// Use host for vulkan emulation regardless of 'gpu' mode
    pub fn use_host_vulkan(&mut self, use_host_vulkan: bool) -> &mut Self {
        self.use_host_vulkan = use_host_vulkan;
        self
    }

    /// Set emulation mode for a camera facing back
    pub fn camera_back(&mut self, camera_back: CameraMode) -> &mut Self {
        self.camera_back = Some(camera_back);
        self
    }

    /// Set emulation mode for a camera facing front
    pub fn camera_front(&mut self, camera_front: CameraMode) -> &mut Self {
        self.camera_front = Some(camera_front);
        self
    }

    /// Lists web cameras available for emulation
    pub fn webcam_list(&mut self, webcam_list: bool) -> &mut Self {
        self.webcam_list = webcam_list;
        self
    }

    /// Load a png or jpeg image as a poster in the virtual scene
    pub fn virtualscene_poster(&mut self, virtualscene_poster: String) -> &mut Self {
        self.virtualscene_poster = Some(virtualscene_poster);
        self
    }

    /// Set emulated screen mode
    pub fn screen(&mut self, screen: ScreenMode) -> &mut Self {
        self.screen = Some(screen);
        self
    }

    /// Always use 32-bit emulator
    pub fn force_32bit(&mut self, force_32bit: bool) -> &mut Self {
        self.force_32bit = force_32bit;
        self
    }

    /// Set SELinux to either disabled or permissive mode
    pub fn selinux(&mut self, selinux: SELinux) -> &mut Self {
        self.selinux = Some(selinux);
        self
    }

    /// Add <path> to the list of allowed Unix pipes
    pub fn unix_pipe(&mut self, unix_pipe: &Path) -> &mut Self {
        self.unix_pipe = Some(unix_pipe.to_owned());
        self
    }

    /// Use fixed 1:1 scale for the initial emulator window
    pub fn fixed_scale(&mut self, fixed_scale: bool) -> &mut Self {
        self.fixed_scale = fixed_scale;
        self
    }

    /// Pause on launch and wait for a debugger process to attach before resuming
    pub fn wait_for_debugger(&mut self, wait_for_debugger: bool) -> &mut Self {
        self.wait_for_debugger = wait_for_debugger;
        self
    }

    /// Skip adb authentication dialogue
    pub fn skip_adb_auth(&mut self, skip_adb_auth: bool) -> &mut Self {
        self.skip_adb_auth = skip_adb_auth;
        self
    }

    /// Enable usage metrics and print the messages to stdout
    pub fn metrics_to_console(&mut self, metrics_to_console: bool) -> &mut Self {
        self.metrics_to_console = metrics_to_console;
        self
    }

    /// Enable usage metrics and send them to google play
    pub fn metrics_collection(&mut self, metrics_collection: bool) -> &mut Self {
        self.metrics_collection = metrics_collection;
        self
    }

    /// Enable usage metrics and write the messages into specified file
    pub fn metrics_to_file(&mut self, metrics_to_file: &Path) -> &mut Self {
        self.metrics_to_file = Some(metrics_to_file.to_owned());
        self
    }

    /// Enable the detection of system image hangs
    pub fn detect_image_hang(&mut self, detect_image_hang: bool) -> &mut Self {
        self.detect_image_hang = detect_image_hang;
        self
    }

    /// Force-enable or disable (-name) the features
    pub fn feature(&mut self, feature: String) -> &mut Self {
        self.feature = Some(feature);
        self
    }

    /// Use icc profile from specified file
    pub fn icc_profile(&mut self, icc_profile: &Path) -> &mut Self {
        self.icc_profile = Some(icc_profile.to_owned());
        self
    }

    /// Use SIM access rules from specified file
    pub fn sim_access_rules_file(&mut self, sim_access_rules_file: &Path) -> &mut Self {
        self.sim_access_rules_file = Some(sim_access_rules_file.to_owned());
        self
    }

    /// Sets the phone number of the emulated device
    pub fn phone_number(&mut self, phone_number: String) -> &mut Self {
        self.phone_number = Some(phone_number);
        self
    }

    /// Specify acpi device proprerties (hierarchical key=value pair)
    pub fn acpi_config(&mut self, acpi_config: &Path) -> &mut Self {
        self.acpi_config = Some(acpi_config.to_owned());
        self
    }

    /// Run Fuchsia image. Bypasses android-specific setup; args after are treated
    /// as standard QEMU args
    pub fn fuchsia(&mut self, fuchsia: bool) -> &mut Self {
        self.fuchsia = fuchsia;
        self
    }

    /// Set window size for when bypassing android-specific setup
    pub fn window_size(&mut self, window_size: String) -> &mut Self {
        self.window_size = Some(window_size);
        self
    }

    /// Allows sending of audio from audio input devices. Otherwise, zeroes
    /// out audio
    pub fn allow_host_audio(&mut self, allow_host_audio: bool) -> &mut Self {
        self.allow_host_audio = allow_host_audio;
        self
    }

    /// Allows restarting guest when it is stalled
    pub fn restart_when_stalled(&mut self, restart_when_stalled: bool) -> &mut Self {
        self.restart_when_stalled = restart_when_stalled;
        self
    }

    /// Run periodic perf stat reporter in the background and write output to
    /// specified file
    pub fn perf_stat(&mut self, perf_stat: &Path) -> &mut Self {
        self.perf_stat = Some(perf_stat.to_owned());
        self
    }

    /// Share current video state in shared memory region
    pub fn share_vid(&mut self, share_vid: bool) -> &mut Self {
        self.share_vid = share_vid;
        self
    }

    /// TCP ports used for the gRPC bridge
    pub fn grpc(&mut self, grpc: String) -> &mut Self {
        self.grpc = Some(grpc);
        self
    }

    /// File with the private key used to enable gRPC TLS
    pub fn grpc_tls_key(&mut self, grpc_tls_key: &Path) -> &mut Self {
        self.grpc_tls_key = Some(grpc_tls_key.to_owned());
        self
    }

    /// File with the public X509 certificate used to enable gRPC TLS
    pub fn grpc_tls_cer(&mut self, grpc_tls_cer: &Path) -> &mut Self {
        self.grpc_tls_cer = Some(grpc_tls_cer.to_owned());
        self
    }

    /// File with the Certificate Authorities used to validate client certificates
    pub fn grpc_tls_ca(&mut self, grpc_tls_ca: &Path) -> &mut Self {
        self.grpc_tls_ca = Some(grpc_tls_ca.to_owned());
        self
    }

    /// Use the emulator console token for gRPC authentication
    pub fn grpc_use_token(&mut self, grpc_use_token: bool) -> &mut Self {
        self.grpc_use_token = grpc_use_token;
        self
    }

    /// Terminate the emulator if there is no gRPC activity within <timeout> seconds
    pub fn idle_grpc_timeout(&mut self, idle_grpc_timeout: u32) -> &mut Self {
        self.idle_grpc_timeout = Some(idle_grpc_timeout);
        self
    }

    /// Mode in which to run waterfall
    pub fn waterfall(&mut self, waterfall: String) -> &mut Self {
        self.waterfall = Some(waterfall);
        self
    }

    /// Config multiple displays
    pub fn multidisplay(&mut self, multidisplay: String) -> &mut Self {
        self.multidisplay = Some(multidisplay);
        self
    }

    /// API key to use with the Google Maps GUI
    pub fn google_maps_key(&mut self, google_maps_key: String) -> &mut Self {
        self.google_maps_key = Some(google_maps_key);
        self
    }

    /// Disable the location UI in the extended window
    pub fn no_location_ui(&mut self, no_location_ui: bool) -> &mut Self {
        self.no_location_ui = no_location_ui;
        self
    }

    /// Use keycode forwarding instead of host charmap translation
    pub fn use_keycode_forwarding(&mut self, use_keycode_forwarding: bool) -> &mut Self {
        self.use_keycode_forwarding = use_keycode_forwarding;
        self
    }

    /// Screen record the emulator session
    pub fn record_session(&mut self, record_session: &Path) -> &mut Self {
        self.record_session = Some(record_session.to_owned());
        self
    }

    /// Use legacy camera HAL for the emulated fake camera
    pub fn legacy_fake_camera(&mut self, legacy_fake_camera: bool) -> &mut Self {
        self.legacy_fake_camera = legacy_fake_camera;
        self
    }

    /// Disable high qualify edge processing for emulated camera
    pub fn no_camera_hq_edge(&mut self, no_camera_hq_edge: bool) -> &mut Self {
        self.no_camera_hq_edge = no_camera_hq_edge;
        self
    }

    /// Use external adb executable for internal communication
    pub fn no_direct_adb(&mut self, no_direct_adb: bool) -> &mut Self {
        self.no_direct_adb = no_direct_adb;
        self
    }

    /// Check if a snasphot is loadable
    pub fn check_snapshot_loadable(&mut self, check_snapshot_loadable: String) -> &mut Self {
        self.check_snapshot_loadable = Some(check_snapshot_loadable);
        self
    }

    /// Disable HiDPI scaling of guest display on macOS devices
    pub fn no_hidpi_scaling(&mut self, no_hidpi_scaling: bool) -> &mut Self {
        self.no_hidpi_scaling = no_hidpi_scaling;
        self
    }

    /// Do not reposition the mouse to emulator window center if mouse pointer gets
    /// out of the window
    pub fn no_mouse_reposition(&mut self, no_mouse_reposition: bool) -> &mut Self {
        self.no_mouse_reposition = no_mouse_reposition;
        self
    }

    /// Enable guest ANGLE as system driver
    pub fn guest_angle(&mut self, guest_angle: bool) -> &mut Self {
        self.guest_angle = guest_angle;
        self
    }

    /// Pass arguments to qemu
    pub fn qemu(&mut self, qemu: bool) -> &mut Self {
        self.qemu = qemu;
        self
    }

    /// Same as '-debug-init'
    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose = verbose;
        self
    }

    /// Enable/disable debug messages
    pub fn debug(&mut self, debug: DebugTags) -> &mut Self {
        self.debug = Some(debug);
        self
    }

    pub fn debug_no(&mut self, debug_no: DebugTags) -> &mut Self {
        self.debug_no = Some(debug_no);
        self
    }

    /// Print this help
    pub fn help(&mut self, help: bool) -> &mut Self {
        self.help = help;
        self
    }

    /// About disk images
    pub fn help_disk_images(&mut self, help_disk_images: bool) -> &mut Self {
        self.help_disk_images = help_disk_images;
        self
    }

    /// Debug tags for -debug <tags>
    pub fn help_debug_tags(&mut self, help_debug_tags: bool) -> &mut Self {
        self.help_debug_tags = help_debug_tags;
        self
    }

    /// Character <device> specification
    pub fn help_char_devices(&mut self, help_char_devices: bool) -> &mut Self {
        self.help_char_devices = help_char_devices;
        self
    }

    /// Environment variables
    pub fn help_environment(&mut self, help_environment: bool) -> &mut Self {
        self.help_environment = help_environment;
        self
    }

    /// Virtual device management
    pub fn help_virtual_device(&mut self, help_virtual_device: bool) -> &mut Self {
        self.help_virtual_device = help_virtual_device;
        self
    }

    /// About disk images when using the SDK
    pub fn help_sdk_images(&mut self, help_sdk_images: bool) -> &mut Self {
        self.help_sdk_images = help_sdk_images;
        self
    }

    /// About disk images when building Android
    pub fn help_build_images(&mut self, help_build_images: bool) -> &mut Self {
        self.help_build_images = help_build_images;
        self
    }

    /// Prints all help content
    pub fn help_all(&mut self, help_all: bool) -> &mut Self {
        self.help_all = help_all;
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut emulator = emulator_tool()?;
        if self.list_avds {
            emulator.arg("-list-avds");
        }
        if let Some(sysdir) = &self.sysdir {
            emulator.arg("-sysdir").arg(sysdir);
        }
        if let Some(system) = &self.system {
            emulator.arg("-system").arg(system);
        }
        if let Some(vendor) = &self.vendor {
            emulator.arg("-vendor").arg(vendor);
        }
        if self.writable_system {
            emulator.arg("-writable-system");
        }
        if self.delay_adb {
            emulator.arg("-delay-adb");
        }
        if let Some(datadir) = &self.datadir {
            emulator.arg("-datadir").arg(datadir);
        }
        if let Some(kernel) = &self.kernel {
            emulator.arg("-kernel").arg(kernel);
        }
        if let Some(ramdisk) = &self.ramdisk {
            emulator.arg("-ramdisk").arg(ramdisk);
        }
        if let Some(image) = &self.image {
            emulator.arg("-image").arg(image);
        }
        if let Some(initdata) = &self.initdata {
            emulator.arg("-initdata").arg(initdata);
        }
        if let Some(data) = &self.data {
            emulator.arg("-data").arg(data);
        }
        if let Some(encryption_key) = &self.encryption_key {
            emulator.arg("-encryption-key").arg(encryption_key);
        }
        if let Some(logcat_output) = &self.logcat_output {
            emulator.arg("-logcat-output").arg(logcat_output);
        }
        if let Some(partition_size) = &self.partition_size {
            emulator
                .arg("-partition-size")
                .arg(partition_size.to_string());
        }
        if let Some(cache) = &self.cache {
            emulator.arg("-cache").arg(cache);
        }
        if let Some(cache_size) = &self.cache_size {
            emulator.arg("-cache-size").arg(cache_size.to_string());
        }
        if self.no_cache {
            emulator.arg("-no-cache");
        }
        if self.nocache {
            emulator.arg("-nocache");
        }
        if let Some(sdcard) = &self.sdcard {
            emulator.arg("-sdcard").arg(sdcard);
        }
        if let Some(quit_after_boot) = &self.quit_after_boot {
            emulator
                .arg("-quit-after-boot")
                .arg(quit_after_boot.to_string());
        }
        if let Some(qemu_top_dir) = &self.qemu_top_dir {
            emulator.arg("-qemu-top-dir").arg(qemu_top_dir);
        }
        if self.monitor_adb {
            emulator.arg("-monitor-adb");
        }
        if let Some(snapstorage) = &self.snapstorage {
            emulator.arg("-snapstorage").arg(snapstorage);
        }
        if self.no_snapstorage {
            emulator.arg("-no-snapstorage");
        }
        if let Some(snapshot) = &self.snapshot {
            emulator.arg("-snapshot").arg(snapshot);
        }
        if self.no_snapshot {
            emulator.arg("-no-snapshot");
        }
        if self.no_snapshot_save {
            emulator.arg("-no-snapshot-save");
        }
        if self.no_snapshot_load {
            emulator.arg("-no-snapshot-load");
        }
        if self.snapshot_list {
            emulator.arg("-snapshot-list");
        }
        if self.no_snapshot_update_time {
            emulator.arg("-no-snapshot-update-time");
        }
        if self.wipe_data {
            emulator.arg("-wipe-data");
        }
        if let Some(avd) = &self.avd {
            emulator.arg("-avd").arg(avd);
        }
        if let Some(avd_arch) = &self.avd_arch {
            emulator.arg("-avd-arch").arg(avd_arch);
        }
        if let Some(skindir) = &self.skindir {
            emulator.arg("-skindir").arg(skindir);
        }
        if let Some(skin) = &self.skin {
            emulator.arg("-skin").arg(skin);
        }
        if self.no_skin {
            emulator.arg("-no-skin");
        }
        if self.noskin {
            emulator.arg("-noskin");
        }
        if let Some(memory) = &self.memory {
            emulator.arg("-memory").arg(memory.to_string());
        }
        if let Some(ui_only) = &self.ui_only {
            emulator.arg("-ui-only").arg(ui_only);
        }
        if let Some(id) = &self.id {
            emulator.arg("-id").arg(id);
        }
        if let Some(cores) = &self.cores {
            emulator.arg("-id").arg(cores.to_string());
        }
        if let Some(accel) = &self.accel {
            emulator.arg("-id").arg(accel.to_string());
        }
        if self.no_accel {
            emulator.arg("-no-accel");
        }
        if self.ranchu {
            emulator.arg("-ranchu");
        }
        if let Some(engine) = &self.engine {
            emulator.arg("-engine").arg(engine.to_string());
        }
        if let Some(netspeed) = &self.netspeed {
            emulator.arg("-netspeed").arg(netspeed.to_string());
        }
        if let Some(netdelay) = &self.netdelay {
            emulator.arg("-netdelay").arg(netdelay);
        }
        if self.netfast {
            emulator.arg("-netfast");
        }
        if let Some(code_profile) = &self.code_profile {
            emulator.arg("-code-profile").arg(code_profile);
        }
        if self.show_kernel {
            emulator.arg("-show-kernel");
        }
        if self.shell {
            emulator.arg("-shell");
        }
        if self.no_jni {
            emulator.arg("-no-jni");
        }
        if self.nojni {
            emulator.arg("-nojni");
        }
        if self.dalvik_vm_checkjni {
            emulator.arg("-dalvik-vm-checkjni");
        }
        if let Some(logcat) = &self.logcat {
            emulator.arg("-logcat").arg(logcat);
        }
        if self.no_audio {
            emulator.arg("-no-audio");
        }
        if self.noaudio {
            emulator.arg("-noaudio");
        }
        if let Some(audio) = &self.audio {
            emulator.arg("-audio").arg(audio);
        }
        if let Some(radio) = &self.radio {
            emulator.arg("-radio").arg(radio);
        }
        if let Some(port) = &self.port {
            emulator.arg("-port").arg(port);
        }
        if let Some(ports) = &self.ports {
            emulator.arg("-ports").arg(
                ports
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(onion) = &self.onion {
            emulator.arg("-onion").arg(onion);
        }
        if let Some(onion_alph) = &self.onion_alph {
            emulator.arg("-onion-alpha").arg(onion_alph);
        }
        if let Some(onion_rotation) = &self.onion_rotation {
            emulator.arg("-onion-rotation").arg(onion_rotation);
        }
        if let Some(dpi_device) = &self.dpi_device {
            emulator.arg("-dpi-device").arg(dpi_device);
        }
        if let Some(scale) = &self.scale {
            emulator.arg("-scale").arg(scale);
        }
        if let Some(wifi_client_port) = &self.wifi_client_port {
            emulator.arg("-wifi-client-port").arg(wifi_client_port);
        }
        if let Some(wifi_server_port) = &self.wifi_server_port {
            emulator.arg("-wifi-server-port").arg(wifi_server_port);
        }
        if let Some(http_proxy) = &self.http_proxy {
            emulator.arg("-http-proxy").arg(http_proxy);
        }
        if let Some(timezone) = &self.timezone {
            emulator.arg("-timezone").arg(timezone);
        }
        if let Some(change_language) = &self.change_language {
            emulator.arg("-change-language").arg(change_language);
        }
        if let Some(change_country) = &self.change_country {
            emulator.arg("-change-country").arg(change_country);
        }
        if let Some(change_locale) = &self.change_locale {
            emulator.arg("-change-locale").arg(change_locale);
        }
        if let Some(dns_server) = &self.dns_server {
            emulator.arg("-dns-server").arg(dns_server);
        }
        if let Some(net_tap) = &self.net_tap {
            emulator.arg("-net-tap").arg(net_tap);
        }
        if let Some(net_tap_script_up) = &self.net_tap_script_up {
            emulator.arg("-net-tap-script-up").arg(net_tap_script_up);
        }
        if let Some(net_tap_script_down) = &self.net_tap_script_down {
            emulator
                .arg("-net-tap-script-down")
                .arg(net_tap_script_down);
        }
        if let Some(cpu_delay) = &self.cpu_delay {
            emulator.arg("-cpu-delay").arg(cpu_delay);
        }
        if self.no_boot_anim {
            emulator.arg("-no-boot-anim");
        }
        if self.no_window {
            emulator.arg("-no-window");
        }
        if self.qt_hide_window {
            emulator.arg("-qt-hide-window");
        }
        if self.no_sim {
            emulator.arg("-no-sim");
        }
        if self.lowram {
            emulator.arg("-lowram");
        }
        if self.version {
            emulator.arg("-version");
        }
        if self.no_passive_gps {
            emulator.arg("-no-passive-gps");
        }
        if let Some(gnss_file_path) = &self.gnss_file_path {
            emulator.arg("-gnss-file-path").arg(gnss_file_path);
        }
        if let Some(gnss_grpc_port) = &self.gnss_grpc_port {
            emulator.arg("-gnss-grpc-port").arg(gnss_grpc_port);
        }
        if self.virtio_console {
            emulator.arg("-virtio-console");
        }
        if self.read_only {
            emulator.arg("-read-only");
        }
        if let Some(is_restart) = &self.is_restart {
            emulator.arg("-is-restart").arg(is_restart);
        }
        if let Some(report_console) = &self.report_console {
            emulator.arg("-report-console").arg(report_console);
        }
        if let Some(gps) = &self.gps {
            emulator.arg("-gps").arg(gps);
        }
        if let Some(shell_serial) = &self.shell_serial {
            emulator.arg("-shell-serial").arg(shell_serial);
        }
        if let Some(tcpdump) = &self.tcpdump {
            emulator.arg("-tcpdump").arg(tcpdump);
        }
        if let Some(bootchart) = &self.bootchart {
            emulator.arg("-bootchart").arg(bootchart);
        }
        if let Some(charmap) = &self.charmap {
            emulator.arg("-charmap").arg(charmap);
        }
        if let Some(studio_params) = &self.studio_params {
            emulator.arg("-studio-params").arg(studio_params);
        }
        if let Some(prop) = &self.prop {
            emulator.arg("-prop").arg(prop);
        }
        if let Some(shared_net_id) = &self.shared_net_id {
            emulator.arg("-shared-net-id").arg(shared_net_id);
        }
        if let Some(gpu) = &self.gpu {
            emulator.arg("-gpu").arg(gpu);
        }
        if self.use_host_vulkan {
            emulator.arg("-use-host-vulkan");
        }
        if let Some(camera_back) = &self.camera_back {
            emulator.arg("-camera-back").arg(camera_back.to_string());
        }
        if let Some(camera_front) = &self.camera_front {
            emulator.arg("-camera-front").arg(camera_front.to_string());
        }
        if self.webcam_list {
            emulator.arg("-webcam-list");
        }
        if let Some(virtualscene_poster) = &self.virtualscene_poster {
            emulator
                .arg("-virtualscene-poster")
                .arg(virtualscene_poster);
        }
        if let Some(screen) = &self.screen {
            emulator.arg("-screen").arg(screen.to_string());
        }
        if self.force_32bit {
            emulator.arg("-force-32bit");
        }
        if let Some(selinux) = &self.selinux {
            emulator.arg("-selinux").arg(selinux.to_string());
        }
        if let Some(unix_pipe) = &self.unix_pipe {
            emulator.arg("-unix-pipe").arg(unix_pipe);
        }
        if self.fixed_scale {
            emulator.arg("-fixed-scale");
        }
        if self.wait_for_debugger {
            emulator.arg("-wait-for-debugger");
        }
        if self.skip_adb_auth {
            emulator.arg("-skip-adb-auth");
        }
        if self.metrics_to_console {
            emulator.arg("-metrics-to-console");
        }
        if self.metrics_collection {
            emulator.arg("-metrics-collection");
        }
        if let Some(metrics_to_file) = &self.metrics_to_file {
            emulator.arg("-metrics-to-file").arg(metrics_to_file);
        }
        if self.detect_image_hang {
            emulator.arg("-detect-image-hang");
        }
        if let Some(feature) = &self.feature {
            emulator.arg("-feature").arg(feature);
        }
        if let Some(icc_profile) = &self.icc_profile {
            emulator.arg("-icc-profile").arg(icc_profile);
        }
        if let Some(sim_access_rules_file) = &self.sim_access_rules_file {
            emulator
                .arg("-sim-access-rules-file")
                .arg(sim_access_rules_file);
        }
        if let Some(phone_number) = &self.phone_number {
            emulator.arg("-phone-number").arg(phone_number);
        }
        if let Some(acpi_config) = &self.acpi_config {
            emulator.arg("-acpi-config").arg(acpi_config);
        }
        if self.fuchsia {
            emulator.arg("-fuchsia");
        }
        if let Some(window_size) = &self.window_size {
            emulator.arg("-window-size").arg(window_size);
        }
        if self.allow_host_audio {
            emulator.arg("-allow-host-audio");
        }
        if self.restart_when_stalled {
            emulator.arg("-restart-when-stalled");
        }
        if let Some(perf_stat) = &self.perf_stat {
            emulator.arg("-perf-stat").arg(perf_stat);
        }
        if self.share_vid {
            emulator.arg("-share-vid");
        }
        if let Some(grpc) = &self.grpc {
            emulator.arg("-grpc").arg(grpc);
        }
        if let Some(grpc_tls_key) = &self.grpc_tls_key {
            emulator.arg("-grpc-tls-key").arg(grpc_tls_key);
        }
        if let Some(grpc_tls_cer) = &self.grpc_tls_cer {
            emulator.arg("-grpc-tls-cer").arg(grpc_tls_cer);
        }
        if let Some(grpc_tls_ca) = &self.grpc_tls_ca {
            emulator.arg("-grpc-tls-ca").arg(grpc_tls_ca);
        }
        if self.grpc_use_token {
            emulator.arg("-grpc-use-token");
        }
        if let Some(idle_grpc_timeout) = &self.idle_grpc_timeout {
            emulator
                .arg("-idle-grpc-timeout")
                .arg(idle_grpc_timeout.to_string());
        }
        if let Some(waterfall) = &self.waterfall {
            emulator.arg("-waterfall").arg(waterfall);
        }
        if let Some(multidisplay) = &self.multidisplay {
            emulator.arg("-multidisplay").arg(multidisplay);
        }
        if let Some(google_maps_key) = &self.google_maps_key {
            emulator.arg("-google-maps-key").arg(google_maps_key);
        }
        if self.no_location_ui {
            emulator.arg("-no-location-ui");
        }
        if self.use_keycode_forwarding {
            emulator.arg("-use-keycode-forwarding");
        }
        if let Some(record_session) = &self.record_session {
            emulator.arg("-record-session").arg(record_session);
        }
        if self.legacy_fake_camera {
            emulator.arg("-legacy-fake-camera");
        }
        if self.no_camera_hq_edge {
            emulator.arg("-no-camera-hq-edge");
        }
        if self.no_direct_adb {
            emulator.arg("-no-direct-adb");
        }
        if let Some(check_snapshot_loadable) = &self.check_snapshot_loadable {
            emulator
                .arg("-check-snapshot-loadable")
                .arg(check_snapshot_loadable);
        }
        if self.no_hidpi_scaling {
            emulator.arg("-no-hidpi-scaling");
        }
        if self.no_mouse_reposition {
            emulator.arg("-no-mouse-reposition");
        }
        if self.guest_angle {
            emulator.arg("-guest-angle");
        }
        if self.qemu {
            emulator.arg("-qemu");
        }
        if self.verbose {
            emulator.arg("-verbose");
        }
        if let Some(debug) = &self.debug {
            emulator.arg("-debug").arg(debug.to_string());
        }
        if let Some(debug_no) = &self.debug_no {
            emulator.arg(format!("-debug-no-{}", debug_no));
        }
        if self.help {
            emulator.arg("-help");
        }
        if self.help_disk_images {
            emulator.arg("-help-disk-images");
        }
        if self.help_debug_tags {
            emulator.arg("-help-debug-tags");
        }
        if self.help_debug_tags {
            emulator.arg("-help-debug-tags");
        }
        if self.help_char_devices {
            emulator.arg("-help-char-devices");
        }
        if self.help_environment {
            emulator.arg("-help-environment");
        }
        if self.help_virtual_device {
            emulator.arg("-help-virtual-device");
        }
        if self.help_sdk_images {
            emulator.arg("-help-sdk-images");
        }
        if self.help_build_images {
            emulator.arg("-help-build-images");
        }
        if self.help_all {
            emulator.arg("-help-all");
        }
        emulator.output_err(true)?;
        Ok(())
    }
}
