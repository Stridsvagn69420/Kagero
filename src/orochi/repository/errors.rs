/// Download Errors
pub enum DownloadError {
    /// Resource not found.
    NotFound404,
    /// Access to Resource denied.
    AccessDenied403,
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
            DownloadError::NotFound404 => "Resource not found".to_string(),
            DownloadError::AccessDenied403 => "Access to Resource denied".to_string(),
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
            DownloadError::NotFound404 => "Resource not found",
            DownloadError::AccessDenied403 => "Access to Resource denied",
            DownloadError::ServerError500 => "Server encounters internal errors",
            DownloadError::NetworkError => "Internet or network not available",
            DownloadError::InvalidUrl => "URL is invalid",
            DownloadError::NotCompatible => "Package not compatible",
            DownloadError::NotParsable => "Shuriken not parsable",
        }
    }
}