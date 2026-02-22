// The code found in this file is heavily inspired by
// https://github.com/zed-extensions/lua/blob/main/src/lua.rs
// which is the official Zed Lua language server.
//
// However, since I try to package the language server for pico-8 within the extension,
// the code is slightly different.

/// The following code was created by Claude AI
use zed_extension_api::{self as zed, serde_json, LanguageServerId, Result};

/// PICO-8 Language Server Extension for Zed;
///
/// This extension provides language server support for PICO-8 Lua files (.p8, .lua)
/// using the bundled pico8-ls language server.
///
/// The language server is bundled in the extension at `language-server/main.js`
/// and is run using Node.js.
struct Pico8Extension;

impl zed::Extension for Pico8Extension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // The language server is bundled with the extension at language-server/main.js
        // This path is relative to the extension's root directory
        let server_path = "language-server/main.js".to_string();

        // Get the Node.js binary path from Zed
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

zed::register_extension!(Pico8Extension);
