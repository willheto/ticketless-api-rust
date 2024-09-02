# Ticket sales API

## Requirements

Docker and Rust must be installed.

## Technologies used

Rust,
Actix,
SeaORM,
MariaDB

## Installation

First spin up the database container

```bash
docker-compose up -d
```

Then navigate to /database folder and run db migrations:

```bash
cargo run -- up
```

Then run 

```bash
cargo watch -x run
```

on root folder
