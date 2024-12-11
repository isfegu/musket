#[tokio::main]
async fn main() {
    match lib::run().await {
        Ok(messages) => {
            messages.iter().for_each(|message| println!("{message}"));
        }
        Err(e) => {
            eprintln!("Sorry, but the following error has occurred: \n\t{e}");
            std::process::exit(1);
        }
    }
}
