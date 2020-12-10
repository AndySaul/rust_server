use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        println!("listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((stream, address)) => {
                    println!("OK");
                }
                Err(e) => {
                    println!("Failed to establish connection: {}", e);
                }
            }
            // let result = listener.accept();

            // if result.is_err() {
            //     continue;
            // }

            // let (stream, address) = result.unwrap();
        }
    }
}
