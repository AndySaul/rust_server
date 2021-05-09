# Rust server

My first stab at Rust. Currently, this is a single-threaded webserver with limited features. Next steps:
- Make it multithreaded

This is based on the 
[Udemy course](https://www.udemy.com/course/rust-fundamentals/ "Learn Rust by Building Real Applications")
and the [Rust Book](https://doc.rust-lang.org/book/title-page.html "The Rust Programming Language")

## Getting started

* Run this crate
    * ```cargo run```
* Browse to the server address
    * http://127.0.0.1:8080/ 
    * http://127.0.0.1:8080/hello

## Installing rust

https://www.rust-lang.org/tools/install

After installing, run `cargo` commands from a terminal.
Microsoft Visual Code has good Rust support

### Cargo

https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

Create a new Rust project:
```rust
cargo new project_name
```

Format all files in the project using Rust standardized formatting:
```rust
cargo fmt
```

Build & run in debug mode:
```rust
cargo run
```

Build debug binaries in ./target/debug
```rust
cargo build
```


Build release binaries in ./target/release
```rust
cargo build --release
```
