use std::error::Error;

#[cfg(not(debug_assertions))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Run the backend server
    backend::run(None).await;
    Ok(())
}

// main only for `cargo run` and not `cargo run --release`
#[cfg(debug_assertions)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use backend::api::ApiDocs;
    use utoipa::OpenApi;
    let docs = ApiDocs::openapi();
    // Save docs to file
    save_docs(&docs).await.unwrap();
    // Run the backend server
    backend::run(Some(docs)).await;
    Ok(())
}

#[cfg(debug_assertions)]
async fn save_docs(docs: &utoipa::openapi::OpenApi) -> Result<(), std::io::Error> {
    use std::path::Path;
    use tokio::{fs::File, io::AsyncWriteExt};
    let docs = docs.to_pretty_json()?;
    let path = Path::new("docs.json");
    let mut file = File::create(&path).await?;
    file.write_all(docs.as_bytes()).await?;
    Ok(())
}
