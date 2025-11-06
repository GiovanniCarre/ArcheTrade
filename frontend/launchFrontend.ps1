# Charger le .env
Get-Content .env | ForEach-Object {
    if ($_ -match "^(.*?)=(.*)$") {
        Set-Item -Path "Env:$($matches[1])" -Value $matches[2]
    }
}

# Supprimer l'ancien container
$oldContainer = docker ps -a -q --filter "name=marketpulse-frontend"
if ($oldContainer) {
    docker stop $oldContainer | Out-Null
    docker rm $oldContainer | Out-Null
}

# Build l'image
docker build -t marketpulse-frontend .

# Run le container sur un port libre (ici 3001)
docker run -d -p 3001:80 -e BACKEND_URL=$env:BACKEND_URL --name marketpulse-frontend marketpulse-frontend

Write-Host "Frontend running on http://localhost:3001"
