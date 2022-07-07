use async_trait::async_trait;
use reqwest::Url;
use super::OrochiDownloader;
use super::DownloadError;
use super::ParseError;
use crate::orochi::{Katana, Shuriken};

pub struct FtpDownloader {

}

#[async_trait]
impl OrochiDownloader for FtpDownloader {
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
        unimplemented!("{}", name)
    }

    /// Shuriken URL Builder.
    fn shuriken_url(&self, name: &str) -> Result<Url, ParseError> {
        unimplemented!("{}", name)
    }

    /// Base URL
    fn base(&self) -> Result<Url, ParseError> {
        unimplemented!()
    }

    /// Website
    fn website(&self) -> Result<Url, String> {
        unimplemented!()
    }

    /// New Downloader
    fn new(katana: Katana) -> Self {
        unimplemented!("{}", katana.uuid.as_str())
    } 
}