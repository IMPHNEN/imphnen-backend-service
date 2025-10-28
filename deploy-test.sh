#!/bin/bash
# Script deploy semua binary hasil build Rust (untuk Linux)

set -e

REMOTE_USER="asephs"
REMOTE_HOST="70.153.9.42"
REMOTE_PATH="/home/asephs/imphnen-backend-service"

# Build release binary
echo "🔧 Building Rust project..."
cargo build --release

# Filter hanya file executable tanpa ekstensi .exe, .rlib, atau .d
BINARIES=$(find target/release -maxdepth 1 -type f ! -name "*.exe" ! -name "*.rlib" ! -name "*.d")

if [ -z "$BINARIES" ]; then
    echo "❌ Tidak ada binary Linux (.exe/.rlib/.d diabaikan)"
    exit 1
fi

# Upload semua binary yang valid ke server
echo "🚀 Mengirim binary ke server..."
rsync -avz --compress-level=9 --progress \
    $BINARIES \
    $REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH/target/release/

# Jalankan ulang service utama di VPS
echo "♻️  Restart service utama di server..."
ssh $REMOTE_USER@$REMOTE_HOST << 'EOF'
set -e
cd /home/asephs/imphnen-backend-service
export PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
if [ -f ~/.bashrc ]; then
    source ~/.bashrc
fi

# Pastikan semua binary bisa dieksekusi
chmod +x target/release/*

# Restart service utama (misalnya api.d)
if pm2 list | grep -q api; then
    pm2 restart api --update-env
else
    pm2 start target/release/api.d --name api
fi
EOF

echo "✅ Deploy semua binary Linux selesai ke $REMOTE_HOST:$REMOTE_PATH"
