# ----------------------------
# Build stage
# ----------------------------
FROM rust:1.75-bookworm AS builder

WORKDIR /app

# Copy everything needed to build
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build
RUN cargo build --release

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
