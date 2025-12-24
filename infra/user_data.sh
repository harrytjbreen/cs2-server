#!/bin/bash
set -euxo pipefail

APP_DIR="/opt/cs2"
REPO_URL="https://github.com/harrytjbreen/cs2-server.git"
BRANCH="master"
REGION="eu-west-2"
SECRET_NAME="cs2/prod"

# ------------------------
# Base system
# ------------------------
apt-get update
apt-get install -y \
  ca-certificates \
  curl \
  gnupg \
  lsb-release \
  git \
  unzip \
  jq

# ------------------------
# Docker
# ------------------------
if ! command -v docker >/dev/null; then
  curl -fsSL https://get.docker.com | sh
fi

systemctl enable docker
systemctl start docker

# Docker Compose v2
if ! docker compose version >/dev/null 2>&1; then
  mkdir -p /usr/local/lib/docker/cli-plugins
  curl -SL https://github.com/docker/compose/releases/download/v2.29.2/docker-compose-linux-x86_64 \
    -o /usr/local/lib/docker/cli-plugins/docker-compose
  chmod +x /usr/local/lib/docker/cli-plugins/docker-compose
fi

# ------------------------
# App
# ------------------------
mkdir -p "$APP_DIR"

if [ ! -d "$APP_DIR/.git" ]; then
  git clone --branch "$BRANCH" "$REPO_URL" "$APP_DIR"
fi

# ------------------------
# AWS CLI
# ------------------------
if ! command -v aws >/dev/null; then
  curl -s "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o /tmp/awscliv2.zip
  unzip -q /tmp/awscliv2.zip -d /tmp
  /tmp/aws/install
fi

# ------------------------
# Secrets → env file
# ------------------------
aws secretsmanager get-secret-value \
  --region "$REGION" \
  --secret-id "$SECRET_NAME" \
  --query SecretString \
  --output text \
| jq -r 'to_entries | map("\(.key)=\(.value)") | .[]' \
> "$APP_DIR/.env"

chmod 600 "$APP_DIR/.env"

# ------------------------
# Deploy script (root)
# ------------------------
cat >"$APP_DIR/deploy.sh" <<'EOF'
#!/bin/bash
set -euo pipefail

cd /opt/cs2
BRANCH="master"

echo "Updating repo"
git fetch origin
git checkout "$BRANCH"
git reset --hard "origin/$BRANCH"

echo "Pulling images"
docker compose pull

echo "Starting containers"
docker compose up -d
EOF

chmod +x "$APP_DIR/deploy.sh"

# ------------------------
# systemd service (root)
# ------------------------
cat >/etc/systemd/system/cs2.service <<EOF
[Unit]
Description=CS2 Server (Docker Compose)
Requires=docker.service
After=docker.service

[Service]
Type=oneshot
RemainAfterExit=true
WorkingDirectory=$APP_DIR
EnvironmentFile=$APP_DIR/.env
ExecStart=$APP_DIR/deploy.sh
ExecStop=/usr/bin/docker compose down
TimeoutStartSec=0

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable cs2
systemctl start cs2

# ------------------------
# SSM agent (Ubuntu)
# ------------------------
snap install amazon-ssm-agent --classic || true
systemctl enable snap.amazon-ssm-agent.amazon-ssm-agent
systemctl start snap.amazon-ssm-agent.amazon-ssm-agent

echo "CS2 bootstrap complete"
