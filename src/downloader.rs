use std::{fs::File, io::{stdout, Write}, time::{SystemTime, UNIX_EPOCH}};

use futures_util::StreamExt;

use crate::types::Callback;

#[derive(Debug)]
pub enum DownloaderError {
    ClientError(reqwest::Error),
    FilesystemError(std::io::Error)
}

#[derive(Debug)]
pub struct Downloader {
    pub(crate) client: reqwest::Client,
    pub(crate) report_callback: Option<Callback>
}

impl Downloader {
    pub async fn download(&mut self, url: &str, destination: &str) -> Result<(), DownloaderError> {
        let res = match self.client.get(url).send().await {
            Ok(response) => response,
            Err(err) => {
                return Err(DownloaderError::ClientError(err));
            }
        };
    
        let total_size = res.content_length().unwrap_or(0);
    
        let mut file = match File::create(destination) {
            Ok(file) => file,
            Err(err) => {
                return Err(DownloaderError::FilesystemError(err));
            }
        };
    
        {
            let mut downloaded: u64 = 0;
            let mut downloaded_in_sec: usize = 0;
            let mut stream = res.bytes_stream();

            let mut sys_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let mut speed: usize = 0;

            while let Some(item) = stream.next().await {
                let chunk = match item {
                    Ok(chunk) => chunk,
                    Err(err) => {
                        return Err(DownloaderError::ClientError(err));
                    }
                };

                if let Err(err) = file.write_all(&chunk) {
                    return Err(DownloaderError::FilesystemError(err));
                }

                downloaded += chunk.len() as u64;
                downloaded_in_sec += chunk.len() as usize;

                // let percent = (downloaded as f64 / total_size as f64) * 100.0;

                let new_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                if new_time > sys_time {
                    speed = downloaded_in_sec;
                    downloaded_in_sec = 0;
                    sys_time = new_time;
                }

                if self.report_callback.is_some() {
                    let cb = self.report_callback.unwrap();

                    cb(downloaded as usize, total_size as usize, speed);

                    stdout().flush().unwrap();
                }
            }

            if self.report_callback.is_some() {
                println!();
            }
        }

        Ok(())
    }
}