use bevy::prelude::*;

use crate::{player::Player, data::TEST_MAP};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Setting things up...");

    let mut cam_transform = Transform::from_xyz(0.0, 10.0, 10.0);
    cam_transform.look_at((0.0, 0.0, 0.0).into(), Vec3::Y);

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: cam_transform,
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.0, 0.7, 0.2).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    })
        .insert(Player);
    
    for (i, tile) in TEST_MAP.iter().enumerate() {
        let x = (i % 16) as f32;
        let z = (i / 16) as f32;

        if tile > &0 {
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.7, 0.5, 0.2).into()),
                transform: Transform::from_xyz(x, -1.0, z),
                ..default()
            });
        }

        match tile {
            2 => {
                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                    material: materials.add(Color::rgb(0.0, 0.9, 0.1).into()),
                    transform: Transform::from_xyz(x, 0.0, z),
                    ..default()
                });
            }
            3 => {
                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.75 })),
                    material: materials.add(Color::rgb(1.0, 0.85, 0.0).into()),
                    transform: Transform::from_xyz(x, 0.0, z),
                    ..default()
                });
            }
            _ => ()
        }
    }
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}
