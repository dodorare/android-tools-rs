#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SELinux {
    Disabled,
    Permissive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Engine {
    Auto,
    Classic,
    Qemu2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Netspeed {
    Gsm,
    Hscsd,
    Gprs,
    Wdge,
    Umts,
    Hsdpa,
    Lte,
    Evdo,
    Full,
    Num,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    Emulated,
    None,
    Webcam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenMode {
    Touch,
    MultiTouch,
    NoTouch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccelMode {
    Auto,
    Off,
    On,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugTags {
    Init,
    Console,
    Modem,
    Radio,
    Keys,
    Events,
    Slirp,
    Timezone,
    Socket,
    Proxy,
    Audio,
    Audioin,
    Audioout,
    Surface,
    Qemud,
    Gps,
    NandLimits,
    HwControl,
    AvdConfig,
    Sensors,
    Memcheck,
    Camera,
    Adevice,
    SensorsPort,
    Mtport,
    Mtscreen,
    Gles,
    Gles1emu,
    Adbserver,
    Adbclient,
    Adb,
    Asconnector,
    Asyncsocket,
    Sdkctlsocket,
    Updater,
    Metrics,
    Rotation,
    Goldfishsync,
    Syncthreads,
    Memory,
    Car,
    Record,
    Snapshot,
    Virtualscene,
    Automation,
    Offworld,
    Videoinjection,
    Foldable,
    Curl,
    CarRotary,
    Wifi,
    Tvremote,
    Time,
    Ini,
    All,
}

impl std::fmt::Display for SELinux {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Disabled => write!(f, "disabled"),
            Self::Permissive => write!(f, "permissive"),
        }
    }
}

impl std::fmt::Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Auto => write!(f, "auto"),
            Self::Classic => write!(f, "classic"),
            Self::Qemu2 => write!(f, "qemu2"),
        }
    }
}

impl std::fmt::Display for Netspeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Gsm => write!(f, "gsm"),
            Self::Hscsd => write!(f, "hscsd"),
            Self::Gprs => write!(f, "gprs"),
            Self::Wdge => write!(f, "wdge"),
            Self::Umts => write!(f, "umts"),
            Self::Hsdpa => write!(f, "hsdpa"),
            Self::Lte => write!(f, "lte"),
            Self::Evdo => write!(f, "evdo"),
            Self::Full => write!(f, "full"),
            Self::Num => write!(f, "num"),
            Self::Up => write!(f, "up"),
            Self::Down => write!(f, "down"),
        }
    }
}

impl std::fmt::Display for CameraMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Emulated => write!(f, "emulated"),
            Self::None => write!(f, "none"),
            Self::Webcam => write!(f, "webcam"),
        }
    }
}

impl std::fmt::Display for ScreenMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Touch => write!(f, "touch"),
            Self::MultiTouch => write!(f, "multi-touch"),
            Self::NoTouch => write!(f, "no-touch"),
        }
    }
}

impl std::fmt::Display for AccelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Auto => write!(f, "auto"),
            Self::Off => write!(f, "off"),
            Self::On => write!(f, "on"),
        }
    }
}

impl std::fmt::Display for DebugTags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Init => write!(f, "init"),
            Self::Console => write!(f, "console"),
            Self::Modem => write!(f, "modem"),
            Self::Radio => write!(f, "radio"),
            Self::Keys => write!(f, "keys"),
            Self::Events => write!(f, "events"),
            Self::Slirp => write!(f, "slirp"),
            Self::Timezone => write!(f, "timezone"),
            Self::Socket => write!(f, "socket"),
            Self::Proxy => write!(f, "proxy"),
            Self::Audio => write!(f, "audio"),
            Self::Audioin => write!(f, "audioin"),
            Self::Audioout => write!(f, "audioout"),
            Self::Surface => write!(f, "surface"),
            Self::Qemud => write!(f, "qemud"),
            Self::Gps => write!(f, "gps"),
            Self::NandLimits => write!(f, "nand_limits"),
            Self::HwControl => write!(f, "hw_control"),
            Self::AvdConfig => write!(f, "avd_config"),
            Self::Sensors => write!(f, "sensors"),
            Self::Memcheck => write!(f, "memcheck"),
            Self::Camera => write!(f, "camera"),
            Self::Adevice => write!(f, "adevice"),
            Self::SensorsPort => write!(f, "sensors_port"),
            Self::Mtport => write!(f, "mtport"),
            Self::Mtscreen => write!(f, "mtscreen"),
            Self::Gles => write!(f, "gles"),
            Self::Gles1emu => write!(f, "gles1emu"),
            Self::Adbserver => write!(f, "adbserver"),
            Self::Adbclient => write!(f, "adbclient"),
            Self::Adb => write!(f, "adb"),
            Self::Asconnector => write!(f, "asconnector"),
            Self::Asyncsocket => write!(f, "asyncsocket"),
            Self::Sdkctlsocket => write!(f, "sdkctlsocket"),
            Self::Updater => write!(f, "updater"),
            Self::Metrics => write!(f, "metrics"),
            Self::Rotation => write!(f, "rotation"),
            Self::Goldfishsync => write!(f, "goldfishsync"),
            Self::Syncthreads => write!(f, "syncthreads"),
            Self::Memory => write!(f, "memory"),
            Self::Car => write!(f, "car"),
            Self::Record => write!(f, "record"),
            Self::Snapshot => write!(f, "snapshot"),
            Self::Virtualscene => write!(f, "virtualscene"),
            Self::Automation => write!(f, "automation"),
            Self::Offworld => write!(f, "offworld"),
            Self::Videoinjection => write!(f, "videoinjection"),
            Self::Foldable => write!(f, "foldable"),
            Self::Curl => write!(f, "curl"),
            Self::CarRotary => write!(f, "car_rotary"),
            Self::Wifi => write!(f, "wifi"),
            Self::Tvremote => write!(f, "tvremote"),
            Self::Time => write!(f, "time"),
            Self::Ini => write!(f, "ini"),
            Self::All => write!(f, "all"),
        }
    }
}
