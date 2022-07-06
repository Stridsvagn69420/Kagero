use super::{Katana, Maintainer, Shuriken};
use reqwest::Url;

/// FTP Repository implementation
/// 
/// This module implements utils for FTP repositores.
mod ftp;
pub use self::ftp::FtpDownloader;

/// HTTP Repository implementation
/// 
/// This module implements utils for HTTP repositores.
mod http;
pub use self::http::HttpDownloader;

/// SSH Repository implementation
/// 
/// This module implements utils for SSH repositores.
mod ssh;
pub use self::ssh::SshDownloader;

/// Collection of Errors
/// 
/// A module featuring enums that represent errors.
mod errors;
pub use self::errors::DownloadError;

/// Downloader
/// 
/// Trait for downloaders.
pub trait OrochiDownloader {
    /// Download a package from the repository.
    /// 
    /// Attempts to download a package from the repository as a [byte array](Vec<u8>).
    fn download_package(&self, package: Shuriken) -> Result<Vec<u8>, DownloadError>;

    /// Package URL Builder
    ///
    /// Builds a URL for a package for the current system.
    fn package_url(&self, package: Shuriken) -> Url;

    /// Download a Shuriken from the repository.
    /// 
    /// Attempts to download a package from the repository as a [String].
    fn download_shuriken(&self, name: &str) -> Result<Shuriken, DownloadError>;

    /// Shuriken URL Builder
    /// 
    /// Builds a Shuriken URL.
    fn shuriken_url(&self, name: &str) -> Url;

    /// Base URL
    /// 
    /// Returns the base URL for this repository as a [Url].
    fn base(&self) -> Url;

    /// Maintainers
    /// 
    /// Returns a list of maintainers for this repository as a [Vec<Maintainer>].
    fn maintainers(&self) -> Vec<Maintainer>;

    /// Website
    /// 
    /// Returns the website for this repository as a [Url].
    fn website(&self) -> Url;

    /// New Downloader
    /// 
    /// Creates a new downloader for set Katana.
    fn new(katana: Katana) -> Self;
}