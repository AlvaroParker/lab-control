use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Run the backend server
    backend::run().await;
    Ok(())
}
