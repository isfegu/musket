#[tokio::main]
async fn main() {
    if let Err(e) = lib::run().await {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
