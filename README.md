# Rust server

My first stab at Rust. Currently, this is a single-threaded webserver with limited features. Next steps:
- Make it multithreaded
- Add unit tests

This is based on the 
[Udemy course](https://www.udemy.com/course/rust-fundamentals/ "Learn Rust by Building Real Applications")
and the [Rust Book](https://doc.rust-lang.org/book/title-page.html "The Rust Programming Language")

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

Build project:
```rust
cargo build
```

Build & run in debug mode:
```rust
cargo run
```

Run with release optimizations:
```rust
cargo build --release
```