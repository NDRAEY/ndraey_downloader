use colored::Colorize;
use std::io::{stdout, Write};

pub mod builder;
pub mod downloader;
pub mod types;

pub use builder::DownloaderBuilder;
pub use downloader::Downloader;

fn format_bytes(bytes: usize) -> String {
    let bytes = bytes as f64;

    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    if bytes < KB {
        format!("{:.2} B", bytes)
    } else if bytes < MB {
        format!("{:.2} kB", bytes / KB)
    } else if bytes < GB {
        format!("{:.2} MB", bytes / MB)
    } else {
        format!("{:.2} GB", bytes / GB)
    }
}

fn report_callback(downloaded: usize, total_size: usize, speed: usize) {
    let percent = (downloaded as f64 / total_size as f64) * 100.0;

    let total_size_str = if total_size == 0 {
        "--.--".to_string()
    } else {
        format_bytes(total_size)
    };

    let percent_str = if total_size == 0 {
        "--.--".to_string()
    } else {
        format!("{:.2}", percent).to_string()
    };

    print!(
        "\r[{}/s] [{} / {} ~ ({}%)]\x1b[K",
        format_bytes(speed).yellow().bold(),
        format_bytes(downloaded).green(),
        total_size_str.green().bold(),
        percent_str.bold().red()
    );

    stdout().flush().unwrap();
}

pub async fn progress(url: String, path: String) -> bool {
    let mut client = match DownloaderBuilder::new()
        .with_callback(report_callback)
        .build()
    {
        Ok(c) => c,
        Err(err) => {
            println!(
                "[DOWNLOADER] Failed to build a downloader! (Error: {:?})",
                err
            );
            return false;
        }
    };

    match client.download(&url, &path).await {
        Ok(_) => true,
        Err(err) => {
            println!(
                "[DOWNLOADER] Encountered an error while downloading: {:?}",
                err
            );
            false
        }
    }
}
