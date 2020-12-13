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

        if !self.verify(&absolute_path) {
            return None;
        }

        fs::read_to_string(absolute_path).ok()
    }

    fn verify(&self, path: &PathBuf) -> bool {
        if !path.is_file() {
            return false;
        }
        self.is_in_public_folder(path)
    }

    fn is_in_public_folder(&self, path: &PathBuf) -> bool {
        match path.canonicalize() {
            Err(_) => false,
            Ok(path) => {
                let public = PathBuf::from(&self.public_path);
                match public.canonicalize() {
                    Err(_) => false,
                    Ok(p) => path.starts_with(p),
                }
            }
        }
    }

    fn get(&mut self, path: &str) -> Response {
        match path {
            "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
            "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
            path => match self.read_file(path) {
                Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                None => Response::new(StatusCode::NotFound, None),
            },
        }
    }
}

impl Handler for Website {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => self.get(request.path()),
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
