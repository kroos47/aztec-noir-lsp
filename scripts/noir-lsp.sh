#!/usr/bin/env bash
# noir-lsp: Finds nargo and runs `nargo lsp`
#
# Useful when nargo isn't in your editor's PATH but is installed
# via aztec-up at ~/.aztec/current/bin/nargo
#
# Zed settings.json:
#   "lsp": { "nargo": { "binary": { "path": "/path/to/noir-lsp.sh" } } }

set -euo pipefail

# Prevent shared library injection
unset LD_PRELOAD LD_LIBRARY_PATH DYLD_INSERT_LIBRARIES DYLD_LIBRARY_PATH 2>/dev/null || true

readonly NARGO_AZTEC="${HOME}/.aztec/current/bin/nargo"
readonly NARGO_STANDALONE="${HOME}/.nargo/bin/nargo"

die() {
    printf '[noir-lsp] Error: %s\n' "$*" >&2
    exit 1
}

validate_bin() {
    local bin="$1"
    [[ -x "$bin" ]] && [[ -f "$bin" || -L "$bin" ]]
}

# Prefer aztec-installed nargo (version-matched to aztec toolchain)
if validate_bin "$NARGO_AZTEC"; then
    exec "$NARGO_AZTEC" lsp "$@"
fi

# Standalone nargo via noirup
if validate_bin "$NARGO_STANDALONE"; then
    exec "$NARGO_STANDALONE" lsp "$@"
fi

# Fall back to PATH
if command -v nargo >/dev/null 2>&1; then
    exec nargo lsp "$@"
fi

die "nargo not found.
  Install via aztec: bash -i <(curl -s https://install.aztec.network)
  Or standalone: https://noir-lang.org/docs/getting_started/installation/"
