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