use std::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};

/// Metadata for a package.
/// 
/// A Shuriken defines a package, e.g. its version, supported arch and platform, package type, etc.
#[derive(Serialize, Deserialize)]
pub struct Shuriken {
    /// Package Hash
    /// 
    /// A SHA-256 hash of the package for verification,
    pub hash: String,

    /// Package ID
    /// 
    /// A unique identifier for the package. **Don't change it after publishing.**
    /// It can be everything but for Kagero, it has to be ASCII-encoded only.
    /// Something along `com.thisisme.mypackage` or `mypackage-version-author` is recommended.
    pub package: String,

    /// Package Version
    /// 
    /// Fixed-length array of 3 unsigned 8-bit integers representing the semantic version of the package.
    pub version: [u8; 3],

    /// Target Platform
    /// 
    /// The platform the package is made for. See [Platform] for more.
    pub platform: Platform,

    /// Target Architecture
    /// 
    /// The architecture the package is made for. See [Arch] for more.
    pub arch: Arch,

    /// Content Path
    /// 
    /// Path to the package's content.
    /// If it's just a filename, the absolute path will be ${ROOT}/${ARCH}/${PLATFORM}/${FILENAME},
    /// e.g. <https://orochi.mydomain.com/x86_64/linux/mypackage.tar.gz>
    pub path: String,

    /// Package Type
    /// 
    /// The type of the package content. See [PackType] for more.
    pub packtype: PackType
}

/// Platforms
/// 
/// Enum for supported platforms.
#[derive(Serialize, Deserialize)]
pub enum Platform {
    /// other
    /// 
    /// Value for packages that aren't from the common types like binaries or libraries.
    Other,

    /// any
    /// 
    /// Value for operating system-independent packages.
    Any,

    /// windows
    /// 
    /// Value for all (modern) versions of Windows.
    Windows,

    /// macos
    /// 
    /// Value for all (modern) versions of macOS.
    MacOS,

    /// linux
    /// 
    /// Value for Linux-based operating systems,
    /// e.g. Arch Linux, Void Linux, Ubuntu, Linux Mint, etc.
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

/// Architectures
/// 
/// An enum that contains supported CPU architectures. 
#[derive(Serialize, Deserialize)]
pub enum Arch {
    /// aarch64
    /// 
    /// 64-bit ARM architecture. Used on phones and SoCs like the Raspberry Pi 4.
    AARCH64,

    /// armhf
    /// 
    /// 32-bit ARM architecture. Used on SoCs like the Raspberry Pi 3 and older as well as some old mobile devices.
    ARM,

    /// i386/i586/i686
    /// 
    /// 32-bit x86 architecture. Used on old PCs and Laptops.
    X86,

    /// x86_64
    /// 
    /// 64-bit x86 architecture. Used on modern PCs and Laptops. Your system probably uses this.
    X86_64,

    /// runtime
    /// 
    /// Value for packages that are architecture-independent, e.g. Node.js projects.
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

/// Package types
/// 
/// The package will be installed depending on the package type,
/// e.g. `exe`s will just be copied and noted that they're installed, while the others get dedicated folders.
/// You can extend the enum to add custom types if Kagero doesn't plan on supporting them.
#[derive(Serialize, Deserialize)]
pub enum PackType {
    /// exe
    ///
    /// Standalone executable
    Exe,

    /// git
    ///
    /// Git repository
    Git,

    /// tar+gzip
    ///
    /// Gzipped Tar archive
    TarGz,

    /// tar+xz
    ///
    /// Xzipped Tar archive
    TarXz,

    /// zip
    /// 
    /// Zip archive
    Zip
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

/// Manifest of a repository
/// 
/// Manifest of a repository that contains every data needed.
#[derive(Serialize, Deserialize)]
pub struct Katana {
    /// UUIDv4 of the repository
    /// 
    /// Used to identify a repo. This allows you to host a mirror of another repository.
    pub uuid: String,

    /// Name of the repository
    /// 
    /// Pretty self-explainatory.
    pub name: String,

    /// Root of the repository
    /// 
    /// The base for all requests.
    /// This allows you to have a local Katana file that points to a repository online.
    /// So applicationxy.exe for windows x86_64 will be located at <https://orochi.mydomain.com/windows/x86_64/applicationxy.exe> if the root is set to <https://orochi.mydomain.com>.
    pub root: String,

    /// Website related to the repository
    /// 
    /// Doesn't have a functional need.
    /// It's only used to let the user know what the repo is linked to.
    pub website: String,

    /// Maintainers
    /// 
    /// List of the [maintainers](Maintainer) of the repository.
    pub maintainers: Vec<Maintainer>,

    /// Shurikens/Packages
    ///
    /// List of the [Shurikens](Shuriken) that the repository contains.
    pub shurikens: Vec<Shuriken>
}

/// Maintainer of a repository
/// 
/// Maintainer of a repository as listed in a Katana file.
/// It consists of the name, email and website of a maintainer, although only the name is required.
#[derive(Serialize, Deserialize)]
pub struct Maintainer {
    pub name: String,
    pub email: String,
    pub website: String
}

impl Display for Maintainer {
    fn fmt(&self, f: &mut Formatter) -> Result {
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