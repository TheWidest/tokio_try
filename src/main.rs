use tokio::time::{sleep, Duration};
use tokio_try::{time_consumer, file_reader};

async fn run() {
    tokio::join!(
        time_consumer(),
        file_reader("script.txt"),
    );
}

#[tokio::main]
async fn main() {
    println!("Starting async operation...");
    
    run().await;
    
    println!("End of async operation.");
}