async fn life() -> u32 {
    42
}

// general purpose async executor is the tokio function
#[tokio::main]
pub async fn main() {
    let future = life();
    let value = future.await;

    let value: u32 = life().await;
}