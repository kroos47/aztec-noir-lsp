# Aztec Noir — Zed Extension

A [Zed](https://zed.dev) editor extension for [Noir](https://noir-lang.org) language support, built with the [Aztec](https://aztec.network) smart contract development workflow in mind.

> **⚠️ Not yet published.** This extension is not available on the Zed extension marketplace. See [Dev Extension Installation](#dev-extension-installation) below.

## Features

- **Syntax highlighting** — Granular coloring for Noir syntax including differentiation of modules, types (PascalCase), and functions (lowercase) in `use` statements
- **LSP diagnostics** — Inline errors, warnings, hover info, go-to-definition, and completions via `nargo lsp` or `aztec lsp`
- **Code snippets** — Noir basics (`fn`, `struct`, `trait`, `for`, `assert`, etc.) and Aztec-specific boilerplate (`contract`, `#[aztec(private)]`, storage structs, notes, events)
- **Docker-aware LSP wrapper** — Bundled `noir-lsp` script tries `aztec lsp` (Docker) first, falls back to `nargo lsp` automatically

## Prerequisites

This extension assumes you have at least one of the following installed:

| Tool | What it's for | Install |
|------|--------------|---------|
| **aztec** | Aztec.nr contract development (requires Docker) | `bash -i <(curl -s https://install.aztec.network)` |
| **nargo** | Plain Noir development | [noir-lang.org/docs/getting_started/installation](https://noir-lang.org/docs/getting_started/installation/) |
| **Docker** | Required by `aztec lsp` — the aztec binary runs its toolchain inside Docker | [docs.docker.com/get-docker](https://docs.docker.com/get-docker/) |

If you're working on **Aztec.nr contracts**, you need `aztec` + Docker running. For **plain Noir** projects, `nargo` alone is enough.

## Dev Extension Installation

Since this extension isn't published yet, install it as a dev extension:

1. Clone or copy this repo locally:
   ```bash
   git clone https://github.com/your-username/aztec-noir-zed.git
   # or just keep it wherever you have it, e.g. ~/KROOS/noir-zed
   ```

2. In Zed: **Cmd+Shift+P** → `zed: install dev extension`

3. Select the folder containing `extension.toml` (the root of this repo)

4. Zed will build the WASM extension and load it. You should see `.nr` files get syntax highlighting immediately.

> **Note:** After making changes to the extension source, run **Cmd+Shift+P** → `zed: rebuild dev extension` to pick up updates.

## Zed Settings

Add the following to your Zed settings (`Cmd+,`):

### Using the `noir-lsp` wrapper (recommended)

The bundled `scripts/noir-lsp.sh` wrapper automatically tries `aztec lsp` when Docker is running and falls back to `nargo lsp` when it's not. Point Zed to it with an absolute path:

```json
{
  "languages": {
    "Noir": {
      "tab_size": 4,
      "hard_tabs": false,
      "format_on_save": "off"
    }
  },
  "lsp": {
    "nargo": {
      "binary": {
        "path": "/absolute/path/to/noir-zed/scripts/noir-lsp.sh"
      }
    }
  }
}
```

Make sure the script is executable:
```bash
chmod +x scripts/noir-lsp.sh
```

`format_on_save` is off because nargo LSP doesn't support formatting yet.

### Pointing directly to a binary

If you prefer to skip the wrapper:

```json
"lsp": {
  "nargo": {
    "binary": {
      "path": "/Users/you/.aztec/bin/aztec",
      "arguments": ["lsp"]
    }
  }
}
```

Replace the path with the output of `which aztec` or `which nargo`.

### No settings (auto-detect)

If you don't configure `lsp.nargo.binary` at all, the extension tries to find a binary in this order:
1. `noir-lsp` wrapper in PATH
2. `aztec` in PATH (needs Docker)
3. `nargo` in PATH

## A Note on Docker

The `aztec` binary is a shell script that runs the Aztec toolchain inside a Docker container. This means:

- Docker Desktop (or Docker Engine) must be **running** for `aztec lsp` to work
- First launch may take a few seconds while the container starts
- If Docker is not running, the `noir-lsp.sh` wrapper automatically falls back to `nargo lsp`
- The wrapper also rejects remote Docker daemons (`DOCKER_HOST=tcp://...`) for security — LSP should run locally

If you see LSP errors, check `docker info` first.

## Troubleshooting

1. **No syntax highlighting** — Rebuild the dev extension (**Cmd+Shift+P** → `zed: rebuild dev extension`). If the grammar fails to build, there may be a WASM compatibility issue with the tree-sitter grammar — open an issue on this repo.

2. **LSP not starting** — Check that Docker is running (`docker info`) if using `aztec`. Verify `aztec compile` or `nargo compile` succeeds in your project directory — LSP won't work if the project can't compile.

> **Note:** If the grammar fails to compile for Zed's WASM target, a compatibility patch may be needed. Raise an issue if it happens so an appropriate patch can be applied. 

## Acknowledgements

Syntax highlighting in this extension is powered by [tree_sitter_noir](https://github.com/tsujp/tree_sitter_noir) grammar by [**@tsujp**](https://github.com/tsujp). 
This is by far the most comprehensive tree-sitter grammar available for Noir — actively developed, built directly against the Noir compiler source, and kept in sync with language changes. We are grateful for the effort that has gone into making this resource available to the community.

Built for the [Aztec](https://aztec.network) and [Noir](https://noir-lang.org) ecosystems.
