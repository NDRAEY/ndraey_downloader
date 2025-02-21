use crate::{types::Callback, Downloader};

#[derive(Debug)]
pub enum BuilderError {
    ClientError(reqwest::Error)
}

#[derive(Debug)]
pub struct DownloaderBuilder {
    callback: Option<Callback>
}

impl DownloaderBuilder {
    pub fn new() -> Self {
        DownloaderBuilder {
            callback: None
        }
    }

    pub fn with_callback(self, callback: Callback) -> Self {
        let mut downloader = self;

        downloader.callback = Some(callback);

        downloader
    }

    pub fn build(self) -> Result<Downloader, BuilderError> {
        let client = match reqwest::Client::builder()
            .user_agent("Mozilla/5.0")
            .build() {
                Ok(client) => client,
                Err(err) => {
                    return Err(BuilderError::ClientError(err));
                }
            };

        Ok(Downloader {
            client,
            report_callback: self.callback
        })
    }
}
