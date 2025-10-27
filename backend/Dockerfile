# Étape 1 : Build
FROM rust:1.72 as builder

WORKDIR /usr/src/backend

# Copier Cargo.toml et Cargo.lock pour le cache
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release
# Maintenant copier le vrai code
COPY ./src ./src
RUN cargo build --release

# Étape 2 : Runtime léger
FROM debian:buster-slim

WORKDIR /usr/app
COPY --from=builder /usr/src/backend/target/release/backend .

# Exposer le port (celui d'Axum)
EXPOSE 3000

CMD ["./backend"]