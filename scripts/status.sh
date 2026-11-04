#!/usr/bin/env bash
# FastMelt Flatpak Status Checker

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

FLATPAK_DIR="$(find_flatpak_dir)"
BIN_DIR="$FLATPAK_DIR/bin"
LIB_DIR="$FLATPAK_DIR/lib"

echo "=========================================================="
echo "          Kdenlive Flatpak MLT Configuration Status       "
echo "=========================================================="
echo "Flatpak Path: $FLATPAK_DIR"
echo "Project Path: $REPO_DIR"
echo
 
# Check Project Binaries
echo "--- Project Binaries ---"
if [ -f "$PROJECT_MELT" ]; then
    echo "  [OK] melt binary: $PROJECT_MELT"
else
    echo "  [!] melt binary: NOT FOUND (run 'cargo build --release' first)"
fi

if [ -f "$PROJECT_LIBMLT" ]; then
    echo "  [OK] libmlt library: $PROJECT_LIBMLT"
else
    echo "  [!] libmlt library: NOT FOUND (run 'cargo build --release' first)"
fi
echo

# Inspect Flatpak Links
echo "--- Flatpak Links in /app ---"

MELT_LINK="$BIN_DIR/melt"
if [ -L "$MELT_LINK" ]; then
    MELT_TARGET="$(readlink "$MELT_LINK")"
    if [[ "$MELT_TARGET" == *"$PROJECT_MELT"* || "$MELT_TARGET" == *"$REPO_DIR"* ]]; then
        MELT_STATUS="[PROJECT] FastMelt"
    elif [[ "$MELT_TARGET" == "melt-7" || "$MELT_TARGET" == *"/melt-7"* ]]; then
        MELT_STATUS="[ORIGINAL] Stock MLT"
    else
        MELT_STATUS="[CUSTOM] $MELT_TARGET"
    fi
    echo "  melt:        $MELT_TARGET -> $MELT_STATUS"
elif [ -f "$MELT_LINK" ]; then
    echo "  melt:        Regular file (not a symlink)"
else
    echo "  melt:        Missing"
fi

LIBMLT_LINK="$LIB_DIR/libmlt.so"
if [ -L "$LIBMLT_LINK" ]; then
    LIBMLT_TARGET="$(readlink "$LIBMLT_LINK")"
    if [[ "$LIBMLT_TARGET" == *"$PROJECT_LIBMLT"* || "$LIBMLT_TARGET" == *"$REPO_DIR"* ]]; then
        LIBMLT_STATUS="[PROJECT] FastMelt"
    else
        LIBMLT_STATUS="[OTHER] $LIBMLT_TARGET"
    fi
    echo "  libmlt.so:   $LIBMLT_TARGET -> $LIBMLT_STATUS"
elif [ -e "$LIBMLT_LINK" ]; then
    echo "  libmlt.so:   Present (not a symlink)"
else
    echo "  libmlt.so:   Not present in original flatpak (optional)"
fi

LIBMLT7_LINK="$LIB_DIR/libmlt-7.so"
if [ -L "$LIBMLT7_LINK" ]; then
    LIBMLT7_TARGET="$(readlink "$LIBMLT7_LINK")"
    if [[ "$LIBMLT7_TARGET" == *"$PROJECT_LIBMLT"* || "$LIBMLT7_TARGET" == *"$REPO_DIR"* ]]; then
        LIBMLT7_STATUS="[PROJECT] FastMelt"
    elif [[ "$LIBMLT7_TARGET" == "libmlt-7.so.7"* ]]; then
        LIBMLT7_STATUS="[ORIGINAL] Stock MLT"
    else
        LIBMLT7_STATUS="[CUSTOM] $LIBMLT7_TARGET"
    fi
    echo "  libmlt-7.so: $LIBMLT7_TARGET -> $LIBMLT7_STATUS"
fi

echo "=========================================================="

