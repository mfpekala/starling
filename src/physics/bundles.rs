use rand::Rng;

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
    static_rx: StaticReceiver,
    /// The bird is a trigger
    trigger_rx: TriggerReceiver,
    /// The bird has to exist spatially
    spatial: SpatialBundle,
}
impl BirdPhysicsBundle {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self {
            dyno_tran: DynoTran { vel },
            gravity: Gravity::Normal,
            bounds: Bounds::from_shape(Shape::Circle { radius: 7.0 }),
            static_rx: StaticReceiver::from_kind(StaticReceiverKind::Normal),
            trigger_rx: TriggerReceiver::from_kind(TriggerKind::Bird),
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
        let mut rng = rand::thread_rng();
        Self {
            bounds,
            statics: StaticProvider::from_kind(StaticProviderKind::Sticky),
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.extend(ZIX_STICKY) + rng.r#gen::<f32>(),
            )),
        }
    }
}

#[derive(Bundle)]
pub struct BulletPhysicsBundle {
    dyno_tran: DynoTran,
    gravity: Gravity,
    bounds: Bounds,
    static_rx: StaticReceiver,
    trigger_rx: TriggerReceiver,
    spatial: SpatialBundle,
}
impl BulletPhysicsBundle {
    pub fn new(pos: Vec2, vel: Vec2, good: bool) -> Self {
        let kind = if good {
            TriggerKind::BulletGood
        } else {
            TriggerKind::BulletBad
        };
        Self {
            dyno_tran: DynoTran { vel },
            gravity: Gravity::Normal,
            bounds: Bounds::from_shape(Shape::Circle { radius: 2.0 }),
            static_rx: StaticReceiver::from_kind(StaticReceiverKind::Stop),
            trigger_rx: TriggerReceiver::from_kind(kind),
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.extend(ZIX_BULLET),
            )),
        }
    }
}

#[derive(Bundle)]
pub struct TutorialTriggerPhysicsBundle {
    pub bounds: Bounds,
    trigger_rx: TriggerReceiver,
    spatial: SpatialBundle,
}
impl TutorialTriggerPhysicsBundle {
    pub fn new(pos: Vec2, radius: f32, key: String) -> Self {
        Self {
            bounds: Bounds::from_shape(Shape::Circle { radius }),
            trigger_rx: TriggerReceiver::from_kind(TriggerKind::Tutorial { key }),
            spatial: spat_tran(pos.x, pos.y, ZIX_TUTORIAL_TRIGGER),
        }
    }
}
