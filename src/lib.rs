use tokio::time::{
    Duration,
    sleep,
};
use tokio::io::AsyncReadExt;

pub async fn time_consumer() {
    println!("Time consuming operation starts now");
    sleep(Duration::from_secs(2)).await;
    println!("Time consuming operation ends now");
}

pub async fn file_reader(path: &str) {
    println!("Starting to read a file");
    let mut f = tokio::fs::File::open(path).await.expect("failed to open the file");
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.expect("failed to read the file");
    println!("The script contained {} bytes", contents.len());
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    // Test for time_consumer function
    #[tokio::test]
    async fn test_time_consumer() {
        let start = std::time::Instant::now();
        time_consumer().await;
        let duration = start.elapsed();
        // Since we expect the sleep to take around 2 seconds
        assert!(duration.as_secs() >= 2);
    }
}