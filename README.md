# rust-schema-registry-server

Avro schema registry. Compatible with Spring Cloud Schema Registry API.

## Development

### Rust
To install:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To update:
```bash
rustup update
```

### Build Project
```bash
cargo build
```

### Diesel CLI for making migrations
```bash
cargo install diesel_cli --no-default-features --features postgres sqlite
```

If you encounter an error it could be related to not having the postgres development headers. On ubuntu you can install them with the following:
```bash
sudo apt install postgresql-server-dev-all
```

For Fedora/Redhat, install `postgresql-devel` with yum or dnf:
```bash
sudo dnf install postgresql-devel
```
