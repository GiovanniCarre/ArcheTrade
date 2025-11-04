# Image avec Rust et Cargo pour le dev
FROM rust:1.83

# Répertoire de travail
WORKDIR /usr/src/backend

# Copier le projet dans l'image (utile pour build initial)
COPY . .

# Installer des outils utiles pour dev (optionnel)
RUN rustup component add clippy rustfmt

# Exposer le port utilisé par ton serveur
EXPOSE 3000

# Lancer le serveur en mode dev avec hot reload
CMD ["cargo", "run"]
