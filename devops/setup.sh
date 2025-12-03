#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="${1:-}"

echo "=== START: setup Jenkins on Docker ==="

# Helper
command_exists() {
  command -v "$1" >/dev/null 2>&1
}

# 1) Installer Docker si absent
if command_exists docker; then
  echo "Docker est déjà installé : $(docker --version)"
else
  echo "Docker non trouvé — installation en cours..."
  apt-get update -y
  apt-get install -y ca-certificates curl gnupg lsb-release

  mkdir -p /etc/apt/keyrings
  curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg
  echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
    $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null

  apt-get update -y
  apt-get install -y docker-ce docker-ce-cli containerd.io

  systemctl enable docker
  systemctl start docker

  echo "Docker installé : $(docker --version)"
fi

if command_exists docker-compose; then
  echo "docker-compose déjà présent : $(docker-compose --version)"
else
  echo "Installation de docker-compose via binaire officiel..."
  DOCKER_COMPOSE_VERSION="2.23.3"
  curl -L "https://github.com/docker/compose/releases/download/v${DOCKER_COMPOSE_VERSION}/docker-compose-$(uname -s)-$(uname -m)" \
    -o /usr/local/bin/docker-compose
  chmod +x /usr/local/bin/docker-compose
  echo "docker-compose version: $(docker-compose --version)"
fi

CURRENT_USER="${SUDO_USER:-$USER}"
if id -nG "$CURRENT_USER" | grep -qw docker; then
  echo "Utilisateur $CURRENT_USER déjà dans le groupe docker."
else
  echo "Ajout de l'utilisateur $CURRENT_USER au groupe docker..."
  usermod -aG docker "$CURRENT_USER" || true
  echo "Reconnectez-vous pour que le groupe soit pris en compte."
fi

if docker volume ls -q | grep -qw jenkins-data; then
  echo "Volume jenkins-data déjà présent."
else
  echo "Création du volume jenkins-data..."
  docker volume create jenkins-data >/dev/null
fi

if docker ps -a --format '{{.Names}}' | grep -qw '^jenkins$'; then
  echo "Conteneur 'jenkins' trouvé — arrêt et suppression..."
  docker rm -f jenkins >/dev/null || true
fi

echo "Pull de l'image jenkins/jenkins:lts..."
docker pull jenkins/jenkins:lts >/dev/null

RUN_CMD=(docker run -d
  --name jenkins
  --restart unless-stopped
  -p 8080:8080
  -p 50000:50000
  -v jenkins-data:/var/jenkins_home
  -v /var/run/docker.sock:/var/run/docker.sock
  -v /usr/local/bin/docker-compose:/usr/local/bin/docker-compose
  --user root
)

# Montage optionnel du projet
if [[ -n "$PROJECT_DIR" ]]; then
  if [[ -d "$PROJECT_DIR" ]]; then
    echo "Montage du répertoire projet dans Jenkins: $PROJECT_DIR -> /var/jenkins_home/workspace/marketpulse"
    RUN_CMD+=(-v "$PROJECT_DIR":/var/jenkins_home/workspace/marketpulse)
  else
    echo "PROJECT_DIR fourni mais introuvable: $PROJECT_DIR (ignorer le montage)"
  fi
fi

RUN_CMD+=(jenkins/jenkins:lts)

echo "Lancement du conteneur Jenkins..."
"${RUN_CMD[@]}"

sleep 2
echo "Conteneurs Jenkins (résumé):"
docker ps --filter "name=jenkins" --format "table {{.ID}}\t{{.Image}}\t{{.Names}}\t{{.Status}}\t{{.Ports}}"

echo
echo "=== FIN: setup Jenkins on Docker ==="
echo "Accéder à Jenkins: http://<IP_DU_VPS>:8080"
echo "Pour obtenir le mot de passe initial :"
echo "  docker exec -it jenkins cat /var/jenkins_home/secrets/initialAdminPassword"
echo "Reconnectez-vous si nécessaire pour que le groupe docker soit pris en compte."
