use futures_util::StreamExt;
use std::cmp::min;
/// A simple module for downloading large files
/// by NDRAEY (c) 2022
// use reqwest;
use std::fs::File;
use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

/// Download file from {url} divided by chunks with progress bar
pub async fn progress(url: String, path: String) -> bool {
    let mut _res = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .unwrap()
        .get(url.clone())
        .send()
        .await
        .or(Err("Failed to make GET request!"));

    let res: reqwest::Response = match _res {
        Err(err) => {
            println!(
                "[ndraey_downloader] Failed to send request! (Error: {})",
                err
            );
            return false;
        }
        Ok(d) => d,
    };

    let total_size = res.content_length();

    let mut file = File::create(path.clone())
        .or(Err(format!("Failed to create file '{}'", path.clone())))
        .unwrap();
    let mut downloaded: u64 = 0;
    let mut downloaded_in_sec: usize = 0;
    let mut stream = res.bytes_stream();

    let splitted = path.split('/').collect::<Vec<&str>>();
    let name = splitted[splitted.len() - 1];

    let mut sys_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_secs();
    let mut speed: usize = 0;

    while let Some(item) = stream.next().await {
        let chunk = item
            .or(Err(format!(
                "Error while downloading file: {}",
                url.clone()
            )))
            .unwrap();
        let _result = file.write_all(&chunk).or(Err(format!(
            "Error while writing to file: {}",
            path.clone()
        )));
        
        let new = min(downloaded + (chunk.len() as u64), total_size.unwrap_or(1));
        downloaded = new;
        downloaded_in_sec += chunk.len() as usize;

        let ntsize: f64 = total_size.unwrap_or(1) as f64;
        let percent = (new as f64 / ntsize) * 100_f64;
        let chars = ((new as f64 / ntsize) * 20_f64) as usize;

        let newtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time error")
            .as_secs();
            
        if newtime > sys_time + 1 {
            speed = downloaded_in_sec;
            downloaded_in_sec = 0;
            sys_time = newtime;
        }

        print!(
            "[{}] [{:.1}%] [{} kB/s] [{:.0} / {:.0} kB] [{:20}]\x1b[K\r",
            name,
            percent,
            speed/1024,
            new as f64 / 1024_f64,
            ntsize / 1024_f64,
            "=".to_string().repeat(chars)
        );
        io::stdout().flush().unwrap();
    }
    println!();
    true
}
