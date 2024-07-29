use crate::prelude::*;

pub mod simp;
pub mod spawner;
pub mod spew;

pub use simp::*;
pub use spawner::*;
pub use spew::*;

pub trait EnemyBundle: Bundle {
    /// A queryable component to tell us how many of these enemies are alive
    /// Useful so we can force a spawner to start spawning if all enemies of a given type are dead
    type CountComponent: Component;

    fn spawn(pos: Vec2, commands: &mut Commands, parent: Entity);
}

pub(super) struct EnemiesPlugin;
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        simp::register_simps(app);
        spawner::register_spawners(app);
        spew::register_spews(app);
    }
}
