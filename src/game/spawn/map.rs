use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use noiselib::{simplex::simplex_noise_1d, uniform::UniformRandomGen};

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_map);
}

#[derive(Event, Debug)]
pub struct SpawnMap {
    pub seed: u32,
    pub cycle_steps: usize,
}

impl Default for SpawnMap {
    fn default() -> Self {
        Self {
            seed: Default::default(),
            cycle_steps: 1000,
        }
    }
}

impl SpawnMap {
    /// Requests a map generated with a random seed.
    pub fn random() -> Self {
        Self {
            seed: rand::random(),
            ..Default::default()
        }
    }
}

fn spawn_map(spawn_trigger: Trigger<SpawnMap>, mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn((
            Collider::cuboid(100.0, 0.1, 100.0),
            StateScoped(Screen::Playing),
        ))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn((RigidBody::Dynamic, StateScoped(Screen::Playing)))
        .insert(Collider::ball(2.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 16.0, -10.0)));

    let seed = spawn_trigger.event().seed;
    let mut rng = UniformRandomGen::new(seed);

    generate_cycle_walls(
        &mut rng,
        Vec3::new(-10.0, 0.0, 0.0),
        spawn_trigger.event(),
        &mut commands,
    );
    generate_cycle_walls(
        &mut rng,
        Vec3::new(10.0, 0.0, 0.0),
        spawn_trigger.event(),
        &mut commands,
    );
}

fn generate_cycle_walls(
    rng: &mut UniformRandomGen,
    offset: Vec3,
    config: &SpawnMap,
    commands: &mut Commands,
) {
    const WALL_HEIGHT: f32 = 4.0;
    const SEGMENT_LENGTH: f32 = 10.0;
    const NOISE_AMPLITUDE: f32 = 50.0;
    const NOISE_FREQUENCY: f32 = 0.005;
    const WALL_VERTICIES: [Vec3; 2] = [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, WALL_HEIGHT, 0.0)];

    let verticies = (0..=config.cycle_steps).flat_map(move |step_index| {
        let depth_offset = -(step_index as f32) * SEGMENT_LENGTH;

        let horizontal_offset =
            simplex_noise_1d(rng, depth_offset * NOISE_FREQUENCY, config.seed) * NOISE_AMPLITUDE;

        let segment_offset = Vec3::new(horizontal_offset, 0.0, depth_offset) + offset;

        WALL_VERTICIES
            .iter()
            .map(move |vertex| *vertex + segment_offset)
    });

    let index_pattern = [[0, 1, 2], [2, 3, 1]].iter();
    let indices = (0..config.cycle_steps).flat_map(|step_index| {
        index_pattern.clone().map(move |triangle| {
            let mut triangle = triangle.iter();

            let index_offset = (step_index * WALL_VERTICIES.len()) as u32;
            let array: [u32; 3] = std::array::from_fn(|_| triangle.next().unwrap() + index_offset);

            array
        })
    });

    let collider = Collider::trimesh(verticies.collect(), indices.collect());

    commands.spawn((RigidBody::KinematicVelocityBased, collider));
}
