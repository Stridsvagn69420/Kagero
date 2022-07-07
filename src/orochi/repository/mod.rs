use super::{Katana, Shuriken};
use async_trait::async_trait;
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
pub use self::errors::ParseError;

/// Downloader
/// 
/// Trait for downloaders.
#[async_trait]
pub trait OrochiDownloader {
    /// Download a package from the repository.
    /// 
    /// Attempts to download a package from the repository as a [byte array](Vec<u8>).
    fn download_package(&self, package: Shuriken) -> Result<Vec<u8>, DownloadError>;

    /// Package URL Builder
    ///
    /// Builds a URL for a package for the current system.
    fn package_url(&self, package: Shuriken) -> Result<Url, ParseError>;

    /// Download a Shuriken from the repository.
    /// 
    /// Attempts to download a package from the repository as a [String].
    async fn download_shuriken(&self, name: &str) -> Result<Vec<Shuriken>, DownloadError>;

    /// Shuriken URL Builder
    /// 
    /// Builds a Shuriken URL.
    fn shuriken_url(&self, name: &str) -> Result<Url, ParseError>;

    /// Base URL
    /// 
    /// Returns the base URL for this repository as a [Url].
    fn base(&self) -> Result<Url, ParseError>;

    /// Website
    /// 
    /// Returns the website for this repository as a [Url], optionally as a [String] if parsing fails.
    fn website(&self) -> Result<Url, String>;

    /// New Downloader
    /// 
    /// Creates a new downloader for set Katana.
    fn new(katana: Katana) -> Self;
}