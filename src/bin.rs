use tracing::{error, info};

#[tokio::main]
async fn main() {
    match lib::entrypoint::run().await {
        Ok(messages) => {
            for message in &messages {
                info!("{message}");
            }
        }
        Err(e) => {
            error!("{e}");
            std::process::exit(1);
        }
    }
}
