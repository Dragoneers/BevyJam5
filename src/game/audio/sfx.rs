use bevy::{audio::PlaybackMode, prelude::*};

pub(super) fn play_sfx(
    trigger: Trigger<Sfx>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let event = trigger.event();
    let path = match event {
        Sfx::ButtonHover => "audio/sfx/button_hover.ogg",
        Sfx::ButtonPress => "audio/sfx/button_press.ogg",
        Sfx::Step => "audio/sfx/motorcycle-sound-effects-sfx-179535.ogg",
    };
    let source = asset_server.load::<AudioSource>(path);
    let settings = PlaybackSettings {
        mode: PlaybackMode::Despawn,
        ..default()
    };
    commands.spawn(AudioSourceBundle { source, settings });
}

/// Play a single sound effect.
#[derive(Event)]
pub enum Sfx {
    ButtonHover,
    ButtonPress,
    Step,
}
