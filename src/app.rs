use crate::camera::{VIEW_MODEL_LAYER, WORLD_LAYER};
use avian3d::prelude::*;
use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{
    bodypart::{BodyPart, BodyPartPlugin, BodyPartType},
    camera::CameraPlugin,
    player::PlayerPlugin,
    weapon::WeaponPlugin,
};

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
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    let skin = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.65, 0.5),
        ..default()
    });

    let head_mesh = meshes.add(Sphere::new(0.25));
    commands.spawn((
        // Head
        BodyPart::new(BodyPartType::Head, 50.0),
        Mesh3d(head_mesh),
        MeshMaterial3d(skin.clone()),
        RigidBody::Dynamic,
        Collider::sphere(0.25),
        Transform::from_xyz(0.0, 0.75, 0.0),
        RenderLayers::layer(WORLD_LAYER),
    ));
}
