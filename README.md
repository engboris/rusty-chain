# rusty-chain

Blazingly naive blockchain implementation for learning purposes

## Build

```
cargo build
```

### Run

Server

```
RUST_LOG=info cargo run --bin server
```

Use `RUST_LOG=error` if you only want to see errors and `RUST_LOG=debug` if you
want to see all information including for mining (nounce and calculated hash).

Client

```
cargo run --bin client
```
