#!/usr/bin/env bash
# noir-lsp: Wrapper that tries `aztec lsp` (Docker) first, falls back to `nargo lsp`
#
# Install:
#   cp bin/noir-lsp ~/.local/bin/noir-lsp && chmod +x ~/.local/bin/noir-lsp
#
# Zed settings.json:
#   "lsp": { "nargo": { "binary": { "path": "noir-lsp" } } }

set -euo pipefail

# Safer PATH (optional). Uncomment if you want to restrict binary resolution.
# export PATH="/usr/local/bin:/usr/bin:/bin:${HOME}/.local/bin:${HOME}/.aztec/bin:${HOME}/.nargo/bin"

# Prevent shared library injection
unset LD_PRELOAD LD_LIBRARY_PATH DYLD_INSERT_LIBRARIES DYLD_LIBRARY_PATH 2>/dev/null || true

die() {
  printf '[noir-lsp] Error: %s\n' "$*" >&2
  exit 1
}

has_cmd() {
  command -v "$1" >/dev/null 2>&1
}

validate_bin() {
  local bin="$1"
  [[ -x "$bin" ]] && [[ -f "$bin" || -L "$bin" ]]
}

docker_running() {
  command -v docker >/dev/null 2>&1 || return 1

  # Reject remote Docker daemons — LSP should run locally
  if [[ "${DOCKER_HOST:-}" =~ ^tcp:// ]]; then
    return 1
  fi

  # Timeout prevents hanging if Docker daemon is wedged
  timeout 5 docker info >/dev/null 2>&1
}

# Prefer known absolute installs first to reduce PATH hijack risk
AZTEC_BIN=""
NARGO_BIN=""

if validate_bin "${HOME}/.aztec/bin/aztec"; then
  AZTEC_BIN="${HOME}/.aztec/bin/aztec"
elif has_cmd aztec; then
  AZTEC_BIN="$(command -v aztec)"
fi

if validate_bin "${HOME}/.nargo/bin/nargo"; then
  NARGO_BIN="${HOME}/.nargo/bin/nargo"
elif has_cmd nargo; then
  NARGO_BIN="$(command -v nargo)"
fi

# Try aztec first if Docker is available
if [[ -n "$AZTEC_BIN" ]] && docker_running; then
  exec "$AZTEC_BIN" lsp "$@"
fi

# Fall back to nargo
if [[ -n "$NARGO_BIN" ]]; then
  exec "$NARGO_BIN" lsp "$@"
fi

die "Neither 'aztec' (with Docker available) nor 'nargo' found.
  Install aztec: bash -i <(curl -s https://install.aztec.network)
  Install nargo: https://noir-lang.org/docs/getting_started/installation/"
