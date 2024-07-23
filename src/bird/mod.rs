use crate::prelude::*;

/// The component that marks the bird entity (protagonist)
/// There should only ever be one of these
#[derive(Component, Debug, Clone, Reflect)]
pub struct Bird;

#[derive(Bundle)]
pub struct BirdBundle {
    name: Name,
    bird: Bird,
    physics: BirdPhysicsBundle,
}
impl BirdBundle {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self {
            name: Name::new("bird"),
            bird: Bird,
            physics: BirdPhysicsBundle::new(pos, vel),
        }
    }
}

fn do_launch(
    mut launch: EventReader<Launch>,
    mut bird_q: Query<(Entity, &mut DynoTran, &mut Transform), With<Bird>>,
    mut commands: Commands,
) {
    let Some(launch) = launch.read().last() else {
        return;
    };
    let Ok((eid, mut dyno_tran, mut tran)) = bird_q.get_single_mut() else {
        return;
    };
    commands.entity(eid).remove::<Stuck>();
    dyno_tran.vel = launch.0 * 10.0;
    tran.set_angle(0.0);
}

pub(super) struct BirdPlugin;
impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (do_launch).after(PhysicsSet));
    }
}
