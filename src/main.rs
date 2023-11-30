mod config;
mod loader;
mod optimizor;
mod utils;


#[tokio::main]
async fn main() {
    loader::loader().await;
}
