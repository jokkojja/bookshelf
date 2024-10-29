# Stage 1: Building
FROM rust:1.82 AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY . .

RUN cargo build --release

# Stage 2: Run app
FROM ubuntu:22.04
WORKDIR /app

COPY --from=builder /app/target/release/bookshelf ./

# ENTRYPOINT ["tail", "-f", "/dev/null"]
ENTRYPOINT [ "./bookshelf" ]