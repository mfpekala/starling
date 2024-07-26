use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::prelude::*;
mod drag_markers;
pub mod ghost;

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
    face_dyno: FaceDyno,
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
            face_dyno: FaceDyno,
            physics: BirdPhysicsBundle::new(pos, vel),
            multi: multi!([
                (
                    "core",
                    anim_man!({
                        path: "lenny/fly.png",
                        size: (24, 24),
                        length: 3,
                        fps: 16.0,
                    })
                    .with_offset(Vec3::new(-1.0, 0.0, 0.0))
                ),
                (
                    "light",
                    anim_man!({
                        path: "lenny/spotlight.png",
                        size: (48, 48),
                        length: 1,
                    })
                    .with_render_layers(LightCamera::render_layers())
                    .with_scale(Vec2::new(2.5, 2.5))
                ),
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
    commands
        .spawn((
            Name::new("bullet"),
            BulletPhysicsBundle::new(gtran.translation().truncate(), fire.0 * 10.0, true),
        ))
        .set_parent(parent_eid);
}

fn refresh_launches_n_bullets(
    mut bird_q: Query<(&mut Bird, &StaticReceiver)>,
    static_collisions: Query<&StaticCollisionRecord>,
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
            bird.launches_left = 3;
            bird.bullets_left = 3;
        }
    }
}

#[derive(Resource, Reflect)]
struct BirdFlightConsts {
    drag: f32,
    hor_mul: f32,
    down_mul: f32,
    up_mul: f32,
    max_hor_speed: f32,
    max_up_speed: f32,
    max_down_speed: f32,
}
impl Default for BirdFlightConsts {
    fn default() -> Self {
        Self {
            drag: 0.99,
            hor_mul: 125.0,
            down_mul: 100.0,
            up_mul: 800.0,
            max_down_speed: 240.0,
            max_hor_speed: 80.0,
            max_up_speed: 80.0,
        }
    }
}
impl BirdFlightConsts {
    fn apply(&self, dir: Vec2) -> Vec2 {
        let x = dir.x * self.hor_mul;
        let y = if dir.y > 0.0 {
            dir.y * self.up_mul
        } else {
            dir.y * self.down_mul
        };
        Vec2::new(x, y)
    }
}

fn flying(
    mut bird_q: Query<(Entity, &mut DynoTran, &mut Transform), With<Bird>>,
    movement: Res<MovementInput>,
    mut commands: Commands,
    flight_consts: Res<BirdFlightConsts>,
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
) {
    let Ok((eid, mut dyno_tran, mut tran)) = bird_q.get_single_mut() else {
        return;
    };
    let time_factor = time.delta_seconds() * bullet_time.factor();
    let vel_nudge = flight_consts.apply(movement.get_dir()) * time_factor;
    if movement.get_dir().length_squared() > 0.0 {
        tran.set_angle(0.0);
        commands.entity(eid).remove::<Stuck>();
        let new_vel = dyno_tran.vel + vel_nudge;
        if new_vel.x.abs() > dyno_tran.vel.x.abs() {
            // We can only speed up when below the max speed
            if new_vel.x.abs() < flight_consts.max_hor_speed {
                dyno_tran.vel.x = new_vel.x;
            }
        } else {
            // We can always slow down
            dyno_tran.vel.x = new_vel.x;
        }
        // Vertical is a lil different
        if vel_nudge.y > 0.0 {
            if new_vel.y < flight_consts.max_up_speed {
                dyno_tran.vel.y = new_vel.y;
            }
        }
        if vel_nudge.y < 0.0 {
            if new_vel.y > -flight_consts.max_down_speed {
                dyno_tran.vel.y = new_vel.y;
            }
        }
    }
}

pub(super) struct BirdPlugin;
impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BirdFlightConsts::default());
        app.add_plugins(ResourceInspectorPlugin::<BirdFlightConsts>::new());
        app.add_plugins(drag_markers::DragMarkerPlugin);

        app.add_systems(
            Update,
            (
                update_bullet_time,
                do_launch,
                do_fire,
                refresh_launches_n_bullets,
                flying,
            )
                .run_if(in_state(PhysicsState::Active))
                .after(PhysicsSet),
        );
    }
}
