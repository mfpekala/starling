use crate::prelude::*;

/// The component that marks the bird entity (protagonist)
/// There should only ever be one of these
#[derive(Component, Debug, Clone, Reflect)]
pub struct Bird {
    launches_left: u32,
}
impl Bird {
    pub fn get_launches_left(&self) -> u32 {
        self.launches_left
    }
}

#[derive(Bundle)]
pub struct BirdBundle {
    name: Name,
    bird: Bird,
    physics: BirdPhysicsBundle,
}
impl BirdBundle {
    pub fn new(pos: Vec2, vel: Vec2, launches_left: u32) -> Self {
        Self {
            name: Name::new("bird"),
            bird: Bird { launches_left },
            physics: BirdPhysicsBundle::new(pos, vel),
        }
    }
}

fn update_bullet_time(
    mut bullet_time: ResMut<BulletTime>,
    bird_q: Query<&Bird>,
    mouse_state: Res<MouseState>,
) {
    let Ok(bird) = bird_q.get_single() else {
        // No bird = no bullet time
        *bullet_time = BulletTime::Inactive;
        return;
    };
    if bird.launches_left <= 0 {
        // No launches = no bullet time
        *bullet_time = BulletTime::Inactive;
        return;
    }
    if mouse_state.get_left_drag_start().is_none() {
        // No drag = no bullet time
        *bullet_time = BulletTime::Inactive;
        return;
    }
    *bullet_time = BulletTime::Active;
}

fn do_launch(
    mut launch: EventReader<Launch>,
    mut bird_q: Query<(Entity, &mut Bird, &mut DynoTran, &mut Transform)>,
    mut commands: Commands,
) {
    let Some(launch) = launch.read().last() else {
        return;
    };
    let Ok((eid, mut bird, mut dyno_tran, mut tran)) = bird_q.get_single_mut() else {
        return;
    };
    if bird.launches_left == 0 {
        return;
    }
    bird.launches_left -= 1;
    commands.entity(eid).remove::<Stuck>();
    dyno_tran.vel = launch.0 * 10.0;
    tran.set_angle(0.0);
}

fn refresh_launches(mut bird_q: Query<(&mut Bird, &StaticReceiver)>) {
    for (mut bird, receiver) in bird_q.iter_mut() {
        if receiver
            .collisions
            .clone()
            .into_iter()
            .any(|collision| collision.provider_kind == StaticProviderKind::Sticky)
        {
            bird.launches_left = 3;
        }
    }
}

pub(super) struct BirdPlugin;
impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_bullet_time, do_launch, refresh_launches).after(PhysicsSet),
        );
    }
}
