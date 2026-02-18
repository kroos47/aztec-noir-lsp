use std::fs;
use zed_extension_api::{
    self as zed, serde_json, settings::LspSettings, Command, LanguageServerId, Result, Worktree,
};

struct AztecNoirExtension {
    cached_binary_path: Option<String>,
}

impl AztecNoirExtension {
    fn language_server_binary_path(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<String> {
        // 1. Check user-configured path from Zed settings
        if let Some(path) = LspSettings::for_worktree("nargo", worktree)
            .ok()
            .and_then(|s| s.settings)
            .and_then(|s| s.get("path").and_then(|v| v.as_str()).map(String::from))
        {
            return Ok(path);
        }

        let (platform, _) = zed::current_platform();

        if platform == zed::Os::Windows {
            if let Some(path) = worktree.which("nargo") {
                return Ok(path);
            }
            return Err("Noir does not provide pre-built Windows binaries. \
                Please build nargo from source and add it to your PATH."
                .to_string());
        }

        // 2. Try `noir-lsp` wrapper (handles aztec→nargo fallback with Docker check)
        if let Some(path) = worktree.which("noir-lsp") {
            return Ok(path);
        }

        // 3. Try `aztec` (needs Docker running)
        if let Some(path) = worktree.which("aztec") {
            return Ok(path);
        }

        // 4. Try `nargo` in PATH
        if let Some(path) = worktree.which("nargo") {
            return Ok(path);
        }

        // 5. Try cached downloaded binary
        if let Some(path) = self
            .cached_binary_path
            .as_ref()
            .filter(|p| fs::metadata(p).is_ok())
        {
            return Ok(path.clone());
        }

        Err("Could not find noir-lsp, aztec, or nargo in PATH. \
            Install aztec: bash -i <(curl -s https://install.aztec.network) \
            Or install nargo: https://noir-lang.org/docs/getting_started/installation/"
            .to_string())
    }
}

impl zed::Extension for AztecNoirExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let binary_path = self.language_server_binary_path(language_server_id, worktree)?;

        let settings = LspSettings::for_worktree("nargo", worktree)
            .ok()
            .and_then(|s| s.settings);

        // The wrapper script (noir-lsp) handles `lsp` internally,
        // but aztec and nargo need the `lsp` subcommand.
        let is_wrapper = binary_path.ends_with("noir-lsp");

        let mut args = if is_wrapper {
            vec![]
        } else {
            vec!["lsp".to_string()]
        };

        // Add any custom arguments from settings
        let extra_args = settings
            .as_ref()
            .and_then(|s| s.get("args"))
            .and_then(|v| v.as_array())
            .into_iter()
            .flatten()
            .filter_map(|v| v.as_str())
            .map(String::from);

        args.extend(extra_args);

        Ok(Command {
            command: binary_path,
            args,
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("nargo", worktree)
            .ok()
            .and_then(|s| s.initialization_options);
        Ok(settings)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("nargo", worktree)
            .ok()
            .and_then(|s| s.settings);
        Ok(settings)
    }
}

zed::register_extension!(AztecNoirExtension);
