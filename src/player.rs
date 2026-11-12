use avian3d::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::{camera::visibility::RenderLayers, prelude::*};
use std::f32::consts::FRAC_PI_2;

use crate::{
    bodypart::{BodyPart, BodyPartType},
    camera::WORLD_LAYER,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
}

#[derive(Component)]
pub struct CameraSensitivity(pub Vec2);

#[derive(Component)]
pub struct CameraPivot;

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}

pub(crate) fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Meshes
    let head_mesh = meshes.add(Sphere::new(0.25));
    let torso_mesh = meshes.add(Cuboid::new(0.7, 0.9, 0.35));
    let arm_mesh = meshes.add(Cuboid::new(0.24, 0.8, 0.24));
    let leg_mesh = meshes.add(Cuboid::new(0.3, 0.9, 0.3));

    // Materials
    let skin = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.65, 0.5),
        ..default()
    });

    let shirt = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.4, 0.8),
        ..default()
    });

    let pants = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.25),
        ..default()
    });

    commands.spawn((
        Player { speed: 5.0 },
        RigidBody::Dynamic,
        CameraSensitivity::default(),
        Transform::from_xyz(0.0, 1.0, 5.0),
        Visibility::default(),
        children![
            // Head
            (
                BodyPart::new(BodyPartType::Head, 50.0),
                Mesh3d(head_mesh),
                MeshMaterial3d(skin.clone()),
                Collider::sphere(0.25),
                Transform::from_xyz(0.0, 0.75, 0.0),
                RenderLayers::layer(WORLD_LAYER)
            ),
            // Torso
            (
                BodyPart::new(BodyPartType::Torso, 150.0),
                Mesh3d(torso_mesh),
                MeshMaterial3d(shirt),
                Collider::cuboid(0.35, 0.45, 0.175),
                Transform::from_xyz(0.0, 0.0, 0.0),
                RenderLayers::layer(WORLD_LAYER)
            ),
            // Left arm
            (
                BodyPart::new(BodyPartType::LeftArm, 75.0),
                Mesh3d(arm_mesh.clone()),
                MeshMaterial3d(skin.clone()),
                Collider::cuboid(0.12, 0.4, 0.12),
                Transform::from_xyz(-0.5, -0.05, 0.0),
                RenderLayers::layer(WORLD_LAYER)
            ),
            // Right arm
            (
                BodyPart::new(BodyPartType::RightArm, 75.0),
                Mesh3d(arm_mesh),
                MeshMaterial3d(skin),
                Collider::cuboid(0.12, 0.4, 0.12),
                Transform::from_xyz(0.5, -0.05, 0.0),
                RenderLayers::layer(WORLD_LAYER)
            ),
            // Left leg
            (
                BodyPart::new(BodyPartType::LeftLeg, 100.0),
                Mesh3d(leg_mesh.clone()),
                MeshMaterial3d(pants.clone()),
                Collider::cuboid(0.15, 0.45, 0.15),
                Transform::from_xyz(-0.2, -0.85, 0.0),
                RenderLayers::layer(WORLD_LAYER)
            ),
            // Right leg
            (
                BodyPart::new(BodyPartType::RightLeg, 100.0),
                Mesh3d(leg_mesh),
                MeshMaterial3d(pants),
                Collider::cuboid(0.15, 0.45, 0.15),
                Transform::from_xyz(0.2, -0.85, 0.0),
                RenderLayers::layer(WORLD_LAYER)
            ),
        ],
    ));
}

fn move_player(
    mouse: Res<AccumulatedMouseMotion>,
    mut players: Query<(&mut Transform, &CameraSensitivity, &Player, &Children)>,
    mut pivots: Query<&mut Transform, (With<CameraPivot>, Without<Player>)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::new(0.0, 0.0, 0.0);

    if keys.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    for (mut transform, sensitivity, player, children) in &mut players {
        let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw - mouse.delta.x * sensitivity.0.x;
        for child in children.iter() {
            if let Ok(mut pivot) = pivots.get_mut(child) {
                let (pitch, _, _) = pivot.rotation.to_euler(EulerRot::XYZ);
                let limit = FRAC_PI_2 - 0.01;
                let pitch = (pitch - mouse.delta.y * sensitivity.0.y).clamp(-limit, limit);
                pivot.rotation = Quat::from_rotation_x(pitch);
            }
        }
        let movement = Quat::from_rotation_y(yaw) * direction.normalize_or_zero();
        let mut final_speed = player.speed;
        if keys.pressed(KeyCode::ShiftLeft) && direction.z < 0.0 {
            final_speed *= 2.0;
        }
        // Mouse delta already measures movement since the previous frame.
        transform.rotation = Quat::from_rotation_y(yaw);
        transform.translation += movement * final_speed * time.delta_secs();
    }
}

pub fn get_velocity(query: &Query<&LinearVelocity, With<Player>>) -> Option<LinearVelocity> {
    query.iter().next().copied()
}
