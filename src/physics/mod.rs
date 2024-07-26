use crate::prelude::*;

pub mod bounds;
pub mod bundles;
pub mod collision;
pub mod dyno;
mod logic;
pub mod statics;
pub mod triggers;

pub use bounds::*;
pub use bundles::*;
pub use collision::*;
pub use dyno::*;
pub use statics::*;
pub use triggers::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSet;

#[derive(Resource, Debug, Copy, Clone)]
pub enum BulletTime {
    Inactive,
    Active,
}
impl BulletTime {
    pub fn factor(&self) -> f32 {
        match self {
            Self::Inactive => 1.0,
            Self::Active => 0.1,
        }
    }
}

pub(super) struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Register the types so we get better debug info in the inspector
        app.register_type::<Bounds>();
        app.register_type::<Inactive>();
        app.register_type::<StaticProvider>();
        app.register_type::<StaticReceiver>();
        app.register_type::<TriggerReceiver>();
        app.register_type::<DynoTran>();
        app.register_type::<DynoRot>();
        app.register_type::<Gravity>();

        // Resources
        app.insert_resource(BulletTime::Inactive);

        // Logic
        logic::register_logic(app);
    }
}
