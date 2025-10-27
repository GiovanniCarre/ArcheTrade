# Étape 1 : Builder
FROM node:20-alpine AS builder

# Définir le répertoire de travail
WORKDIR /app

# Copier package.json et package-lock.json
COPY package*.json ./

# Installer les dépendances
RUN npm install

# Copier tout le code source
COPY . .

# Build de l'application
RUN npm run build

# Étape 2 : Serveur Nginx
FROM nginx:stable-alpine

# Copier les fichiers build dans le dossier de nginx
COPY --from=builder /app/dist /usr/share/nginx/html

# Copier le fichier de configuration nginx si nécessaire
# COPY nginx.conf /etc/nginx/conf.d/default.conf

# Exposer le port 80
EXPOSE 80

# Démarrer nginx
CMD ["nginx", "-g", "daemon off;"]
