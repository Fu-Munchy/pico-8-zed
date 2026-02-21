use zed_extension_api as zed;

struct Pico8Extension {
    // state...
}

impl zed::Extension for Pico8Extension {
    // methods...
}

zed::register_extension!(Pico8Extension);
