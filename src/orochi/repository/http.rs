use crate::orochi::{Katana, Shuriken};
use url::ParseError;
use reqwest::{tls, Client, Url};

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
}

impl HttpDownloader {
    /// New HTTP-Downloader
    /// 
    /// Creates a new [HttpDownloader] instance.
    /// An error is thrown if the repository root URL in the [Katana Manifest](Katana) is not valid.
    pub fn new(katana: Katana, auth: HttpCredentials) -> Result<HttpDownloader, ParseError> {
        // Only the Repository Root has to be validated. The other URLs are for rich representation.
        Url::parse(&katana.root)?;
        Ok(HttpDownloader {
            katana,
            auth,
            http_client: Client::builder()
            .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
            .min_tls_version(tls::Version::TLS_1_2)
            .build().unwrap(),
        })
    }

    /// Shuriken URL-Builder
    /// 
    /// Builds the URL to download a [Shuriken] from HTTP.
    /// Make sure that the [Katana's root URL](Katana) is valid.
    /// Note that Shuriken files are [Vectors](Vec) of [Shurikens](Shuriken).
    pub fn shuriken_url(&self, name: &str) -> Url {
        Url::parse(&self.katana.root).unwrap()
        .join("shurikens/").unwrap()
        .join(
            &(name.to_string() + ".shuriken")
        ).unwrap()
    }

    /// Package URL-Builder
    /// 
    /// Builds the URL to download a package from HTTP.
    /// Note that this is for the case that the `path` parameter is only a file and not a Git or HTTP URL.
    pub fn package_url(&self, shuriken: Shuriken) -> Url {
        Url::parse(&self.katana.root).unwrap()
        .join(shuriken.arch.as_ref()).unwrap()
        .join(shuriken.platform.as_ref()).unwrap()
    }

    /// Download String
    /// 
    /// Downloads a resource via HTTP as a string.
    pub async fn download_string(&self, url: Url) -> Result<String, reqwest::Error> {
        let response = self.http_client.get(url)
        .basic_auth(&self.auth.username, self.auth.password.as_ref())
        .send().await?;
        Ok(response.text().await?)
    }

    /// Download ByteArray
    /// 
    /// Downloads a resource via HTTP as a byte array.
    pub async fn download_bytes(&self, url: Url) -> Result<Vec<u8>, reqwest::Error> {
        let response = self.http_client.get(url)
        .basic_auth(&self.auth.username, self.auth.password.as_ref())
        .send().await?;
        Ok(response.bytes().await?.to_vec())
    }
}
