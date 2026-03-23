FROM rust:1.72-slim as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN rustup component add rustfmt
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/messaging-service /usr/local/bin/messaging
ENV RUST_LOG=info
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/messaging"]
