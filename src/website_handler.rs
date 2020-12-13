use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;
use std::path::PathBuf;

pub struct Website {
    public_path: String,
}

impl Website {
    pub fn new(public_path: String) -> Self {
        Website { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let absolute_path = PathBuf::from(format!("{}/{}", self.public_path, file_path));

        if !absolute_path.is_file() {
            return None;
        }

        match absolute_path.as_path().canonicalize() {
            Ok(path) => {
                println!("display: {}", path.display());

                let public = PathBuf::from(&self.public_path);
                match public.as_path().canonicalize() {
                    Ok(p) => {
                        if path.starts_with(p) {
                            fs::read_to_string(path).ok()
                        } else {
                            println!("Directory Traversal Attack attempted: {}", file_path);
                            None
                        }
                    }
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for Website {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
