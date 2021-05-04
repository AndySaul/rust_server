use server::Server;
use std::env;
use website_handler::Website;

mod http;
mod server;
mod website_handler;

#[cfg(test)]
mod server_test;

fn main() -> std::io::Result<()> {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(Website::new(public_path()))
}

fn public_path() -> String {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    env::var("PUBLIC_PATH").unwrap_or(default_path)
}
