use thiserror::Error;
use reqwest;
use serde_json;

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

/// HTTP-Downloader Errors
///
/// Errors that can occur when working with the [HttpDownloader]
#[derive(Error, Debug)]
pub enum HttpError {
	/// Request Error
	/// 
	/// Translates to [reqwest::Error]]
	#[error("an error occured during the request")]
	RequestError(#[from] reqwest::Error),

	/// Parsing Error
	///
	/// Translates to [serde_json::Error]]
	#[error("json could not be parsed")]
	ParsingError(#[from] serde_json::Error)
}

/// SSH Repository implementation
/// 
/// This module implements utils for SSH repositores.
mod ssh;
pub use self::ssh::SshDownloader;
