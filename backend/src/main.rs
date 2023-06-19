#[tokio::main]
async fn main() {
    let sock_ip = std::env::var("SOCKET_IP").unwrap_or("127.0.0.1".into());
    println!("{}", sock_ip);
    backend::run().await
}
