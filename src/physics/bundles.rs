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
            bounds: Bounds::from_shape(Shape::Circle { radius: 10.0 }),
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
pub struct StickyPlainPhysicsBundle {
    bounds: Bounds,
    statics: StaticProvider,
    spatial: SpatialBundle,
}
impl StickyPlainPhysicsBundle {
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

/// Sticky physics objects that are moving translating but not rotating
#[derive(Bundle)]
pub struct StickyTranPhysicsBundle {
    bounds: Bounds,
    statics: StaticProvider,
    spatial: SpatialBundle,
    dyno_tran: DynoTran,
}
impl StickyTranPhysicsBundle {
    pub fn new(pos: Vec2, bounds: Bounds, dyno_tran: DynoTran) -> Self {
        Self {
            bounds,
            statics: StaticProvider::from_kind(StaticProviderKind::Sticky),
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.extend(ZIX_STICKY),
            )),
            dyno_tran,
        }
    }
}

/// Sticky physics objects that are rotating but not translating
#[derive(Bundle)]
pub struct StickyRotPhysicsBundle {
    bounds: Bounds,
    statics: StaticProvider,
    spatial: SpatialBundle,
    dyno_rot: DynoRot,
}
impl StickyRotPhysicsBundle {
    pub fn new(pos: Vec2, bounds: Bounds, dyno_rot: DynoRot) -> Self {
        Self {
            bounds,
            statics: StaticProvider::from_kind(StaticProviderKind::Sticky),
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.extend(ZIX_STICKY),
            )),
            dyno_rot,
        }
    }
}
