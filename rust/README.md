## Requirements

1. `cargo install cargo-leptos` - install `cargo-leptos` binary
2. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
5. `cargo install diesel_cli --no-default-features --features postgres` - install Diesel CLI

## Before you run

1. `echo DATABASE_URL=postgres://localhost:5432/rust_user_demo > .env` - create `.env` file
2. `diesel setup` - create database and migrations folder

## Running your project

```bash
cargo leptos watch
```

```bash
cargo leptos watch --release
```

## Compiling for Release

```bash
cargo leptos build --release
```

## Testing Your Project

```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```