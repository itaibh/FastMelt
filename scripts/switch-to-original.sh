#!/usr/bin/env bash
# Restore Kdenlive Flatpak to original/stock MLT binaries

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

require_root "$@"

FLATPAK_DIR="$(find_flatpak_dir)"
BIN_DIR="$FLATPAK_DIR/bin"
LIB_DIR="$FLATPAK_DIR/lib"

echo "Restoring Kdenlive Flatpak to original/stock MLT binaries..."

# 1. Restore bin/melt -> melt-7
if [ -f "$BIN_DIR/melt-7" ]; then
    echo "[*] Restoring $BIN_DIR/melt -> melt-7"
    ln -sf "melt-7" "$BIN_DIR/melt"
else
    echo "[!] Warning: $BIN_DIR/melt-7 not found!" >&2
fi

# 2. Restore lib/libmlt-7.so -> libmlt-7.so.7
echo "[*] Restoring $LIB_DIR/libmlt-7.so -> libmlt-7.so.7"
ln -sf "libmlt-7.so.7" "$LIB_DIR/libmlt-7.so"

# 3. Restore lib/libmlt-7.so.7 -> libmlt-7.so.7.43.0 (in case --include-so7 was used)
if [ -f "$LIB_DIR/libmlt-7.so.7.43.0" ]; then
    echo "[*] Restoring $LIB_DIR/libmlt-7.so.7 -> libmlt-7.so.7.43.0"
    ln -sf "libmlt-7.so.7.43.0" "$LIB_DIR/libmlt-7.so.7"
fi

# 4. Clean up / restore lib/libmlt.so
if [ -L "$LIB_DIR/libmlt.so" ]; then
    echo "[*] Removing custom symlink $LIB_DIR/libmlt.so"
    rm -f "$LIB_DIR/libmlt.so"
fi

echo
echo "[✓] Successfully restored Flatpak to stock MLT binaries!"
echo
"$SCRIPT_DIR/status.sh"

