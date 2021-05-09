use server::website_handler::Website;
use server::Server;
use std::env;

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
