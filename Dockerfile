# Stage 1: Building
FROM rust:1.82 AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .

RUN cargo build --release

# Stage 2: Run app
FROM ubuntu:22.04
WORKDIR /app

COPY --from=builder /app/target/release/bookshelf ./

ENTRYPOINT ["./bookshelf"]