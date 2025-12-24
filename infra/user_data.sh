#!/bin/bash
set -euxo pipefail

# Config (edit these)

APP_USER="ubuntu"
APP_DIR="/opt/cs2"
REPO_URL="https://github.com/harrytjbreen/cs2-server.git"
BRANCH="master"

# Base system

apt-get update
apt-get install -y \
  ca-certificates \
  curl \
  gnupg \
  lsb-release \
  git

# Docker

if ! command -v docker >/dev/null; then
  curl -fsSL https://get.docker.com | sh
fi

# Docker Compose v2

if ! docker compose version >/dev/null 2>&1; then
  mkdir -p /usr/local/lib/docker/cli-plugins
  curl -SL https://github.com/docker/compose/releases/download/v2.29.2/docker-compose-linux-x86_64 \
    -o /usr/local/lib/docker/cli-plugins/docker-compose
  chmod +x /usr/local/lib/docker/cli-plugins/docker-compose
fi

systemctl enable docker
systemctl start docker

# App directory

mkdir -p "$APP_DIR"
chown -R "$APP_USER:$APP_USER" "$APP_DIR"

# Clone repo (only once)

if [ ! -d "$APP_DIR/.git" ]; then
  sudo -u "$APP_USER" git clone \
    --branch "$BRANCH" \
    "$REPO_URL" \
    "$APP_DIR"
fi

# Deploy script

cat >/opt/cs2/deploy.sh <<'EOF'
#!/bin/bash
set -euo pipefail

cd /opt/cs2

echo "Updating repo"
git fetch origin
git checkout main
git reset --hard origin/main

echo "Pulling Docker images"
docker compose pull

echo "Starting containers"
docker compose up -d
EOF

chmod +x /opt/cs2/deploy.sh
chown "$APP_USER:$APP_USER" /opt/cs2/deploy.sh

########################################
# systemd service
########################################

cat >/etc/systemd/system/cs2.service <<'EOF'
[Unit]
Description=CS2 Server (Docker Compose, main branch)
Requires=docker.service
After=docker.service

[Service]
Type=oneshot
RemainAfterExit=true
User=ubuntu
WorkingDirectory=/opt/cs2
ExecStart=/opt/cs2/deploy.sh
ExecStop=/usr/bin/docker compose down
TimeoutStartSec=0

[Install]
WantedBy=multi-user.target
EOF

########################################
# Enable service
########################################

systemctl daemon-reload
systemctl enable cs2

########################################
# SSM agent safety (belt + braces)
########################################

snap install amazon-ssm-agent --classic || true
systemctl enable amazon-ssm-agent
systemctl restart amazon-ssm-agent

echo "CS2 bootstrap complete"
