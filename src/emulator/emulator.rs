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
    engine: Option<PathBuf>, // TODO
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

    pub fn list_avds(&mut self, list_avds: bool) -> &mut Self {
        self.list_avds = list_avds;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selinux {
    Disabled,
    Permissive,
}