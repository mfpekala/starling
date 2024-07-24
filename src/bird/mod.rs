use crate::prelude::*;

/// The component that marks the bird entity (protagonist)
/// There should only ever be one of these
#[derive(Component, Debug, Clone, Reflect)]
pub struct Bird {
    launches_left: u32,
    bullets_left: u32,
}
impl Bird {
    pub fn get_launches_left(&self) -> u32 {
        self.launches_left
    }

    pub fn get_bullets_left(&self) -> u32 {
        self.bullets_left
    }
}

#[derive(Bundle)]
pub struct BirdBundle {
    name: Name,
    bird: Bird,
    physics: BirdPhysicsBundle,
    multi: MultiAnimationManager,
}
impl BirdBundle {
    pub fn new(pos: Vec2, vel: Vec2, launches_left: u32, bullets_left: u32) -> Self {
        Self {
            name: Name::new("bird"),
            bird: Bird {
                launches_left,
                bullets_left,
            },
            physics: BirdPhysicsBundle::new(pos, vel),
            multi: MultiAnimationManager::well_lit(
                AnimationManager::single_repeating(
                    SpriteInfo::new("demo/replenish_explode.png", 12, 12),
                    6,
                )
                .with_scale(Vec2::new(5.0, 5.0)),
            ),
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
    let is_launching = bird.launches_left > 0 && mouse_state.get_left_drag_start().is_some();
    let is_firing = bird.bullets_left > 0 && mouse_state.get_right_drag_start().is_some();
    *bullet_time = if is_launching || is_firing {
        BulletTime::Active
    } else {
        BulletTime::Inactive
    };
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
    dyno_tran.vel = launch.0 * 6.0;
    tran.set_angle(0.0);
}

fn do_fire(
    mut fire: EventReader<Fire>,
    mut bird_q: Query<(&mut Bird, &GlobalTransform)>,
    mut commands: Commands,
    meta_state: Res<State<MetaState>>,
    room_root: Res<RoomRoot>,
    tutorial_root: Res<TutorialRoot>,
) {
    let Some(fire) = fire.read().last() else {
        return;
    };
    let Ok((mut bird, gtran)) = bird_q.get_single_mut() else {
        return;
    };
    if bird.bullets_left == 0 {
        return;
    }
    bird.bullets_left -= 1;
    let parent_eid = if meta_state.get_tutorial_state().is_some() {
        tutorial_root.eid()
    } else {
        room_root.eid()
    };
    commands
        .spawn((
            Name::new("bullet"),
            BulletPhysicsBundle::new(gtran.translation().truncate(), fire.0 * 10.0, true),
        ))
        .set_parent(parent_eid);
}

fn refresh_launches_n_bullets(mut bird_q: Query<(&mut Bird, &StaticReceiver)>) {
    for (mut bird, receiver) in bird_q.iter_mut() {
        if receiver
            .collisions
            .clone()
            .into_iter()
            .any(|collision| collision.provider_kind == StaticProviderKind::Sticky)
        {
            bird.launches_left = 3;
            bird.bullets_left = 3;
        }
    }
}

pub(super) struct BirdPlugin;
impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_bullet_time,
                do_launch,
                do_fire,
                refresh_launches_n_bullets,
            )
                .after(PhysicsSet),
        );
    }
}
