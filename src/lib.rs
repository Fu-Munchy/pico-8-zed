// All of the following code was created by Claude AI
use zed_extension_api::{self as zed, /*serde_json,*/ LanguageServerId, Result};

/// PICO-8 Language Server Extension for Zed;
///
/// This extension provides language server support for PICO-8 Lua files (.p8, .lua)
/// using the bundled pico8-ls language server.
///
/// The language server is bundled in the extension at `language-server/pico8-ls_v0.6.1.js`
/// and is run using Node.js.
struct Pico8Extension {
    cached_server_path: Option<String>,
}

impl zed::Extension for Pico8Extension {
    /// # Explanation
    /// Creates a new instance of Pico8Extension, and sets the cached_server_path to none.
    /// The server path is cached, such that repeated access of a given path doesn't have to occur.
    /// This makes it simpler to "look up" the necessary server file at its path.
    ///
    /// # Analogy
    /// Instead of opening every door at a hotel floor to find your room, simply note
    /// your room down, and go directly to that room --> save lot of time!
    fn new() -> Self {
        Self {
            cached_server_path: None,
        }
    }
    /// # Explanation
    /// Puts together the methods in impl Pico8Extension {...} to execute
    /// a command which will run the language server
    /// The command looks something like this:
    ///
    ///> ```$ node <path/to/file/> --stdio```
    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // The server_path variable is set to
        let server_path = self.server_path(language_server_id)?;
        let node_path = zed::node_binary_path()?;

        Ok(zed::Command {
            command: node_path,
            args: vec![server_path, "--stdio".to_string()],
            env: worktree.shell_env(),
        })
    }

    // Generates a configuration file which is checked when the server is initialized
    // This allows for the configuration of the server. Added by Claude, but not used.
    // I believe this can be removed - but will keep this still in case there's a use
    // in the future

    // fn language_server_initialization_options(
    //     &mut self,
    //     _language_server_id: &LanguageServerId,
    //     _worktree: &zed::Worktree,
    // ) -> Result<Option<serde_json::Value>> {
    //     Ok(Some(serde_json::json!({})))
    // }

    // Similar to the method above, but generates a JSON configuration file for the server
    // which is checked periodically. However, I am not sure how this is useful - was added by
    // Claude, and is therefore commented out.

    // fn language_server_workspace_configuration(
    //     &mut self,
    //     _language_server_id: &LanguageServerId,
    //     _worktree: &zed::Worktree,
    // ) -> Result<Option<serde_json::Value>> {
    //     Ok(Some(serde_json::json!({
    //         "pico8-ls": {}
    //     })))
    // }
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

        // Get the latest release from the GitHub repo
        let release = zed::latest_github_release(
            "Fu-Munchy/pico-8-zed",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        // Create version-specific directory and path
        let version_dir = format!("pico8-ls-{}", release.version);
        let relative_server_path = format!("{}/pico8-ls_v0.6.1.js", version_dir);

        // Get the extension's working directory for absolute path
        let current_working_directory = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        let server_path = current_working_directory
            .join(&version_dir)
            .join("pico8-ls_v0.6.1.js")
            .to_string_lossy()
            .to_string();

        // Check if the pico8-ls_v0.6.1.js file has been downloaded before.
        // The map_or() method defaults to false on Err and appiles m.is_file() if m is a file.
        // Is equivalent to:
        // match std::fs::metadata(path) {
        //     Ok(m) => m.is_file(),
        //     Err(_) => false,
        // }
        if !std::fs::metadata(&server_path).map_or(false, |m| m.is_file()) {
            // Updates the installation status to Downloading
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            // Create the directory and all of its
            // parent directories (if missing), if &version_dir does not exist.
            // Return the error "Failed to create directory" if the directory could not be created
            std::fs::create_dir_all(&version_dir)
                .map_err(|e| format!("Failed to create directory: {}", e))?;

            // Find the pico8-ls_v0.6.1.js asset in the releases page of the github page
            let asset = release
                .assets
                .iter()
                .find(|a| a.name == "pico8-ls_v0.6.1.js")
                .ok_or_else(|| "No pico8-ls_v0.6.1.js asset found in release".to_string())?;

            // Download the file and save it to the relative_server_path within the extension's directory
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
