FROM rust:latest AS builder
WORKDIR /pos-be
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /pos-be/target/release/pos-be /pos-be
ENTRYPOINT ["/pos-be"]
EXPOSE 8080
