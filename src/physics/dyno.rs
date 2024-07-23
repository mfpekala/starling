use crate::prelude::*;

/// Marks something that should be affected by gravity.
/// NOTE: Must exist on an entity with DynoTran
#[derive(Component, Debug, Clone, Reflect)]
pub enum Gravity {
    Normal,
}
impl Gravity {
    pub fn strength(&self) -> f32 {
        match self {
            Self::Normal => 300.0,
        }
    }
}

/// Anything that needs to move translationally in the world. Can be either triggers or statics.
#[derive(Component, Debug, Clone, Reflect)]
pub struct DynoTran {
    pub vel: Vec2,
}

/// Anything that needs to move rotationally in the world. Can be either triggers or statics.
#[derive(Component, Debug, Clone, Reflect)]
pub struct DynoRot {
    pub rot: f32,
}
