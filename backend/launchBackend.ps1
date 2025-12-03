$imageName = "rust-axum-app"
$containerName = "rust-axum-container"
Write-Host "Build de l'image Docker..."
docker build -t $imageName .

if (docker ps -a --format "{{.Names}}" | Select-String -Pattern $containerName) {
    Write-Host "🗑 Suppression de l'ancien conteneur..."
    docker rm -f $containerName
}

#Lancer le conteneur avec les variables d'environnement depuis .env
Write-Host "Lancement du conteneur..."
docker run `
    --name $containerName `
    --env-file .env `
    -p 3000:3000 `
    $imageName

Write-Host "App lancée sur http://localhost:3000"
