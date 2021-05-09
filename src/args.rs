use std::env;

pub fn address() -> String {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    let address = if args.len() > 1 {
        &args[1]
    } else {
        "127.0.0.1:8080"
    };
    address.to_string()
}
