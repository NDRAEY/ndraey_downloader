use std::{env, process::exit};

use ndraey_downloader::{Downloader, DownloaderBuilder};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let destination: String;

    if args.len() == 1 {
        eprintln!("Usage: ndraey_download file_url [destination]");
        exit(1);
    }

    let file_url = &args[1];

    if args.len() > 2 {
        destination = args[2].clone();
    } else {
        let tmp = file_url.rsplit_once('/').unwrap().1;
        destination = tmp.to_string();
    }

    let mut dwn = DownloaderBuilder::new().with_callback(|downloaded, total, speed| {
        println!("-> {} {} {}", downloaded, total, speed);
    }).build().expect("Failed to build a Downloader");

    dwn.download(file_url, &destination).await.unwrap();
}
