use std::time::Duration;

use bevy::prelude::*;

use crate::{
    audio::RadioAudio,
    components::{self, ClickableArea, ClickableLabel, ClickableShape},
    gamedata::{AmRadioFreq, PresetAmRadioFreq, SceneId},
    input::MousePosition,
    player::{LoadScene, Player, SceneItem},
    right_speaker::RightSpeakerDestroyed,
};

#[derive(Component)]
struct MorseCodeTranslatorSlot;

#[derive(Component)]
struct MorseCodeTranslator {
    timer: Timer,
}

struct PresetButton {
    freq: i32,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl PresetButton {
    pub const fn new(freq: i32, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            freq,
            left,
            top,
            right,
            bottom,
        }
    }
}

const PRESET_BUTTONS: [PresetButton; 4] = [
    PresetButton::new(
        PresetAmRadioFreq::Morse.value(),
        -700.0,
        -30.0,
        -511.0,
        -64.0,
    ),
    PresetButton::new(
        PresetAmRadioFreq::Music.value(),
        -417.0,
        -30.0,
        -226.0,
        -64.0,
    ),
    PresetButton::new(
        PresetAmRadioFreq::News.value(),
        -700.0,
        -155.0,
        -511.0,
        -188.0,
    ),
    PresetButton::new(
        PresetAmRadioFreq::Numbers.value(),
        -417.0,
        -155.0,
        -226.0,
        -188.0,
    ),
];

fn make_translator(asset_server: &Res<AssetServer>) -> Text2dBundle {
    Text2dBundle {
        text: Text::from_sections([TextSection::new(
            "???",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Regular.ttf"),
                font_size: 80.0,
                color: Color::BLACK,
            },
        )])
        .with_justify(JustifyText::Center),
        transform: Transform::from_xyz(-480.0, 120.0, 3.0),
        ..Default::default()
    }
}

fn load_scene(
    mut commands: Commands,
    player: Res<Player>,
    asset_server: Res<AssetServer>,
    mut load_scene: EventReader<LoadScene>,
) {
    for load_scene in load_scene.read() {
        if load_scene.0 == SceneId::Radio {
            if player.has_installed_morse_code_translator {
                commands.spawn((
                    make_translator(&asset_server),
                    MorseCodeTranslator {
                        timer: Timer::new(Duration::from_millis(7000), TimerMode::Repeating),
                    },
                    SceneItem(SceneId::Radio),
                ));
            } else {
                let text = if player.has_installed_morse_code_translator {
                    "Install Module"
                } else {
                    "Module Slot"
                };
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("images/scenes/empty_radio_module.png"),
                        transform: Transform::from_xyz(0.0, 0.0, 3.0),
                        ..Default::default()
                    },
                    ClickableShape::Rectangle(components::Rectangle::from_pos_width_height(
                        Vec2::new(-480.0, 120.0),
                        430.0,
                        120.0,
                    )),
                    ClickableLabel(text),
                    MorseCodeTranslatorSlot,
                    SceneItem(SceneId::Radio),
                ));
            }
            for pb in PRESET_BUTTONS.iter() {
                commands.spawn((
                    ClickableShape::Rectangle(components::Rectangle {
                        top_left: Vec2::new(pb.left, pb.top),
                        bottom_right: Vec2::new(pb.right, pb.bottom),
                    }),
                    AmRadioFreq(pb.freq),
                    ClickableLabel("Button"),
                    SceneItem(SceneId::Radio),
                ));
            }
        }
    }
}

fn update(
    mouse_pos: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    preset_buttons: Query<(&ClickableShape, &AmRadioFreq)>,
    mut radio: Query<&mut AmRadioFreq, (With<RadioAudio>, Without<ClickableShape>)>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for preset_button in preset_buttons.iter() {
        if preset_button.0.contains(mouse_pos.0) {
            if let Ok(mut radio) = radio.get_single_mut() {
                radio.0 = preset_button.1 .0;
            }
        }
    }
}

fn update_morse_code_translator_slot(
    mut commands: Commands,
    mut player: ResMut<Player>,
    mouse_pos: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
    translator: Query<(Entity, &ClickableShape), With<MorseCodeTranslatorSlot>>,
    mut right_speaker_destroyed: EventWriter<RightSpeakerDestroyed>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for translator in translator.iter() {
        if translator.1.contains(mouse_pos.0) {
            player.has_installed_morse_code_translator = true;
            commands.entity(translator.0).despawn();
            if !player.has_installed_surge_protector {
                player.right_speaker_broken = true;
                right_speaker_destroyed.send(RightSpeakerDestroyed);
            }
            commands.spawn((
                make_translator(&asset_server),
                MorseCodeTranslator {
                    timer: Timer::new(Duration::from_millis(7000), TimerMode::Repeating),
                },
                SceneItem(SceneId::Radio),
            ));
        }
    }
}

fn update_morse_code_translator(
    mut translator: Query<(&mut MorseCodeTranslator, &mut Text)>,
    player: Res<Player>,
    time: Res<Time>,
    radio: Query<&AmRadioFreq, With<RadioAudio>>,
) {
    for mut translator in translator.iter_mut() {
        translator.0.timer.tick(time.delta());

        if translator.0.timer.just_finished() {
            for radio in radio.iter() {
                translator.1.sections[0].value = if radio.0 == PresetAmRadioFreq::Morse.value() {
                    if player.right_speaker_broken {
                        String::from("SEE")
                    } else {
                        String::from("BEHIND")
                    }
                } else {
                    String::from("???")
                };
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            load_scene,
            update,
            update_morse_code_translator_slot,
            update_morse_code_translator,
        ),
    );
}
