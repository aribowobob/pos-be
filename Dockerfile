FROM rust:latest AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/pos-be /app/pos-be
WORKDIR /app
ENTRYPOINT ["/app/pos-be"]
EXPOSE 8080
