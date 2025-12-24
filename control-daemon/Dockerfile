# Base image (shared)

FROM rust:1.75-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .

RUN cargo build --release

# Production runtime

FROM debian:bookworm-slim AS prod

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/cs2-controller /app/cs2-controller

# Non-root user (important for prod)
RUN useradd -m appuser
USER appuser

ENV RUST_LOG=info

CMD ["./cs2-controller"]
