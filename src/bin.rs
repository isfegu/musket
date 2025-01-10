#[tokio::main]
async fn main() {
    match lib::run().await {
        Ok(messages) => {
            for message in &messages {
                println!("{message}");
            }
        }
        Err(e) => {
            eprintln!("Sorry, but the following error has occurred: \n\t{e}");
            std::process::exit(1);
        }
    }
}
