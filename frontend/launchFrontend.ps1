# Charger les variables depuis .env
Get-Content .env | ForEach-Object {
    if ($_ -match "^(.*?)=(.*)$") {
        Set-Item -Path "Env:$($matches[1])" -Value $matches[2]
    }
}

# Supprimer l'ancien container s'il existe
$oldContainer = docker ps -a -q --filter "name=frontend-archetrade"
if ($oldContainer) {
    docker stop $oldContainer | Out-Null
    docker rm $oldContainer | Out-Null
}

# Build l'image Docker en passant l'URL du backend en build-arg
docker build -t frontend-archetrade --build-arg VITE_BACKEND_URL=$env:BACKEND_URL .

# Run le container (Nginx écoute sur 80 à l'intérieur)
docker run -d -p 3001:80 --name frontend-archetrade frontend-archetrade

Write-Host "Frontend running on http://localhost:3001"
