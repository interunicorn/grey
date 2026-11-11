use crate::camera::{VIEW_MODEL_LAYER, WORLD_LAYER};
use avian3d::prelude::*;
use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{
    bodypart::BodyPartPlugin, camera::CameraPlugin, player::PlayerPlugin, weapon::WeaponPlugin,
};

#[derive(Component)]
pub struct RotatingCube {
    pub health: f32,
}

pub fn run() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            CameraPlugin,
            WeaponPlugin,
            BodyPartPlugin,
            PhysicsPlugins::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (update, check_cube_death))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for i in 0..50 {
        commands.spawn((
            RotatingCube { health: 100.0 },
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.3, 0.6, 0.9),
                ..default()
            })),
            RigidBody::Static,
            Collider::cuboid(1.0, 1.0, 1.0),
            Transform::from_xyz(i as f32, 0.0, i as f32),
            RenderLayers::layer(WORLD_LAYER),
        ));
    }

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(50.0, 0.1, 50.0),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.6, 0.9),
            ..default()
        })),
        Transform::from_xyz(0.0, -1.0, 0.0),
        RenderLayers::layer(WORLD_LAYER),
    ));

    commands.spawn((
        PointLight {
            intensity: 500_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 5.0, 4.0),
        RenderLayers::from_layers(&[WORLD_LAYER, VIEW_MODEL_LAYER]),
    ));
}

fn update(time: Res<Time>, mut spheres: Query<&mut Transform, With<RotatingCube>>) {
    for mut transform in &mut spheres {
        transform.rotate_y(time.delta_secs());
    }
}

fn check_cube_death(mut commands: Commands, cubes: Query<(Entity, &RotatingCube)>) {
    for (entity, cube) in &cubes {
        if cube.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
