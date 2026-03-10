# STAGE 1: Builder
FROM rust:latest AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

# STAGE 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy compiled binary
COPY --from=builder /app/target/release/rust_netflix_backend /usr/local/bin/app

ENV PORT=8080
EXPOSE 8080

CMD ["app"]