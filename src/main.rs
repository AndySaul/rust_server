use server::args::Parser;
use server::website_handler::Website;
use server::Server;
use std::env;

fn main() -> std::io::Result<()> {
    let parser = Parser::new();

    let server = Server::new(parser.address());
    server.run(Website::new(public_path()))
}

fn public_path() -> String {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    env::var("PUBLIC_PATH").unwrap_or(default_path)
}
