use bevy::{
    camera::visibility::RenderLayers,
    light::NotShadowCaster,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::player::{CameraPivot, Player, spawn_player};

pub const WORLD_LAYER: usize = 0;
pub const VIEW_MODEL_LAYER: usize = 1;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cameras.after(spawn_player))
            .add_systems(Update, (change_fov, cursor_lock));
    }
}

#[derive(Component)]
pub struct WorldModelCamera;

fn spawn_cameras(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for player in &players {
        commands.entity(player).with_children(|parent| {
            parent
                .spawn((
                    CameraPivot,
                    Transform::from_xyz(0.0, 0.65, 0.0),
                    Visibility::default(),
                ))
                .with_children(|pivot| {
                    pivot.spawn((
                        WorldModelCamera,
                        Camera3d::default(),
                        Projection::from(PerspectiveProjection {
                            fov: 90.0_f32.to_radians(),
                            ..default()
                        }),
                        RenderLayers::layer(WORLD_LAYER),
                    ));

                    pivot.spawn((
                        Camera3d::default(),
                        Camera {
                            order: 1,
                            clear_color: ClearColorConfig::None,
                            ..default()
                        },
                        Projection::from(PerspectiveProjection {
                            fov: 70.0_f32.to_radians(),
                            ..default()
                        }),
                        RenderLayers::layer(VIEW_MODEL_LAYER),
                    ));

                    pivot.spawn((
                        Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.5))),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::srgb(0.3, 0.8, 0.7),
                            ..default()
                        })),
                        Transform::from_xyz(0.2, -0.1, -0.25),
                        RenderLayers::layer(VIEW_MODEL_LAYER),
                        NotShadowCaster,
                    ));
                });
        });
    }
}

fn change_fov(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cameras: Query<&mut Projection, With<WorldModelCamera>>,
) {
    let direction =
        i32::from(input.pressed(KeyCode::ArrowDown)) - i32::from(input.pressed(KeyCode::ArrowUp));

    for mut projection in &mut cameras {
        if let Projection::Perspective(perspective) = &mut *projection {
            perspective.fov = (perspective.fov
                + direction as f32 * 60.0_f32.to_radians() * time.delta_secs())
            .clamp(20.0_f32.to_radians(), 160.0_f32.to_radians());
        }
    }
}

fn cursor_lock(
    mut cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }

    if keys.just_pressed(KeyCode::Escape) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }
}
