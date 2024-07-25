// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

mod audio;
mod background;
mod camera;
mod mouse;
mod pixelate;
mod player;

use audio::audio_plugin;
use background::background_plugin;
use camera::camera_plugin;
use mouse::mouse_plugin;
use player::player_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins((
            camera_plugin,
            mouse_plugin,
            background_plugin,
            audio_plugin,
            player_plugin,
        ))
        .add_plugins(pixelate::PixelatePlugin)
        .run();
}
