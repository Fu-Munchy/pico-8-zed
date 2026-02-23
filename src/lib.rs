/// All of the following code was created by Claude AI
use zed_extension_api::{self as zed, serde_json, LanguageServerId, Result};

/// PICO-8 Language Server Extension for Zed;
///
/// This extension provides language server support for PICO-8 Lua files (.p8, .lua)
/// using the bundled pico8-ls language server.
///
/// The language server is bundled in the extension at `language-server/main.js`
/// and is run using Node.js.
struct Pico8Extension {
    cached_server_path: Option<String>,
}

impl zed::Extension for Pico8Extension {
    fn new() -> Self {
        Self {
            cached_server_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_path(language_server_id)?;
        let node_path = zed::node_binary_path()?;

        Ok(zed::Command {
            command: node_path,
            args: vec![server_path, "--stdio".to_string()],
            env: worktree.shell_env(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(serde_json::json!({})))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(serde_json::json!({
            "pico8-ls": {}
        })))
    }
}

impl Pico8Extension {
    fn server_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        // Check if we already have the server cached
        if let Some(path) = &self.cached_server_path {
            if std::fs::metadata(path).map_or(false, |m| m.is_file()) {
                return Ok(path.clone());
            }
        }

        // Update status: checking for updates
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        // Get the latest release from your GitHub repo
        let release = zed::latest_github_release(
            "Fu-Munchy/pico-8-zed",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        // Create version-specific directory and path
        let version_dir = format!("pico8-ls-{}", release.version);
        let relative_server_path = format!("{}/main.js", version_dir);

        // Get the extension's working directory for absolute path
        let cwd = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        let server_path = cwd
            .join(&version_dir)
            .join("main.js")
            .to_string_lossy()
            .to_string();

        // Check if already downloaded
        if !std::fs::metadata(&server_path).map_or(false, |m| m.is_file()) {
            // Update status: downloading
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            // Create the directory if it doesn't exist
            std::fs::create_dir_all(&version_dir)
                .map_err(|e| format!("Failed to create directory: {}", e))?;

            // Find the main.js asset in the release
            let asset = release
                .assets
                .iter()
                .find(|a| a.name == "main.js")
                .ok_or_else(|| "No main.js asset found in release".to_string())?;

            // Download the file (use relative path for download)
            zed::download_file(
                &asset.download_url,
                &relative_server_path,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|e| format!("Failed to download language server: {}", e))?;
        }

        // Cache the absolute path for future calls
        self.cached_server_path = Some(server_path.clone());
        Ok(server_path)
    }
}
zed::register_extension!(Pico8Extension);
