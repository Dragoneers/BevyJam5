//! Spawn the player.
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

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

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 1.0));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    commands.spawn((
        Name::new("Player"),
        Player,
        MaterialMeshBundle {
            mesh: arm,
            material: arm_material,
            transform: Transform::from_xyz(0.0, -0.0, -0.0),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
}
