use tokio;
mod config;
mod loader;
mod optimizor;


#[tokio::main]
async fn main() {
    loader::loader().await;
}
