use bevy::prelude::*;

pub struct BodyPartPlugin;

impl Plugin for BodyPartPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageBodyPart>().add_systems(
            Update,
            (
                apply_body_part_damage,
                check_body_parts,
                check_destroyed_body_parts,
            ),
        );
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BodyPartType {
    Head,
    Torso,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

#[derive(Component)]
pub struct BodyPart {
    pub part_type: BodyPartType,
    pub max_health: f32,
    pub health: f32,
}

impl BodyPart {
    pub fn new(part_type: BodyPartType, max_health: f32) -> Self {
        Self {
            part_type,
            max_health,
            health: max_health,
        }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount.max(0.0)).max(0.0);
    }

    pub fn is_destroyed(&self) -> bool {
        self.health <= 0.0
    }
}

#[derive(Message)]
pub struct DamageBodyPart {
    pub target: Entity,
    pub damage: f32,
}

#[derive(Component)]
pub struct BodyPartDestroyed;

fn apply_body_part_damage(
    mut messages: MessageReader<DamageBodyPart>,
    mut body_parts: Query<&mut BodyPart>,
) {
    for message in messages.read() {
        if let Ok(mut body_part) = body_parts.get_mut(message.target) {
            body_part.take_damage(message.damage);
        }
    }
}

fn check_body_parts(
    body_parts: Query<(Entity, &BodyPart), Without<BodyPartDestroyed>>,
    mut commands: Commands,
) {
    for (entity, part) in &body_parts {
        if part.is_destroyed() {
            commands.entity(entity).insert(BodyPartDestroyed);
        }
    }
}

fn check_destroyed_body_parts(
    mut commands: Commands,
    destroyed_body_parts: Query<(Entity, &BodyPartDestroyed)>,
) {
    for (entity, part) in &destroyed_body_parts {
        commands.entity(entity).despawn();
    }
}
