use reqwest::Url;
use super::OrochiDownloader;
use super::DownloadError;
use crate::orochi::{Katana, Maintainer, Shuriken};

pub struct FtpDownloader {

}

impl OrochiDownloader for FtpDownloader {
    /// Package Downloader
    fn download_package(&self, package: Shuriken) -> Result<Vec<u8>, DownloadError> {
        unimplemented!("{}", package.hash.as_str())
    }

    /// Package URL Builder
    fn package_url(&self, package: Shuriken) -> Url {
        unimplemented!("{}", package.hash.as_str())
    }

    /// Download a Shuriken from the repository.
    fn download_shuriken(&self, name: &str) -> Result<Shuriken, DownloadError> {
        unimplemented!("{}", name)
    }

    /// Shuriken URL Builder.
    fn shuriken_url(&self, name: &str) -> Url {
        unimplemented!("{}", name)
    }

    /// Base URL
    fn base(&self) -> Url {
        unimplemented!()
    }

    /// Maintainers
    fn maintainers(&self) -> Vec<Maintainer> {
        unimplemented!()
    }

    /// Website
    fn website(&self) -> Url {
        unimplemented!()
    }

    /// New Downloader
    fn new(katana: Katana) -> Self {
        unimplemented!("{}", katana.uuid.as_str())
    } 
}