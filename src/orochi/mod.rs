#[cfg(feature = "orochi-downloader")]
/// Orochi Request Utilities
/// 
/// This module features utilities for interacting with an Orochi repository.
mod downloader;
#[cfg(feature = "orochi-downloader")]
pub use self::downloader::*;

/// Orochi Metadata
mod meta;
pub use self::meta::Maintainer;
pub use self::meta::Katana;
pub use self::meta::Shuriken;
pub use self::meta::Arch;
pub use self::meta::Platform;
pub use self::meta::PackType;