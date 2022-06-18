use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[cfg(feature = "orochi-repo")]
pub mod repo;

#[derive(Serialize, Deserialize)]
pub struct Shuriken {
    pub hash: String,
    pub package: String,
    pub version: [u8; 3],
    pub platform: Platform,
    pub arch: Arch,
    pub path: String,
    pub packtype: PackType
}

#[derive(Serialize, Deserialize)]
pub enum Platform {
    Other,
    Any,
    Windows,
    MacOS,
    Linux
}

impl AsRef<str> for Platform {
    fn as_ref(&self) -> &str {
        match self {
            Platform::Other => "other",
            Platform::Any => "any",
            Platform::Windows => "windows",
            Platform::MacOS => "macos",
            Platform::Linux => "linux"
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Arch {
    AARCH64,
    ARM,
    X86,
    X86_64,
    Runtime
}

impl AsRef<str> for Arch {
    fn as_ref(&self) -> &str {
        match self {
            Arch::AARCH64 => "aarch64",
            Arch::ARM => "arm",
            Arch::X86 => "x86",
            Arch::X86_64 => "x86_64",
            Arch::Runtime => "runtime"
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum PackType {
    Zip,
    TarGz,
    TarXz,
    Exe,
    Git
}

impl AsRef<str> for PackType {
    fn as_ref(&self) -> &str {
        match self {
            PackType::Zip => "zip",
            PackType::TarGz => "tar+gzip",
            PackType::TarXz => "tar+xz",
            PackType::Exe => "exe",
            PackType::Git => "git",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Katana {
    pub uuid: String,
    pub name: String,
    pub root: String,
    pub website: String,
    pub maintainers: Vec<Maintainer>,
    pub shurikens: Vec<Shuriken>
}

#[derive(Serialize, Deserialize)]
pub struct Maintainer {
    pub name: String,
    pub email: String,
    pub website: String
}

impl Display for Maintainer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if (self.email.is_empty()) && (self.website.is_empty()) {
            write!(f, "{} <{}> ({})", self.name, self.email, self.website)
        } else if self.email.is_empty() {
            write!(f, "{} <{}>", self.name, self.email)
        } else if self.website.is_empty() {
            write!(f, "{} ({})", self.name, self.website)
        } else {
            write!(f, "{}", self.name)
        }
    }
}