use crate::prelude::*;

#[derive(Component)]
struct HealthBar;
impl HealthBar {
    const DIMS: Vec2 = Vec2::new(60.0, 4.0);
    const BORDER: f32 = 1.0;
}

fn spawn_health_bar(
    mut commands: Commands,
    meta_state: Res<State<MetaState>>,
    tutorial_root: Res<TutorialRoot>,
    room_root: Res<RoomRoot>,
) {
    let proper_parent = if meta_state.get_tutorial_state().is_some() {
        tutorial_root.eid()
    } else {
        room_root.eid()
    };
    commands
        .spawn((
            Name::new("health_bar"),
            HealthBar,
            multi!([
                (
                    "bg",
                    anim_man!({
                        path: "sprites/default.png",
                        size: (1, 1),
                        color: Color::BLACK,
                    })
                    .with_points(simple_rect(
                        HealthBar::DIMS.x + HealthBar::BORDER * 2.0,
                        HealthBar::DIMS.y + HealthBar::BORDER * 2.0
                    ))
                    .with_offset(-Vec3::Z),
                ),
                (
                    "fg",
                    anim_man!({
                        path: "sprites/default.png",
                        size: (1, 1),
                        color: Color::srgb(0.8, 0.0, 0.0),
                    })
                    .with_points(simple_rect(HealthBar::DIMS.x, HealthBar::DIMS.y)),
                )(
                    "light",
                    anim_man!({
                        path: "sprites/default.png",
                        size: (1, 1),
                    })
                    .with_render_layers(LightCamera::render_layers())
                    .with_points(simple_rect(
                        HealthBar::DIMS.x + HealthBar::BORDER * 2.0,
                        HealthBar::DIMS.y + HealthBar::BORDER * 2.0
                    )),
                )
            ]),
            spat_tran(-119.0, -77.0, ZIX_MAX - 0.1),
        ))
        .set_parent(proper_parent);
}

fn destroy_health_bar(eids: Query<Entity, With<HealthBar>>, mut commands: Commands) {
    for eid in &eids {
        commands.entity(eid).despawn_recursive();
    }
}

fn update_health_bar(
    bird: Query<&Bird>,
    skills: Res<EphemeralSkill>,
    mut multi: Query<&mut MultiAnimationManager, With<HealthBar>>,
    mut commands: Commands,
) {
    let bird = bird.single();
    let mut multi = multi.single_mut();
    let frac_alive = bird.health as f32 / skills.get_max_health() as f32;
    let new_points = simple_rect(HealthBar::DIMS.x * frac_alive, HealthBar::DIMS.y)
        .into_iter()
        .map(|mut p| {
            // First shift so it's centered on my left edge
            p.x += HealthBar::DIMS.x * frac_alive / 2.0;
            // Then shift left to align left edge where expected
            p.x -= HealthBar::DIMS.x / 2.0;
            // Could I combine these? Yes. My brain hurts tho.
            p
        })
        .collect::<Vec<_>>();
    multi
        .manager_mut("fg")
        .set_points(new_points, &mut commands);
}

fn start_dying(
    mut bird: Query<
        (Entity, &mut Bird, &mut MultiAnimationManager),
        (Without<Dying>, Without<Dead>),
    >,
    mut commands: Commands,
) {
    let Ok((eid, mut bird, mut _multi)) = bird.get_single_mut() else {
        return;
    };
    if bird.health == 0 {
        bird.launches_left = 0;
        bird.bullets_left = 0;
        commands.entity(eid).insert(Dying {
            timer: Timer::from_seconds(4.0, TimerMode::Once),
            dont_despawn: true,
        });
    }
}

