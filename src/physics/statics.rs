use crate::prelude::*;

/// Marks an object as being a "static" physics object. Should be attached to entities with `Bounds`.
/// This means that it DOES NOT respond to collisions with other statics or triggers.
/// BUT it provides collisions for any entity with a `StaticReceiver` component.
#[derive(Component, Debug, Clone, Reflect)]
pub enum StaticKind {
    /// Objects will stick to the outside.
    Sticky,
    /// Objects will bounce off the outside with a fixed friction and bounciness
    /// POTENTIALLY TODO: Add in a bouncy variant
    Normal,
}

/// Marks a component as something that should interact with statics. Should be attached to entities with `Bounds`.
#[derive(Component, Debug, Clone, Reflect)]
pub enum StaticReceiver {
    /// Collides "normally". Will stick to sticky things and bounce off normal things.
    Normal,
    /// No matter what kind of static it hits, it will stop momentum and do nothing else.
    /// It will not stick or bounce. Useful to put on projectiles that should stop and
    /// explode on contact with something static.
    Stop,
}

/// Marks an object that is stuck to a sticky static.
#[derive(Component, Debug, Clone, Reflect)]
pub struct Stuck {
    pub time: f32,
}
