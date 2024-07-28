use std::time::Duration;

use crate::prelude::*;

pub mod bounds;
pub mod bundles;
pub mod collisions;
pub mod dyno;
mod logic;
pub mod statics;
pub mod triggers;

pub use bounds::*;
pub use bundles::*;
pub use collisions::*;
pub use dyno::*;
pub use statics::*;
pub use triggers::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct CorePhysicsSet;

#[derive(Resource, Debug, Copy, Clone, PartialEq)]
pub enum BulletTime {
    Inactive,
    Active,
    Custom(f32),
}
impl BulletTime {
    pub fn factor(&self) -> f32 {
        match self {
            Self::Inactive => 1.0,
            Self::Active => 0.1,
            Self::Custom(val) => *val,
        }
    }
}

#[derive(Component)]
pub struct Birthing;

#[derive(Component)]
pub struct Birthed;

#[derive(Component)]
pub struct Dying {
    pub timer: Timer,
    pub dont_despawn: bool,
}

#[derive(Component)]
pub struct Dead;
#[derive(Component)]
struct DeadDespawn;

/// When there is a transition (from like an OnEnter for example) sometimes physics gets wonky
/// I suspect this has something to do with spawning stuff that all has the same parent, and the transforms not updating
/// before physics happens. This means they "fake" overlap and weird shit happens.
/// Solution is to basically only do physics on stuff that has existed for at least one tick of whatever
/// system physics runs in (rn Update, obviously), marked by this component
#[derive(Component)]
struct InitializedPhysics;

fn reap(
    mut dying_souls: Query<(Entity, &mut Dying, Option<&Bird>)>,
    dead_souls: Query<Entity, With<DeadDespawn>>,
    mut commands: Commands,
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
) {
    for (eid, mut dying, has_bird) in &mut dying_souls {
        let mut time_factor = time.delta_seconds();
        if !has_bird.is_some() {
            time_factor *= bullet_time.factor();
        }
        dying.timer.tick(Duration::from_secs_f32(time_factor));
        if dying.timer.finished() {
            if let Some(mut commands) = commands.get_entity(eid) {
                commands.remove::<Dying>();
                commands.insert(Dead);
                if !dying.dont_despawn {
                    commands.insert(DeadDespawn);
                }
            }
        }
    }
    for soul in &dead_souls {
        if let Some(commands) = commands.get_entity(soul) {
            commands.despawn_recursive();
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
        app.register_type::<StaticCollisionRecord>();
        app.register_type::<TriggerReceiver>();
        app.register_type::<TriggerCollisionRecord>();
        app.register_type::<DynoTran>();
        app.register_type::<DynoRot>();
        app.register_type::<Gravity>();

        // Resources
        app.insert_resource(BulletTime::Inactive);

        // Collisions special
        collisions::register_collisions(app);

        // Logic
        logic::register_logic(app);

        // Reaping dead stuff (idk why i put this in physics)
        app.add_systems(PreUpdate, reap);

        // FaceDyno
        app.add_systems(
            Update,
            dyno::update_face_dynos
                .after(PhysicsSet)
                .run_if(in_state(PhysicsState::Active)),
        );
    }
}
