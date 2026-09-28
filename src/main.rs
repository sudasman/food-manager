mod manager;
#[tokio::main]
async fn main() {
    manager::new().await;
}
