use crate::prelude::*;

/// The physics objects that must be attached to the bird
#[derive(Bundle)]
pub struct BirdPhysicsBundle {
    /// The bird needs to move translationally
    dyno_tran: DynoTran,
    /// The bird is affected by gravity
    gravity: Gravity,
    /// The birds extent in the physical realm
    bounds: Bounds,
    /// The bird should respond normally to statics
    statics: StaticReceiver,
    /// The bird is a trigger
    triggers: TriggerKind,
    /// The bird has to exist spatially
    spatial: SpatialBundle,
}
impl BirdPhysicsBundle {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self {
            dyno_tran: DynoTran { vel },
            gravity: Gravity::Normal,
            bounds: Bounds::from_shape(Shape::Circle { radius: 5.0 }),
            statics: StaticReceiver::from_kind(StaticReceiverKind::Normal),
            triggers: TriggerKind::Bird,
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.extend(ZIX_BIRD),
            )),
        }
    }
}

/// Sticky physics objects that are not translating or rotating
#[derive(Bundle)]
pub struct StickyPhysicsBundle {
    bounds: Bounds,
    statics: StaticProvider,
    spatial: SpatialBundle,
}
impl StickyPhysicsBundle {
    pub fn new(pos: Vec2, bounds: Bounds) -> Self {
        Self {
            bounds,
            statics: StaticProvider::from_kind(StaticProviderKind::Sticky),
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.extend(ZIX_STICKY),
            )),
        }
    }
}

#[derive(Bundle)]
pub struct BulletPhysicsBundle {
    dyno_tran: DynoTran,
    gravity: Gravity,
    bounds: Bounds,
    statics: StaticReceiver,
    triggers: TriggerKind,
    spatial: SpatialBundle,
}
impl BulletPhysicsBundle {
    pub fn new(pos: Vec2, vel: Vec2, good: bool) -> Self {
        Self {
            dyno_tran: DynoTran { vel },
            gravity: Gravity::Normal,
            bounds: Bounds::from_shape(Shape::Circle { radius: 2.0 }),
            statics: StaticReceiver::from_kind(StaticReceiverKind::Stop),
            triggers: if good {
                TriggerKind::BulletGood
            } else {
                TriggerKind::BulletBad
            },
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.extend(ZIX_BULLET),
            )),
        }
    }
}
