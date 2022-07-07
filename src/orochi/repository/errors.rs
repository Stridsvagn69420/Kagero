/// Download Errors
/// 
/// Download errors that happen in download processes.
pub enum DownloadError {
    /// Client error.
    /// 
    /// This probably means the resource isn't available.
    ClientError400,
    /// Server encounters internal errors.
    ServerError500,
    /// Internet or network not available.
    NetworkError,
    /// URL is invalid.
    InvalidUrl,
    /// Package not compatible. (Package exclusive)
    NotCompatible,
    /// Shuriken not parsable. (Shuriken exclusive)
    NotParsable,
}

impl DownloadError {
    /// Returns a [String] representation of the error.
    pub fn to_string(&self) -> String {
        match *self {
            DownloadError::ClientError400 => "Client error".to_string(),
            DownloadError::ServerError500 => "Server encounters internal errors".to_string(),
            DownloadError::NetworkError => "Internet or network not available".to_string(),
            DownloadError::InvalidUrl => "URL is invalid".to_string(),
            DownloadError::NotCompatible => "Package not compatible".to_string(),
            DownloadError::NotParsable => "Shuriken not parsable".to_string(),
        }
    }
    /// Returns a [str] representation of the error.
    pub fn to_str(&self) -> &str {
        match *self {
            DownloadError::ClientError400 => "Client error",
            DownloadError::ServerError500 => "Server encounters internal errors",
            DownloadError::NetworkError => "Internet or network not available",
            DownloadError::InvalidUrl => "URL is invalid",
            DownloadError::NotCompatible => "Package not compatible",
            DownloadError::NotParsable => "Shuriken not parsable",
        }
    }
}

/// Parse Errors
/// 
/// Errors that are exclusively related to parsing.
pub enum ParseError {
    /// JSON is invalid
    InvalidJson,
    /// URL is invalid
    InvalidUrl,
    /// Shuriken not found
    ShurikenError,
    /// Package not found
    PackageError,
    /// Wrong protocol
    WrongProtocol
}

impl ParseError {
    /// Returns a [String] representation of the error.
    pub fn to_string(&self) -> String {
        match *self {
            ParseError::InvalidJson => "Invalid JSON".to_string(),
            ParseError::InvalidUrl => "Invalid URL".to_string(),
            ParseError::ShurikenError => "Shuriken not found".to_string(),
            ParseError::PackageError => "Package not found".to_string(),
            ParseError::WrongProtocol => "Mismatched protocols".to_string()
        }
    }
    /// Returns a [str] representation of the error.
    pub fn to_str(&self) -> &str {
        match *self {
            ParseError::InvalidJson => "Invalid JSON",
            ParseError::InvalidUrl => "Invalid URL",
            ParseError::ShurikenError => "Shuriken not found",
            ParseError::PackageError => "Package not found",
            ParseError::WrongProtocol => "Mismatched protocols"
        }
    }
}