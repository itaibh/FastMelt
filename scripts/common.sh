#!/usr/bin/env bash
# Common configuration and helper functions for FastMelt Flatpak switching

set -euo pipefail

# Determine repository root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Project binary paths
TARGET_RELEASE_DIR="$REPO_DIR/target/release"
PROJECT_MELT="$TARGET_RELEASE_DIR/melt"
PROJECT_LIBMLT="$TARGET_RELEASE_DIR/libmlt_7.so"

# Locate Kdenlive Flatpak directory
find_flatpak_dir() {
    local candidates=(
        "/var/lib/flatpak/app/org.kde.kdenlive/current/active/files"
        "/var/lib/flatpak/app/org.kde.kdenlive/x86_64/stable/active/files"
        "$HOME/.local/share/flatpak/app/org.kde.kdenlive/current/active/files"
        "$HOME/.local/share/flatpak/app/org.kde.kdenlive/x86_64/stable/active/files"
    )

    for candidate in "${candidates[@]}"; do
        if [ -d "$candidate" ]; then
            readlink -f "$candidate"
            return 0
        fi
    done

    echo "Error: Could not locate Kdenlive Flatpak installation directory." >&2
    return 1
}

# Ensure root privileges for modifying Flatpak files
require_root() {
    if [ "$EUID" -ne 0 ]; then
        echo "[i] Root privileges required to update Flatpak files in /var/lib/flatpak."
        echo "[i] Elevating with sudo..."
        exec sudo "$0" "$@"
    fi
}

