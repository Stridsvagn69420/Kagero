use async_trait::async_trait;
use reqwest::{Url, Client, tls};
use super::OrochiDownloader;
use super::DownloadError;
use super::ParseError;
use crate::orochi::{Katana, Shuriken};

pub struct HttpDownloader {
    http_client: Client,
    pub katana: Katana,
}

#[async_trait]
impl OrochiDownloader for HttpDownloader {
    /// Package Downloader
    fn download_package(&self, package: Shuriken) -> Result<Vec<u8>, DownloadError> {
        unimplemented!("{}", package.hash.as_str())
    }

    /// Package URL Builder
    fn package_url(&self, package: Shuriken) -> Result<Url, ParseError> {
        unimplemented!("{}", package.hash.as_str())
    }

    /// Download a Shuriken from the repository.
    async fn download_shuriken(&self, name: &str) -> Result<Vec<Shuriken>, DownloadError> {
        match self.shuriken_url(name) {
            Err(_) => Err(DownloadError::InvalidUrl),
            Ok(url) => {
                // Start the request
                match self.http_client.get(url).send().await {
                    Ok(res) => {
                        // Check the status code
                        if res.status().is_server_error() {
                            return Err(DownloadError::ServerError500);
                        }
                        if res.status().is_client_error() {
                            return Err(DownloadError::ClientError400);
                        }

                        // Parse Response
                        let jsontxt = res.text().await.unwrap();
                        match serde_json::from_str(&jsontxt) {
                            Ok(shurikens) => Ok(shurikens),
                            Err(_) => Err(DownloadError::NotParsable),
                        }
                    },
                    // Network error
                    Err(_) => Err(DownloadError::NetworkError)
                }
            }
        }
    }

    /// Shuriken URL Builder.
    fn shuriken_url(&self, name: &str) -> Result<Url, ParseError> {
        match self.base() {
            Ok(base) => Ok(
                // Add shuriken folder path
                base.join("shurikens").unwrap()
                .join(
                    // Add <name>.shuriken
                    &(name.to_string() + ".shuriken")
                ).unwrap()
            ),
            Err(e) => Err(e),
        }
    }

    /// Base URL
    fn base(&self) -> Result<Url, ParseError> {
        if !self.katana.root.starts_with("http://") || !self.katana.root.starts_with("https://") {
            return Err(ParseError::InvalidUrl);
        }
        match Url::parse(&self.katana.root) {
            Ok(url) => Ok(url),
            Err(_) => Err(ParseError::InvalidUrl)
        }
    }

    /// Website
    fn website(&self) -> Result<Url, String> {
        match Url::parse(&self.katana.website) {
            Ok(url) => Ok(url),
            Err(_) => Err(String::from(&self.katana.website)),
        }
    }

    /// New Downloader
    fn new(katana: Katana) -> Self {
        Self {
            katana,
            http_client: Client::builder()
            .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
            .min_tls_version(tls::Version::TLS_1_2)
            .build().unwrap()
        }
    } 
}