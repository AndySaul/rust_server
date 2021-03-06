extern crate clap;

use clap::{clap_app, ArgMatches};

pub struct Parser<'a> {
    pub args: ArgMatches<'a>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Self {
            args: clap_app!(server=>
                (version: env!("CARGO_PKG_VERSION"))
                (about: "Basic REST server")
                (@arg ADDRESS: +required
                    "Sets the IP address to listen on, e.g. 127.0.0.1:8080")
            )
            .get_matches()
        }
    }
    pub fn address(self) -> String {
        self.args.value_of("ADDRESS").unwrap().to_string()
    }
}
