# Rust server

My first stab at Rust

This is based on the Udemy course https://www.udemy.com/course/rust-fundamentals/

Also on the Rust book https://doc.rust-lang.org/book/title-page.html

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