#!/bin/bash
# deploy.sh - Script for manual deployment to Digital Ocean

# Check if tag is provided
if [ -z "$1" ]; then
  echo "Usage: ./deploy.sh <tag>"
  echo "Example: ./deploy.sh v1.0.0"
  exit 1
fi

TAG=$1
IMAGE_NAME="aribowobob/pos-be:$TAG"

# Ask for Digital Ocean droplet IP if not set
if [ -z "$DROPLET_IP" ]; then
  read -p "Enter your Digital Ocean droplet IP: " DROPLET_IP
fi

echo "Deploying $IMAGE_NAME to $DROPLET_IP..."

# Pull the latest image
ssh root@$DROPLET_IP "mkdir -p /root/pos-app"
scp docker-compose.yml root@$DROPLET_IP:/root/pos-app/
ssh root@$DROPLET_IP "cd /root/pos-app && docker pull $IMAGE_NAME && docker-compose down && docker-compose up -d"

echo "Deployment completed!"
