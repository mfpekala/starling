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
            Self::Normal => 400.0,
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

/// Can be placed on components with a DynoTran and a Multi. Will set flip_x automatically.
#[derive(Component)]
pub struct FaceDyno;

pub(super) fn update_face_dynos(
    mut face_dynos: Query<(&mut MultiAnimationManager, &DynoTran), With<FaceDyno>>,
    mut commands: Commands,
) {
    for (mut multi, dyno) in &mut face_dynos {
        if dyno.vel.x.abs() < 0.001 {
            continue;
        }
        for anim in multi.map.values_mut() {
            anim.set_flip_x(dyno.vel.x < 0.0, &mut commands);
        }
    }
}
