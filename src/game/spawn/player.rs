//! Spawn the player.
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bike {
    pub(crate) speed: f32,
    pub(crate) accel: f32,
    pub(crate) top_speed: f32,
}

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    mut camera: Query<Entity, (With<IsDefaultUiCamera>)>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 50000.,
            ..Default::default()
        },
        ..default()
    });

    commands.spawn((
        Name::new("Player"),
        Player,
        SceneBundle {
            scene: asset_server.load("models/chopper_motorbike.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Bike {
            speed: 0.0,
            top_speed: 60.0,
            accel: 30.0,
        },
        StateScoped(Screen::Playing),
        RigidBody::Dynamic,
        Velocity::zero(),
        Sleeping::disabled(),
        Collider::cuboid(1.2, 1.85, 3.5),
        LockedAxes::ROTATION_LOCKED.union(LockedAxes::TRANSLATION_LOCKED_Y),
        KinematicCharacterController {
            snap_to_ground: Some(CharacterLength::Relative(0.5)),
            apply_impulse_to_dynamic_bodies: true,
            ..default()
        },
    )).add_child(camera.get_single().unwrap());
}
