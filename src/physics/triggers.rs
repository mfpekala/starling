use crate::prelude::*;

/// Marks an object as being a "trigger" physics object. Should be attached to entities with `Bounds`.
/// This means when other entities containing a `TriggerKind` enter it, each will be notified.
#[derive(Component, Debug, Clone, Reflect)]
pub enum TriggerKind {
    /// Basically marks the hitbox of the protagonist
    Bird,
    /// Any projectile fired by an enemy
    BulletBad,
    /// Any projectile fired by the protagonist
    BulletGood,
    /// Basically marks the hitbox of an enemy
    Enemy,
}
