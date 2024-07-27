use crate::prelude::*;

mod markers;

#[derive(Bundle)]
struct BulletBundle {
    name: Name,
    any_bullet: AnyBullet,
    physics: BulletPhysicsBundle,
    multi: MultiAnimationManager,
}
impl BulletBundle {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self {
            name: Name::new("bullet"),
            any_bullet: AnyBullet,
            physics: BulletPhysicsBundle::new(pos, vel, true),
            multi: multi!([
                (
                    "core",
                    anim_man!({
                        solid: {
                            path: "bullets/good.png",
                            size: (5, 5),
                        },
                        explode: {
                            path: "bullets/good_explode.png",
                            size: (7, 7),
                            length: 2,
                            next: "despawn",
                        },
                    }),
                ),
                (
                    "light",
                    anim_man!({
                        path: "bullets/good_light.png",
                        size: (12, 12),
                    })
                    .with_render_layers(LightCamera::render_layers()),
                )
            ]),
        }
    }
}

fn update_bullet_time(
    mut bullet_time: ResMut<BulletTime>,
    bird_q: Query<&Bird>,
    mouse_state: Res<MouseInput>,
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
    let pos = gtran.translation().truncate();
    let vel = fire.0 * 10.0;
    commands
        .spawn(BulletBundle::new(pos, vel))
        .set_parent(parent_eid);
}

fn refresh_launches_n_bullets(
    mut bird_q: Query<(&mut Bird, &StaticReceiver)>,
    static_collisions: Query<&StaticCollisionRecord>,
    skills: Res<EphemeralSkill>,
) {
    for (mut bird, receiver) in bird_q.iter_mut() {
        if receiver
            .collisions
            .clone()
            .into_iter()
            .any(|collision_eid| match static_collisions.get(collision_eid) {
                Ok(record) => record.provider_kind == StaticProviderKind::Sticky,
                Err(_) => false,
            })
        {
            bird.launches_left = skills.get_num_launches();
            bird.bullets_left = skills.get_num_bullets();
        }
    }
}

pub(super) struct DraggingPlugin;
impl Plugin for DraggingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(markers::DragMarkerPlugin);

        app.add_systems(
            Update,
            (
                update_bullet_time,
                do_launch,
                do_fire,
                refresh_launches_n_bullets,
            )
                .run_if(in_state(PhysicsState::Active))
                .run_if(in_state(BirdAlive::Yes))
                .after(PhysicsSet),
        );
    }
}
