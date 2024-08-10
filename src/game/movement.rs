//! Handle player input and translate it into velocity.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/pull/14223).

use std::ops::Mul;
use std::time::Duration;

use super::{audio::sfx::Sfx, spawn::player::*, GameSystem};
use bevy::prelude::*;
use bevy_rapier3d::dynamics::Velocity;

//OLM AYRISINIZ LAN SİZ

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_player_movement_input.in_set(GameSystem::Movement),
            //update_camera,
        )
            .chain(),
    );
}

/// Since Bevy's default 2D camera setup is scaled such that
/// one unit is one pixel, you can think of this as
/// "How many pixels per second should the player move?"
/// Note that physics engines may use different unit/pixel ratios.
const ACCEL: f32 = 50.0;

/// Time between walk sound effects.
const STEP_SFX_INTERVAL: Duration = Duration::from_millis(250);

/// Handle keyboard input to move the player.
fn handle_player_movement_input(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Bike, &mut Velocity), With<Player>>,
    mut camera: Query<&mut Transform, (With<IsDefaultUiCamera>, Without<Player>)>,
    mut last_sfx: Local<Duration>,
    mut commands: Commands,
) {
    let mut intent = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.z -= 1.5;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.z += 0.4;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x += 0.4;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x -= 0.4;
    }

    // Rotation of the object
    if input.just_pressed(KeyCode::KeyA) {
        for mut transform in &mut player_query {
            //transform.0.rotate_z(0.15);
        }

        for mut transform in &mut camera {
            transform.rotate_z(-0.05);
        }
    }
    if input.just_released(KeyCode::KeyA) {
        for mut transform in &mut player_query {
            //transform.0.rotate_z(-0.15);
        }

        for mut transform in &mut camera {
            transform.rotate_z(0.05);
        }
    }

    if input.just_pressed(KeyCode::KeyD) {
        for mut transform in &mut player_query {
            //transform.0.rotate_z(-0.15);
        }

        for mut transform in &mut camera {
            transform.rotate_z(0.05);
        }
    }
    if input.just_released(KeyCode::KeyD) {
        for mut transform in &mut player_query {
            //transform.0.rotate_z(0.15);
        }

        for mut transform in &mut camera {
            transform.rotate_z(-0.05);
        }
    }
    // Need to normalize and scale because otherwise
    // diagonal movement would be faster than horizontal or vertical movement.
    //let intent = intent.normalize_or_zero();
    let mut target_velocity: Vec3 = Vec3::ZERO;
    for (transform, mut bike, mut velocity) in &mut player_query {
        bike.speed += bike.accel * time.delta_seconds() * intent.z;
        bike.speed *= 1.0-bike.drag*time.delta_seconds();
        velocity.angvel = Vec3::ZERO.with_y(intent.x).with_z(-transform.rotation.z*4.0);
        target_velocity = transform.rotation.mul_vec3(intent.with_z(bike.speed));
        velocity.linvel = target_velocity.with_y(-transform.translation.y/40.0);
        println!("{:?}", transform.rotation.z);
    }


    // If the player is moving, play a step sound effect.
    let now = time.elapsed();
    if intent != Vec3::ZERO && *last_sfx + STEP_SFX_INTERVAL < now {
        *last_sfx = now;
        commands.trigger(Sfx::Step);
    }
}

// fn update_camera(
//     mut camera: Query<&mut Transform, (With<IsDefaultUiCamera>, Without<Player>)>,
//     player: Query<&Transform, (With<Player>, Without<Camera3d>)>,
// ) {
//     let Ok(mut camera) = camera.get_single_mut() else {
//         return;
//     };
//
//     let Ok(player) = player.get_single() else {
//         return;
//     };
//
//     let Vec3 { x, y, z } = player.translation;
//     let direction = Vec3::new(x, y + 2.3, z + 1.1);
//
//     camera.translation = direction;
// }