fn update_dying(
    mut dying_bird: Query<(&Dying, &mut MultiAnimationManager), With<Bird>>,
    mut multis: Query<&mut MultiAnimationManager, Without<Bird>>,
    mut commands: Commands,
    mut bullet_time: ResMut<BulletTime>,
) {
    let Ok((dying_bird, mut multi)) = dying_bird.get_single_mut() else {
        return;
    };
    let frac = dying_bird.timer.fraction_remaining();
    // Holy hack
    multi
        .manager_mut("core")
        .map
        .get_mut("taking_damage")
        .unwrap()
        .fps = (frac - 0.3).max(0.0);
    let color = Color::srgb(frac, frac, frac);
    *bullet_time = BulletTime::Custom((frac * 0.6).powi(2));
    for mut multi in &mut multis {
        for manager in multi.map.values_mut() {
            for anim in manager.map.values_mut() {
                anim.sprite.color = color;
            }
            manager.force_reset(&mut commands);
        }
    }
}

// AHH WHY SO MANY HACKS I AM TIRED
#[derive(Component)]
struct DroppedDead;

fn drop_dead(
    mut dead_bird: Query<
        (Entity, &mut Transform, &mut MultiAnimationManager),
        (With<Bird>, With<Dead>, Without<DroppedDead>),
    >,
    mut commands: Commands,
    mut bullet_time: ResMut<BulletTime>,
    meta_state: Res<State<MetaState>>,
    tutorial_root: Res<TutorialRoot>,
    room_root: Res<RoomRoot>,
    children: Query<&Children>,
) {
    let Ok((eid, mut tran, mut multi)) = dead_bird.get_single_mut() else {
        return;
    };
    multi
        .manager_mut("core")
        .map
        .get_mut("taking_damage")
        .unwrap()
        .fps = 0.0;
    multi.manager_mut("core").force_reset(&mut commands);
    commands.entity(eid).insert(DroppedDead);
    commands.entity(eid).remove::<Stuck>();
    tran.translation.y -= 1.0;
    *bullet_time = BulletTime::Inactive;
    let relevant_root = if meta_state.get_tutorial_state().is_some() {
        tutorial_root.eid()
    } else {
        room_root.eid()
    };
    for id in children.get(relevant_root).unwrap() {
        if *id == eid {
            continue;
        }
        if let Some(commands) = commands.get_entity(*id) {
            commands.despawn_recursive();
        }
    }
    let mut around_room = StickyPlatformBundle::around_room();
    for anim in around_room.multi.map.values_mut() {
        for node in anim.map.values_mut() {
            node.sprite.color = Color::BLACK;
        }
    }
    commands.spawn(around_room).set_parent(relevant_root);
}

fn ghost_up(
    dead_bird: Query<&GlobalTransform, (With<Bird>, With<Dead>, With<DroppedDead>, With<Stuck>)>,
    mut ghost: Query<&mut Transform, With<Ghost>>,
    mut commands: Commands,
    meta_state: Res<State<MetaState>>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
    tutorial_root: Res<TutorialRoot>,
    room_root: Res<RoomRoot>,
) {
    let Ok(gt) = dead_bird.get_single() else {
        return;
    };
    let relevant_root = if meta_state.get_tutorial_state().is_some() {
        tutorial_root.eid()
    } else {
        room_root.eid()
    };
    if ghost.is_empty() {
        let bund = GhostBundle::new(gt.translation().truncate(), true);
        commands.spawn(bund).set_parent(relevant_root);
        return;
    }
    let mut tran = ghost.single_mut();
    tran.translation *= 0.98;
    if tran.translation.length_squared() < 0.5 {
        tran.translation = Vec3::ZERO;
        if meta_state.get_tutorial_state().is_some() {
            next_meta_state.set(TutorialState::Dead.to_meta_state());
        } else {
            panic!("Have to come up with RoomDead state");
        }
    }
}

pub(super) fn register_health_bar(app: &mut App) {
    app.add_systems(OnEnter(BirdAlive::Yes), spawn_health_bar);
    app.add_systems(OnExit(BirdAlive::Yes), destroy_health_bar);
    app.add_systems(Update, update_health_bar.run_if(in_state(BirdAlive::Yes)));
    app.add_systems(
        Update,
        (start_dying, update_dying).run_if(in_state(BirdExists::Yes)),
    );
    app.add_systems(
        Update,
        (ghost_up, drop_dead)
            .chain()
            .run_if(in_state(BirdExists::Yes))
            .run_if(in_state(BirdAlive::No))
            .after(AnimationSet)
            .after(PhysicsSet),
    );
}
