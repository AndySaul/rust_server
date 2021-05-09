extern crate clap;

use clap::{clap_app, ArgMatches};

pub struct Parser<'a> {
    pub args: ArgMatches<'a>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        let args = clap_app!(app=>
            (version: "0.1.0")
            (about: "Basic REST server")
            (@arg ADDRESS: +required 
                "Sets the IP address to listen on, e.g. 127.0.0.1:8080")
        )
        .get_matches();
        Self { args }
    }
    pub fn address(self) -> String {
        self.args.value_of("ADDRESS").unwrap().to_string()
    }
}
