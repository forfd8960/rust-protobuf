# Rust Protobuf

## Dependency

```toml
[dependencies]
prost = "0.13.1"
bytes = "1.7.1"
prost-types = "0.13.1"

[build-dependencies]
anyhow = "1.0.86"
prost-build = { version="0.13.1" }
```

## build.rs

```rust
use anyhow::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .bytes(&["."])
        .out_dir("src/pb")
        .compile_protos(&["user.proto", "blog.proto"], &["."])?;

    Ok(())
}

```

## run

```sh
cargo run .
   Compiling rust-protobuf v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.06s
     Running `target/debug/rust-protobuf .`
user: User { id: "0x123", name: "John", email: "john@example.com", salary: 1000, role: "admin", created_at: Some(Timestamp { seconds: 1724110007, nanos: 759255000 }) }

blog: Blog { id: 100, title: "Hello world", content: "Hello world", author: "John", created_at: Some(Timestamp { seconds: 1724110965, nanos: 621358000 }), updated_at: Some(Timestamp { seconds: 1724110965, nanos: 621358000 }), tags: ["rust", "protobuf"] }
```
