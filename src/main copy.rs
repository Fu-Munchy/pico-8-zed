// The code found in this file is heavily inspired by
// https://github.com/zed-extensions/lua/blob/main/src/lua.rs
// which is the official Zed Lua language server.
//
// However, since I try to package the language server for pico-8 within the extension,
// the code is slightly different.

use std::fs;
use zed_extension_api::{
    self as zed, lsp::CompletionKind, settings::LspSettings, CodeLabel, CodeLabelSpan,
    LanguageServerId, Result,
};


struct Pico8Extension {
    // state...
}

impl zed::Extension for Pico8Extension {
    // methods...
}

zed::register_extension!(Pico8Extension);
