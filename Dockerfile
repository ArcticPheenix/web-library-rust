FROM rust:1.58-slim AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:stretch-slim
RUN apt-get update && apt-get upgrade -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/web-library-rust /usr/bin/web-library-rust
CMD ["/usr/bin/web-library-rust"]
EXPOSE 8080