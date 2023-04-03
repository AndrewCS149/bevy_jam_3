use bevy::prelude::*;
use rand::Rng;

use crate::game::{
    game_cmps::{Hp, Speed},
    world::MAP_SIZE,
};

use super::{enemy_cmps::Enemy, enemy_res::EnemySpawnTimer, ENEMY_HP, ENEMY_SPEED};

pub fn spawn_enemies(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    spawn_timer.0.tick(time.delta());

    let mut rng = rand::thread_rng();

    let map_bounds = MAP_SIZE / 2.0;
    let x = rng.gen_range(-map_bounds..=map_bounds);
    let z = rng.gen_range(-map_bounds..=map_bounds);

    if spawn_timer.0.finished() {
        cmds.spawn((
            PbrBundle {
                material: materials.add(Color::RED.into()),
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.25,
                    depth: 0.25,
                    ..default()
                })),
                transform: Transform::from_xyz(x, 0.3, z),
                ..default()
            },
            Enemy,
            Speed(ENEMY_SPEED),
            Hp(ENEMY_HP),
            Name::new("Enemy"),
        ));
    }
}