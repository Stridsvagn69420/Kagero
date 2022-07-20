use crate::orochi::{Katana, Shuriken, PackType};
use url::ParseError;
use reqwest::{tls, StatusCode, Client, Url, self};
use thiserror::Error;
use serde_json;

/// HTTP-Downloader Errors
///
/// Errors that can occur when working with the [HttpDownloader]
#[derive(Error, Debug)]
pub enum HttpError {
	/// Request Error
	/// 
	/// Translates to [reqwest::Error]].
    /// It means that the reponse is a 500 error or the request itself failed.
	#[error("an error occured during the request")]
	RequestError(#[from] reqwest::Error),

	/// Parsing Error
	///
	/// Translates to [serde_json::Error]].
    /// In this context, it means that the JSON is invalid or the reponse isn't a JSON at all.
	#[error("json could not be parsed")]
	ParsingError(#[from] serde_json::Error),

    /// Not Found Error
    /// 
    /// Represents a 404 error.
    #[error("the requested resource was not found")]
    NotFound,

    /// Unauthorized Error
    /// 
    /// Represents a 401 error.
    #[error("you are not authorized to access this resource")]
    Unauthorized
}

/// HTTP-Credentials
/// 
/// Credentials for Basic Authentication.
pub struct HttpCredentials {
    username: String,
    password: Option<String>,
}

/// HTTP-Downloader
/// 
/// Downloader for HTTP-Repositories.
pub struct HttpDownloader {
    auth: HttpCredentials,
    http_client: Client,
    pub katana: Katana,
    root: Url
}

impl HttpDownloader {
    /// New HTTP-Downloader
    /// 
    /// Creates a new [HttpDownloader] instance.
    /// An error is thrown if the repository root URL in the [Katana Manifest](Katana) is not valid.
    pub fn new(katana: Katana, auth: HttpCredentials) -> Result<HttpDownloader, ParseError> {
        // Only the Repository Root has to be validated. The other URLs are for rich representation.
        let url = Url::parse(&katana.root)?;
        Ok(HttpDownloader {
            katana,
            auth,
            root: url,
            http_client: Client::builder()
            .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
            .min_tls_version(tls::Version::TLS_1_2)
            .build().unwrap(),
        })
    }

    /// Download Package
    /// 
    /// Downloads a package from the repository with set [Shuriken].
    /// If it's a Git Package, it will just return the Shell Command in a Byte Array to clone the repository.
    /// Returns an [HttpError] if the request fails.
    pub async fn download_package(&self, shuriken: Shuriken) -> Result<(Vec<u8>, PackType), HttpError> {
        // TODO: Implement
        // This function should download the package and return it as a Vec<u8> along with its package type so that you can do different things depending on it.
        // It gets the URL for the Shuriken for the PackTypes and checks the scheme. If it's http(s), it will download them via HTTP.
        // But if it starts with Git, it will check if it's standalone or with a tag/branch, parse everything and return the Git-Url with the tag/branch seperated with a semicolon after as a Byte Array.
        // Actually writing to disk will be handled in the filesystem utils.
        let data: Vec<u8>;

        if shuriken.path.starts_with("http+") {
            // Download from set HTTP URL
            let url = match Url::parse(&shuriken.path[5..]) {
                Ok(url) => url,
                Err(_) => return Err(HttpError::NotFound)
            };
            data = self.download_bytes(url).await?;
        } else if shuriken.path.starts_with("git+") {
            // "Download" the Git-URL and return it as a Byte Array
            let url = String::from(&shuriken.path[4..]);
            data = url.into_bytes();

        } else if shuriken.path.starts_with("git:") {
            // "Download" the Git-URL with a Branch or Tag and return it as a Byte Array
            let comps: Vec<&str>  = shuriken.path[4..].split("+").collect();
            let fmturl = match comps.get(1) {
                Some(url) => format!("{};{}", url, comps[0]),
                None => return Err(HttpError::NotFound)
            };
            data = fmturl.into_bytes();
        } else {
            // Download from the repository
            let url = self.package_url(&shuriken);
            data = self.download_bytes(url).await?;
        }
        Ok((data, shuriken.packtype))

    }

    /// Download Shuriken
    /// 
    /// Downloads the [Shuriken] from the repository.
    /// Returns an [HttpError] if the request fails.
    pub async fn download_shuriken(&self, name: &str) -> Result<Shuriken, HttpError> {
        let json = self.download_string(self.shuriken_url(name)).await?;
        Ok(serde_json::from_str(&json)?)
    }

    /// Shuriken URL-Builder
    /// 
    /// Builds the URL to download a [Shuriken] from HTTP.
    /// Make sure that the [Katana's root URL](Katana) is valid.
    /// Note that Shuriken files are [Vectors](Vec) of [Shurikens](Shuriken).
    pub fn shuriken_url(&self, name: &str) -> Url {
        self.root
        .join("shurikens/").unwrap()
        .join(
            &(name.to_string() + ".shuriken")
        ).unwrap()
    }

    /// Package URL-Builder
    /// 
    /// Builds the URL to download a package from HTTP.
    /// Note that this is for the case that the `path` parameter is only a file and not a Git or HTTP URL.
    pub fn package_url(&self, shuriken: &Shuriken) -> Url {
        match Url::parse(shuriken.path.as_str()) {
            // Create URL for Orochi URL if the URL is just a filename
            Err(_) => self.root
            .join(shuriken.arch.as_ref()).unwrap()
            .join(shuriken.platform.as_ref()).unwrap()
            .join(shuriken.path.as_str()).unwrap(),
            // Return the URL if it is a valid URL
            Ok(url) => url
        }        
    }

    /// Download String
    /// 
    /// Downloads a resource via HTTP as a string.
    pub async fn download_string(&self, url: Url) -> Result<String, HttpError> {
        let response = self.http_client.get(url)
        .basic_auth(&self.auth.username, self.auth.password.as_ref())
        .header("Accept", "application/json, text/json, text/plain")
        .send().await?;
        match response.status() {
            StatusCode::OK => Ok(response.text().await?),
            StatusCode::NOT_FOUND => Err(HttpError::NotFound),
            StatusCode::UNAUTHORIZED => Err(HttpError::Unauthorized),
            // This is completely useless but has to be here.
            _ => Err(HttpError::RequestError(
                response.error_for_status()
                .err().unwrap()
            ))
        }
    }

    /// Download ByteArray
    /// 
    /// Downloads a resource via HTTP as a byte array.
    pub async fn download_bytes(&self, url: Url) -> Result<Vec<u8>, HttpError> {
        let response = self.http_client.get(url)
        .basic_auth(&self.auth.username, self.auth.password.as_ref())
        .header("Accept", "application/octet-stream")
        .send().await?;
        match response.status() {
            StatusCode::OK => Ok(response.bytes().await?.to_vec()),
            StatusCode::NOT_FOUND => Err(HttpError::NotFound),
            StatusCode::UNAUTHORIZED => Err(HttpError::Unauthorized),
            // This is completely useless but has to be here.
            _ => Err(HttpError::RequestError(
                response.error_for_status()
                .err().unwrap()
            ))
        }
    }
}
