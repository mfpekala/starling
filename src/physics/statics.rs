use crate::prelude::*;

/// Different ways of providing a static collision hitbox. Admits the design space (StaticKind x StaticReceiver)
#[derive(Debug, Clone, Copy, Reflect)]
pub enum StaticProviderKind {
    /// Objects will stick to the outside.
    Sticky,
    /// Objects will bounce off the outside with a fixed friction and bounciness
    /// POTENTIALLY TODO: Add in a bouncy variant
    Normal,
}

/// Marks an object as being a "static" physics object. Should be attached to entities with `Bounds`.
/// This means that it DOES NOT respond to collisions with other statics or triggers.
/// BUT it provides collisions for any entity with a `StaticReceiver` component.
#[derive(Component, Debug, Clone, Reflect)]
pub struct StaticProvider {
    pub kind: StaticProviderKind,
}
impl StaticProvider {
    pub fn from_kind(kind: StaticProviderKind) -> Self {
        Self { kind }
    }
}

/// Different ways of interacting with statics on collision. Admits the design space (StaticKind x StaticReceiver)
#[derive(Debug, Clone, Copy, Reflect)]
pub enum StaticReceiverKind {
    /// Collides "normally". Will stick to sticky things and bounce off normal things.
    Normal,
    /// No matter what kind of static it hits, it will stop momentum and do nothing else.
    /// It will not stick or bounce. Useful to put on projectiles that should stop and
    /// explode on contact with something static.
    Stop,
}

/// A record of a collision happening on this frame. Inserted into both the provider and receiver.
#[derive(Debug, Clone, Reflect)]
pub struct StaticCollisionRecord {
    provider_eid: Entity,
    provider_kind: StaticProviderKind,
    receiver_eid: Entity,
}

/// Marks a component as something that should interact with statics. Should be attached to entities with `Bounds`.
#[derive(Component, Debug, Clone, Reflect)]
pub struct StaticReceiver {
    pub kind: StaticReceiverKind,
}
impl StaticReceiver {
    pub fn from_kind(kind: StaticReceiverKind) -> Self {
        Self { kind }
    }
}

/// Marks an object that is stuck to a sticky static.
#[derive(Component, Debug, Clone, Reflect)]
pub struct Stuck {
    pub parent: Entity,
    pub my_initial_angle: f32,
    pub parent_initial_angle: f32,
    pub initial_offset: Vec2,
}
