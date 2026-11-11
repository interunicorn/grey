use crate::app::RotatingCube;
use crate::camera::WorldModelCamera;
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
) {
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
        let origin = camera.translation() + *direction * 0.5;

        commands.spawn((
            Bullet {
                damage: weapon.damage,
            },
            BulletLifetime(Timer::from_seconds(3.0, TimerMode::Once)),
            RigidBody::Dynamic,
            Collider::sphere(0.02),
            CollisionEventsEnabled,
            Transform::from_translation(origin),
            LinearVelocity(*direction * weapon.bullet_speed),
            Mesh3d(meshes.add(Sphere::new(0.02))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.3, 0.6, 0.9),
                ..default()
            })),
        ));
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
    mut cubes: Query<&mut RotatingCube>,
) {
    for collision in collisions.read() {
        for (bullet_entity, target) in [
            (collision.collider1, collision.collider2),
            (collision.collider2, collision.collider1),
        ] {
            if let Ok(bullet) = bullets.get(bullet_entity)
                && let Ok(mut cube) = cubes.get_mut(target)
            {
                cube.health -= bullet.damage;
                println!("Health: {}", cube.health);
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
