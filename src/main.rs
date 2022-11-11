mod lib;
// use tokio;

#[tokio::main]
async fn main() {
	lib::progress(
		"https://speed.hetzner.de/1GB.bin".to_string(),
		"test.bin".to_string()
	).await;
}
