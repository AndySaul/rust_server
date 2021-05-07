use server::Server;
use std::env;
use website_handler::Website;

mod http;
mod server;
mod website_handler;

#[cfg(test)]
mod server_test;

fn main() -> std::io::Result<()> {
    let server = Server::new(address());
    server.run(Website::new(public_path()))
}

fn address() -> String {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    let address = if args.len() > 1 {
        &args[1]
    } else {
        "127.0.0.1:8080"
    };
    address.to_string()
}

fn public_path() -> String {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    env::var("PUBLIC_PATH").unwrap_or(default_path)
}
