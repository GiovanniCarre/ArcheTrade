$imageName = "my-mongo-init"
$containerName = "mongo-init-container"
$projectDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$mongoDataDir = "C:/Users/giova/mongo_data"

#Vérifier que Docker est installé
if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
    Write-Error "Docker n'est pas installé ou accessible"
    exit 1
}
docker build -t $imageName $projectDir

# Supprimer le container existant s'il existe
if (docker ps -a --format "{{.Names}}" | Select-String $containerName) {
    Write-Host "Suppression de l'ancien container..."
    docker rm -f $containerName
}

# Créer le dossier de données si nécessaire
if (-not (Test-Path $mongoDataDir)) {
    New-Item -ItemType Directory -Path $mongoDataDir | Out-Null
}

Write-Host "Lancement du container MongoDB"
docker run -d `
    --name $containerName `
    -p 27017:27017 `
    -v "${projectDir}/mongo-init:/docker-entrypoint-initdb.d" `
    -v "${mongoDataDir}:/data/db" `
    --restart unless-stopped `
    $imageName

Write-Host "MongoDB devrait maintenant etre accessible sur localhost:27017"
docker ps --filter "name=$containerName"