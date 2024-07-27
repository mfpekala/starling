use crate::prelude::*;

pub mod simp;

pub use simp::*;

pub(super) struct EnemiesPlugin;
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        simp::register_simps(app);
    }
}
