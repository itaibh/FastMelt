#!/usr/bin/env bash
# Switch Kdenlive Flatpak to use FastMelt (Project Binaries)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

require_root "$@"

FLATPAK_DIR="$(find_flatpak_dir)"
BIN_DIR="$FLATPAK_DIR/bin"
LIB_DIR="$FLATPAK_DIR/lib"

INCLUDE_SO7=false
for arg in "$@"; do
    if [ "$arg" == "--include-so7" ]; then
        INCLUDE_SO7=true
    fi
done

echo "Switching Kdenlive Flatpak to use FastMelt binaries..."

# 1. Verify / Build Project Binaries
if [ ! -f "$PROJECT_MELT" ] || [ ! -f "$PROJECT_LIBMLT" ]; then
    echo "[*] Project binaries not found. Building with cargo build --release..."
    if [ -n "${SUDO_USER:-}" ]; then
        su - "$SUDO_USER" -c "cd '$REPO_DIR' && cargo build --release"
    else
        (cd "$REPO_DIR" && cargo build --release)
    fi
fi

# Ensure target/release/libmlt.so symlink exists in project
ln -sf "libmlt_7.so" "$TARGET_RELEASE_DIR/libmlt.so"

# 2. Update bin/melt -> melt-7 (stock CLI client using unified FastMelt libmlt)
if [ -f "$BIN_DIR/melt-7" ]; then
    echo "[*] Pointing $BIN_DIR/melt -> melt-7 (uses unified FastMelt libmlt)"
    ln -sf "melt-7" "$BIN_DIR/melt"
fi

# 3. Update lib/libmlt.so
echo "[*] Pointing $LIB_DIR/libmlt.so -> $PROJECT_LIBMLT"
ln -sf "$PROJECT_LIBMLT" "$LIB_DIR/libmlt.so"

# 4. Update lib/libmlt-7.so
echo "[*] Pointing $LIB_DIR/libmlt-7.so -> $PROJECT_LIBMLT"
ln -sf "$PROJECT_LIBMLT" "$LIB_DIR/libmlt-7.so"

# 5. Optional libmlt-7.so.7 (for full timeline GUI override)
if [ "$INCLUDE_SO7" = true ]; then
    echo "[!] --include-so7 specified: Pointing $LIB_DIR/libmlt-7.so.7 -> $PROJECT_LIBMLT"
    ln -sf "$PROJECT_LIBMLT" "$LIB_DIR/libmlt-7.so.7"
fi

echo
echo "[✓] Successfully configured Flatpak to use FastMelt!"
echo
"$SCRIPT_DIR/status.sh"

