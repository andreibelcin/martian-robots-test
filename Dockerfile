FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/barclays_challenge /usr/local/bin/barclays_challenge
ENTRYPOINT ["/usr/local/bin/barclays_challenge"]