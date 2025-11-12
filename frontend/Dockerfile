# ========================
# Étape 1 : Builder
# ========================
FROM node:20-alpine AS builder
WORKDIR /app

# Copier package.json et package-lock.json / yarn.lock
COPY package*.json ./
RUN npm ci

# Copier le reste du projet
COPY . .

# Générer la build SSR
# Assure-toi que ton package.json a un script "build:ssr" ou "build"
RUN npm run build:ssr

# ========================
# Étape 2 : Runtime Node
# ========================
FROM node:20-alpine AS runtime
WORKDIR /app

COPY --from=builder /app/package*.json ./
RUN npm ci --production

COPY --from=builder /app/dist /app/dist

EXPOSE 3000

CMD ["node", "dist/server/server.js"]
