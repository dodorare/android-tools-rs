#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallLocation {
    /// Lets system decide the best location
    Auto,
    /// Installs on internal device storage
    Internal,
    /// Installs on external media
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenCompatibilityMode {
    On,
    Off,
}

impl std::fmt::Display for InstallLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Auto => write!(f, "auto"),
            Self::Internal => write!(f, "internal"),
            Self::External => write!(f, "external"),
        }
    }
}

impl std::fmt::Display for ScreenCompatibilityMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
        }
    }
}