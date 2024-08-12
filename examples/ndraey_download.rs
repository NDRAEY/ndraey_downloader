use std::{env, process::exit};

use ndraey_downloader::progress;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut destination: Option<&String> = None;

    if args.len() == 1 {
        eprintln!("Usage: ndraey_download file_url [destination]");
        exit(1);
    }

    if args.len() > 2 {
        destination = Some(&args[2]);
    }

    let file_url = &args[1];


    progress(
	    file_url.to_string(),
        destination.unwrap_or(&file_url.rsplit_once('/').unwrap().1.to_string()).to_string()
        ).await;
}
