//! Run with
//!
//! ```not_rust
//! cargo run -p example-sqlite
//! ```
use password_auth::generate_hash;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::web::App;

mod users;
mod utils;
mod web;

// helper function to generate new password hashes
fn generate_pwd_hashes() -> (String, String) {
    let ranghi_hash = generate_hash("latini");
    let tanque_hash = generate_hash("slavaukraini");

    (ranghi_hash, tanque_hash)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // uncomment below lines to generate new password hashes and print them to console
    // so to update hash in migration file
    // let (ranghi_pwd, tanque_pwd) = generate_pwd_hashes();
    // println!(
    //     "ranghi pwd hash: {:#?}\ntanque pwd hash: {:#?}",
    //     ranghi_pwd, tanque_pwd
    // );

    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| "axum_login=debug,tower_sessions=debug,sqlx=warn,tower_http=debug".into(),
        )))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    App::new().await?.serve().await
}
