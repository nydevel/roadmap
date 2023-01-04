# ROADMAP
Skills roadmap.

It's like midnmap, but with interface for data attachments (articles, links, video) for learning + data status (to read, done etc.)
So, you can build your wished skills roadmap and attach resources required for learning.

## Tech-stack
Back + front
Rust lang only:)

## Schemas
https://miro.com/app/board/uXjVP3DP4Gc=

## Preinstall requirements
For Debian:
```bash
sudo apt install build-essential
```

1. Install Rust
2. Install sqlx-cli fro migrations
```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```
3. Copy .env.example to .env file
4. Install docker
5. Run docker compose
```bash
docker compose up -d
```
6. SQLX DB create
```bash
sqlx database create
```