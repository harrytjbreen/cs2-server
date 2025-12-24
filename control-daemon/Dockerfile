# ----------------------------
# Build stage
# ----------------------------
FROM rust:1.78-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --verbose

# ----------------------------
# Runtime stage
# ----------------------------
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary (adjust name if needed)
COPY --from=builder /app/target/release/control-deamon /usr/local/bin/control-deamon

USER nobody

ENTRYPOINT ["control-deamon"]
