use crate::prelude::*;

/// Different ways of providing a bird collision hitbox. Admits the design space (TriggerKind x TriggerKind)
#[derive(Debug, Clone, Reflect, PartialEq, Eq, Hash)]
pub enum TriggerKind {
    /// Basically marks the hitbox of the protagonist
    Bird,
    /// Any projectile fired by an enemy
    BulletBad,
    /// Any projectile fired by the protagonist
    BulletGood,
    /// A simp's body
    SimpBody,
    /// Something used for the tutorial
    Tutorial { key: String },
    /// A heart you can pick up between rooms
    Heart,
    /// The thing to shoot to go to the next room
    GoNext,
}

/// Marks an object as being a "triggerable" physics object. Should be attached to entities with `Bounds`.
/// This does not purely a reactionary thing. Collisions happen when it hits other triggers, but neither
/// entity has there velocity/rotation/position updated. Will collide with all other triggers.
#[derive(Component, Debug, Clone, Reflect)]
pub struct TriggerReceiver {
    pub kind: TriggerKind,
    pub collisions: VecDeque<Entity>,
}
impl TriggerReceiver {
    pub fn from_kind(kind: TriggerKind) -> Self {
        Self {
            kind,
            collisions: VecDeque::new(),
        }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct TriggerCollisionRecord {
    pub pos: Vec2,
    pub other_eid: Entity,
    pub other_kind: TriggerKind,
}
#[derive(Bundle)]
pub struct TriggerCollisionBundle {
    name: Name,
    record: TriggerCollisionRecord,
}
impl TriggerCollisionBundle {
    pub fn new(record: TriggerCollisionRecord) -> Self {
        Self {
            name: Name::new("trigger_collision"),
            record,
        }
    }
}
