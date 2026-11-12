use crate::bodypart::BodyPart;
use crate::camera::WorldModelCamera;
use crate::player::Player;
use crate::weapon::FireMode::SemiAuto;
use avian3d::prelude::*;
use bevy::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (shoot, despawn_bullets, bullet_hits, switch_firemode),
        )
        .add_systems(Startup, setup);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FireMode {
    SemiAuto,
    Auto,
}

#[derive(Component)]
pub struct Weapon {
    pub damage: f32,
    pub bullet_speed: f32,
    pub fire_mode: FireMode,
}

#[derive(Component)]
struct Bullet {
    pub damage: f32,
}

#[derive(Component)]
struct BulletLifetime(Timer);

#[derive(Component)]
pub struct FireCooldown(pub Timer);

fn setup(mut commands: Commands) {
    let mut timer = Timer::from_seconds(0.1, TimerMode::Once);
    timer.finish();

    commands.spawn((
        Weapon {
            damage: 25.0,
            bullet_speed: 100.0,
            fire_mode: SemiAuto,
        },
        FireCooldown(timer),
    ));
}

fn shoot(
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    camera: Single<&GlobalTransform, With<WorldModelCamera>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut weapons: Query<(&Weapon, &mut FireCooldown)>,
    player_velocity: Single<&LinearVelocity, With<Player>>,
) {
    let player_velocity = player_velocity.into_inner().0;

    for (weapon, mut cooldown) in &mut weapons {
        cooldown.0.tick(time.delta());

        let trigger = match weapon.fire_mode {
            FireMode::SemiAuto => mouse.just_pressed(MouseButton::Left),
            FireMode::Auto => mouse.pressed(MouseButton::Left),
        };

        if !trigger || !cooldown.0.is_finished() {
            continue;
        }

        cooldown.0.reset();

        let direction = camera.forward();
        let rotation = Quat::from_rotation_arc(Vec3::Z, *direction);
        let origin = camera.translation() + *direction * 0.5;

        let bullet = commands
            .spawn((
                Bullet {
                    damage: weapon.damage,
                },
                BulletLifetime(Timer::from_seconds(3.0, TimerMode::Once)),
                RigidBody::Dynamic,
                Collider::sphere(0.02),
                CollisionEventsEnabled,
                LinearVelocity(*direction * weapon.bullet_speed + player_velocity),
                Transform {
                    translation: origin,
                    rotation,
                    ..default()
                },
                // Child mesh transforms are relative to this bullet entity.
                // The hierarchy propagation system needs a GlobalTransform
                // on the parent to compute their world transforms.
                GlobalTransform::default(),
            ))
            .id();

        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.6, 0.9),
            ..default()
        });

        let mesh_rotation = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);

        commands.entity(bullet).with_children(|parent| {
            // Casing, centered on the bullet origin
            parent.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.02, 0.06))),
                MeshMaterial3d(material.clone()),
                Transform::from_rotation(mesh_rotation),
            ));

            // Tip at the front (+Z)
            parent.spawn((
                Mesh3d(meshes.add(Cone::new(0.02, 0.03))),
                MeshMaterial3d(material.clone()),
                Transform {
                    translation: Vec3::new(0.0, 0.0, 0.045),
                    rotation: mesh_rotation,
                    ..default()
                },
            ));
        });
    }
}

fn despawn_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut BulletLifetime)>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.0.tick(time.delta());

        if lifetime.0.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn bullet_hits(
    mut collisions: MessageReader<CollisionStart>,
    bullets: Query<&Bullet>,
    mut body_parts: Query<&mut BodyPart>,
) {
    for collision in collisions.read() {
        for (bullet_entity, target) in [
            (collision.collider1, collision.collider2),
            (collision.collider2, collision.collider1),
        ] {
            if let Ok(bullet) = bullets.get(bullet_entity)
                && let Ok(mut body_part) = body_parts.get_mut(target)
            {
                body_part.take_damage(bullet.damage);
                println!("Health: {}", body_part.health);
            }
        }
    }
}

fn switch_firemode(keys: Res<ButtonInput<KeyCode>>, mut weapons: Query<&mut Weapon>) {
    if keys.just_pressed(KeyCode::KeyB) {
        for mut weapon in &mut weapons {
            weapon.fire_mode = match weapon.fire_mode {
                FireMode::SemiAuto => FireMode::Auto,
                FireMode::Auto => FireMode::SemiAuto,
            };
        }
    }
}
