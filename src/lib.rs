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
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<String> {
        // Check user-configured path from Zed settings first
        if let Some(path) = LspSettings::for_worktree("nargo", worktree)
            .ok()
            .and_then(|s| s.settings)
            .and_then(|s| s.get("path").and_then(|v| v.as_str()).map(String::from))
        {
            return Ok(path);
        }

        let (platform, arch) = zed::current_platform();

        // Windows doesn't have pre-built binaries
        if platform == zed::Os::Windows {
            if let Some(path) = worktree.which("nargo") {
                return Ok(path);
            }
            return Err("Noir does not provide pre-built Windows binaries. \
                Please build nargo from source and add it to your PATH: \
                https://noir-lang.org/docs/getting_started/installation/"
                .to_string());
        }

        // Try `aztec` first (Aztec devnet v3+ toolchain — handles lsp, compile, etc.)
        if let Some(path) = worktree.which("aztec") {
            return Ok(path);
        }

        // Try `nargo` in PATH (plain Noir, or noirup installation)
        if let Some(path) = worktree.which("nargo") {
            return Ok(path);
        }

        // Try cached downloaded binary
        if let Some(path) = self
            .cached_binary_path
            .as_ref()
            .filter(|p| fs::metadata(p).is_ok())
        {
            return Ok(path.clone());
        }

        // Auto-download nargo from GitHub releases as last resort
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "noir-lang/noir",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset_name = match (platform, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "nargo-aarch64-apple-darwin.tar.gz",
            (zed::Os::Mac, zed::Architecture::X8664) => "nargo-x86_64-apple-darwin.tar.gz",
            (zed::Os::Linux, zed::Architecture::Aarch64) => {
                "nargo-aarch64-unknown-linux-gnu.tar.gz"
            }
            (zed::Os::Linux, zed::Architecture::X8664) => "nargo-x86_64-unknown-linux-gnu.tar.gz",
            _ => return Err(format!("unsupported platform: {:?} {:?}", platform, arch)),
        };

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("no asset found for {}", asset_name))?;

        let version_dir = format!("nargo-{}", release.version);
        let binary_path = format!("{}/nargo", version_dir);

        if fs::metadata(&binary_path).is_err() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::GzipTar,
            )
            .map_err(|e| format!("failed to download nargo: {}", e))?;

            zed::make_file_executable(&binary_path)?;

            // Cleanup old versions
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    if name_str.starts_with("nargo-") && name_str != version_dir {
                        fs::remove_dir_all(entry.path()).ok();
                    }
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
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

        let mut args = vec!["lsp".to_string()];

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
