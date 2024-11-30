#[tokio::main]
async fn main() {
    if let Err(e) = lib::run().await {
        eprintln!("Sorry, but the following error has occurred: \n\t{e}");
        std::process::exit(1);
    }
}
