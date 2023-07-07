#[tokio::main]
async fn main() {
    // Run the backend server
    backend::run().await
}
