# Build stage
FROM rust:1.90.0 AS builder
WORKDIR /usr/src/app

# Installer dépendances système (openssl / certs) si besoin
RUN apt-get update \
  && apt-get install -y --no-install-recommends \
     pkg-config libssl-dev ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# Copier les manifests Cargo pour cache des dépendances
COPY Cargo.toml Cargo.lock ./
# Copier tout le code
COPY src ./src

# Compiler en release
RUN cargo build --release --locked

# Runtime stage
FROM debian:bookworm-slim AS runtime
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/backend /app/backend
RUN chmod +x /app/backend

EXPOSE 3000
ENTRYPOINT ["/app/backend"]
