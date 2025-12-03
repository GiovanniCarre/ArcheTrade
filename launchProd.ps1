# Chemin du docker-compose.yml
$composeFile = "docker-compose.yml"

Write-Host "📦 Build et lancement des services Docker..."
docker-compose -f $composeFile up --build -d

Write-Host "✅ Services lancés en arrière-plan."

# Attendre 2-3 secondes pour que Mongo et le backend démarrent
Start-Sleep -Seconds 3

# Afficher les logs du backend en temps réel
Write-Host "📝 Affichage des logs du backend (CTRL+C pour quitter)..."
docker-compose -f $composeFile logs -f backend
