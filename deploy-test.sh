#!/bin/bash
# Script deploy ke VPS

set -e

REMOTE_USER="asephs"
REMOTE_HOST="70.153.9.42"
REMOTE_PATH="/home/asephs/imphnen-backend-service"

# Build project
taskset -c 0,1 cargo build --release -j 2 

# Rsync hasil build dan file yang diperlukan
rsync -avz --delete \
    target/release/ \
    $REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH/target/release/

# Sync file konfigurasi dan source code (jika perlu)
rsync -avz --delete \
    imphnen-backend/ \
    $REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH/imphnen-backend/

rsync -avz --delete \
    docker-compose.yml Dockerfile \
    $REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH/

# Tambahkan file lain jika diperlukan
ssh $REMOTE_USER@$REMOTE_HOST << 'EOF'
cd /home/asephs/imphnen-backend-service
export PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin"
export PATH=$PATH:/root/.local/share/pnpm
export PATH=$PATH:/home/asephs/.nvm/versions/node/v22.17.1/bin
export PATH=$PATH:/home/asephs/.bun/bin/bun

if [ -f ~/.bashrc ]; then
    source ~/.bashrc
fi
pm2 restart 4 --update-env
EOF

echo "Deploy selesai ke $REMOTE_HOST:$REMOTE_PATH"
