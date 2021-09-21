#!/usr/bin/env bash

# Copyright (c) DeFi Blockchain Developers

set -Eeuo pipefail

main() {
    _ensure_script_dir
    trap _cleanup 0 1 2 3 6 15 ERR
    cd "$_SCRIPT_DIR"
    echo "> run pkg_install_deps"
    ../make.sh pkg_install_deps
    echo "> run build_prepare"
    ../make.sh build_prepare
}

_ensure_script_dir() {
    _WORKING_DIR="$(pwd)"
    local dir
    dir="$(dirname "${BASH_SOURCE[0]}")"
    _SCRIPT_DIR="$(cd "${dir}/" && pwd)"
}

_cleanup() {
    cd "$_WORKING_DIR"
}

main "$@"