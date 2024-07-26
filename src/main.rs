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
mod input;
mod mask;
mod pixelate;
mod player;
mod tv;

use audio::audio_plugin;
use background::background_plugin;
use camera::camera_plugin;
use player::player_plugin;
use tv::tv_plugin;

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
            tv_plugin,
            camera_plugin,
            input::plugin,
            background_plugin,
            audio_plugin,
            player_plugin,
        ))
        .add_plugins(pixelate::PixelatePlugin)
        .add_plugins(mask::MaskPlugin)
        .run();
}
