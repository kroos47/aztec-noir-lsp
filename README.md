# Aztec Noir — Zed Extension

A [Zed](https://zed.dev) editor extension for [Noir](https://noir-lang.org) language support, built with the [Aztec](https://aztec.network) smart contract development workflow in mind.

> **⚠️ Not yet published.** This extension is not available on the Zed extension marketplace. See [Dev Extension Installation](#dev-extension-installation) below.

## Features

- **Syntax highlighting** — Granular coloring for Noir syntax including differentiation of modules, types (PascalCase), and functions (lowercase) in `use` statements
- **LSP diagnostics** — Inline errors, warnings, hover info, go-to-definition, and completions via `nargo lsp`
- **Code snippets** — Noir basics (`fn`, `struct`, `trait`, `for`, `assert`, etc.) and Aztec-specific boilerplate (`contract`, `#[aztec(private)]`, storage structs, notes, events)
- **Smart brackets & indentation** — Auto-close, auto-indent, and bracket matching for `.nr` files

## Prerequisites

This extension assumes you have the Aztec toolchain installed. The installer sets up both `aztec` (CLI for compile, deploy, etc.) and `nargo` (compiler + LSP):

```bash
bash -i <(curl -s https://install.aztec.network)
```

After installation, nargo is located at `~/.aztec/current/bin/nargo`. The LSP is started via `nargo lsp`.

> **Note:** If you only work on plain Noir (non-Aztec) projects, a standalone `nargo` installation is sufficient: [noir-lang.org/docs/getting_started/installation](https://noir-lang.org/docs/getting_started/installation/)

## Dev Extension Installation

Since this extension isn't published yet, install it as a dev extension:

1. Clone or copy this repo locally:
   ```bash
   git clone https://github.com/your-username/aztec-noir-zed.git
   ```

2. In Zed: **Cmd+Shift+P** → `zed: install dev extension`

3. Select the folder containing `extension.toml` (the root of this repo)

4. Zed will build the WASM extension and load it. You should see `.nr` files get syntax highlighting immediately.

> **Note:** After making changes to the extension source, run **Cmd+Shift+P** → `zed: rebuild dev extension` to pick up updates.

## Zed Settings

Add the following to your Zed settings (`Cmd+,`):

### Option A: Point directly to nargo (simplest)

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
        "path": "/Users/you/.aztec/current/bin/nargo",
        "arguments": ["lsp"]
      }
    }
  }
}
```

Replace the path with the output of `which nargo`.

### Option B: Use the `noir-lsp` wrapper

The bundled `scripts/noir-lsp.sh` wrapper finds nargo in known install locations (`~/.aztec/current/bin/`, `~/.nargo/bin/`, or PATH). Useful if nargo isn't in your editor's PATH:

```json
"lsp": {
  "nargo": {
    "binary": {
      "path": "/absolute/path/to/noir-zed/scripts/noir-lsp.sh"
    }
  }
}
```

Make sure the script is executable: `chmod +x scripts/noir-lsp.sh`

### Option C: No settings (auto-detect)

If you don't configure `lsp.nargo.binary` at all, the extension tries to find nargo in PATH automatically.

`format_on_save` is off because nargo LSP doesn't support formatting yet.

## Troubleshooting

1. **No syntax highlighting** — Rebuild the dev extension (**Cmd+Shift+P** → `zed: rebuild dev extension`). If the grammar fails to build, there may be a WASM compatibility issue with the tree-sitter grammar — open an issue on this repo.

2. **LSP not starting** — Verify `nargo lsp` runs in your terminal. Check that `which nargo` returns a valid path and that it matches what's in your Zed settings.

3. **`LineTooLarge` errors** — This means LSP couldn't load file contents, almost always because the project failed to compile. Fix your `Nargo.toml` dependencies and run `aztec compile` (or `nargo compile`) first.

4. **`Nargo.toml` dependency issues** — Make sure dependencies point to the correct `aztec-packages` monorepo tag:
   ```toml
   [dependencies]
   aztec = { git = "https://github.com/AztecProtocol/aztec-packages/", tag = "v4.0.0-devnet.1-patch.1", directory = "noir-projects/aztec-nr/aztec" }
   ```

5. **LSP stale after `Nargo.toml` change** — Compile first (`aztec compile` or `nargo compile`), then restart LSP via **Cmd+Shift+P** → `zed: restart language server`.

## Acknowledgements

Syntax highlighting in this extension is powered by [tree_sitter_noir](https://github.com/tsujp/tree_sitter_noir), maintained by **@tsujp**. This is by far the most comprehensive tree-sitter grammar available for Noir — actively developed, built directly against the Noir compiler source, and kept in sync with language changes. We are grateful for the effort that has gone into making this resource available to the community.

> **Note:** If the grammar fails to compile for Zed's WASM target, a compatibility patch may be needed. A pre-made patch is available in `grammar-patches/` — if you run into this, please open an issue.

Patterns and inspiration from existing Zed Noir extensions: [zoir](https://github.com/nicholasgasior/zed-noir) by **Hydepwns** and [zed-noir](https://github.com/shuklaayush/zed-noir) by **@shuklaayush**.

Built for the [Aztec](https://aztec.network) and [Noir](https://noir-lang.org) ecosystems.

## License

MIT
