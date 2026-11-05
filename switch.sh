#!/usr/bin/env bash
# FastMelt Switcher for Kdenlive Flatpak

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

usage() {
    echo "FastMelt Kdenlive Flatpak Switcher"
    echo "Usage: $0 [command] [options]"
    echo
    echo "Commands:"
    echo "  project     Switch Flatpak to use FastMelt (melt and libmlt.so)"
    echo "  original    Restore Flatpak to original/stock MLT"
    echo "  status      Show current configuration and link targets"
    echo
    echo "Options:"
    echo "  --include-so7   Also redirect libmlt-7.so.7 (for timeline ABI testing)"
    echo
    echo "Run without arguments for interactive selection."
    exit 0
}

COMMAND="${1:-}"

if [ -z "$COMMAND" ]; then
    bash "$SCRIPT_DIR/scripts/status.sh"
    echo
    echo "Choose an option:"
    echo "  1) Use FastMelt (Project Binaries)"
    echo "  2) Use Original Stock MLT"
    echo "  3) Show Status"
    echo "  4) Exit"
    echo
    read -rp "Enter choice [1-4]: " choice
    case "$choice" in
        1)
            bash "$SCRIPT_DIR/scripts/switch-to-project.sh"
            ;;
        2)
            bash "$SCRIPT_DIR/scripts/switch-to-original.sh"
            ;;
        3)
            bash "$SCRIPT_DIR/scripts/status.sh"
            ;;
        *)
            echo "Exiting."
            exit 0
            ;;
    esac
else
    case "$COMMAND" in
        project|custom|fastmelt|olive)
            shift
            bash "$SCRIPT_DIR/scripts/switch-to-project.sh" "$@"
            ;;
        original|stock|restore)
            shift
            bash "$SCRIPT_DIR/scripts/switch-to-original.sh" "$@"
            ;;
        status|info)
            bash "$SCRIPT_DIR/scripts/status.sh"
            ;;
        help|-h|--help)
            usage
            ;;
        *)
            echo "Unknown command: $COMMAND" >&2
            echo "Run '$0 --help' for usage." >&2
            exit 1
            ;;
    esac
fi

