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
  git \
  unzip

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

#get secrets

curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
sudo ./aws/install

SECRET_NAME="cs2/prod"
REGION="eu-west-2"

SECRET_JSON=$(aws secretsmanager get-secret-value \
  --region "$REGION" \
  --secret-id "$SECRET_NAME" \
  --query SecretString \
  --output text)

# Deploy script

cat >/opt/cs2/deploy.sh <<EOF
#!/bin/bash
set -euo pipefail

cd /opt/cs2

BRANCH="${BRANCH}"

echo "Updating repo (branch: \$BRANCH)"
git fetch origin
git checkout "\$BRANCH"
git reset --hard "origin/\$BRANCH"

echo "Pulling Docker images"
docker compose pull

echo "Starting containers"
docker compose up -d
EOF

chmod +x /opt/cs2/deploy.sh
chown "$APP_USER:$APP_USER" /opt/cs2/deploy.sh

# systemd service

cat >/etc/systemd/system/cs2.service <<'EOF'
[Unit]
Description=CS2 Server (Docker Compose, ${BRANCH} branch)
Requires=docker.service
After=docker.service

[Service]
Type=oneshot
RemainAfterExit=true
User=root
Environment=$(echo "$SECRET_JSON" | jq -r 'to_entries | map("\(.key)=\(.value)") | join(" ")')
WorkingDirectory=/opt/cs2
ExecStart=/opt/cs2/deploy.sh
ExecStop=/usr/bin/docker compose down
TimeoutStartSec=0

[Install]
WantedBy=multi-user.target
EOF

# Enable service
#
sudo git config --system --add safe.directory /opt/cs2

systemctl daemon-reload
systemctl enable cs2

# SSM agent safety (belt + braces)

snap install amazon-ssm-agent --classic || true
systemctl enable amazon-ssm-agent
systemctl restart amazon-ssm-agent

cd /opt/cs2
sudo ./deploy.sh

echo "CS2 bootstrap complete"
