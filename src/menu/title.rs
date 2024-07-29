use crate::prelude::*;

#[derive(Bundle)]
struct FakeBirdBundle {
    name: Name,
    bird: FakeBird,
    face_dyno: FaceDyno,
    physics: FakeBirdPhysicsBundle,
    multi: MultiAnimationManager,
    particles: DynoAwareParticleSpawner,
}
impl FakeBirdBundle {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self {
            name: Name::new("fake_bird"),
            bird: FakeBird,
            face_dyno: FaceDyno,
            physics: FakeBirdPhysicsBundle::new(pos, vel),
            multi: multi!([
                (
                    "core",
                    anim_man!({
                        normal: {
                            path: "lenny/fly.png",
                            size: (24, 24),
                            length: 3,
                            fps: 16.0,
                        },
                    })
                    .with_offset(Vec3::new(-1.0, 0.0, 0.0))
                ),
                // (
                //     "light",
                //     anim_man!({
                //         path: "lenny/spotlight.png",
                //         size: (48, 48),
                //         length: 1,
                //     })
                //     .with_render_layers(LightCamera::render_layers())
                //     .with_scale(Vec2::new(2.5, 2.5))
                // ),
            ]),
            particles: DynoAwareParticleSpawner::new(
                Particle::new(default())
                    .with_colors(
                        Color::srgb_u8(245, 219, 203),
                        Color::srgba_u8(110, 181, 196, 0),
                    )
                    .with_sizes(6.0, 4.0),
            ),
        }
    }
}

fn setup_title(
    mut commands: Commands,
    menu_root: Res<MenuRoot>,
    mut music_manager: ResMut<MusicManager>,
) {
    BackgroundKind::SkyOnly.spawn(default(), menu_root.eid(), &mut commands);
    music_manager.fade_to_song(MusicKind::Elegy);
    commands
        .spawn((
            Name::new("title_text"),
            spat_tran(0.0, 0.0, 100.0),
            multi!(anim_man!({
                path: "menu/title_text3D.png",
                size: (160, 60),
            })
            .with_render_layers(MenuCamera::render_layers())),
        ))
        .set_parent(menu_root.eid());
    commands
        .spawn(StickyPlatformBundle::mega_around_room(1.5))
        .set_parent(menu_root.eid());

    for _ in 0..10 {
        let mut bund = FakeBirdBundle::new(
            Vec2::new(thread_rng().gen_range(-160.0..160.0), 0.0),
            default(),
        );
        bund.physics.spatial.transform.translation.z = thread_rng().gen();
        commands.spawn(bund).set_parent(menu_root.eid());
    }
}

fn destroy_title(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands.entity(menu_root.eid()).despawn_descendants();
}

fn update_title(
    mut input: EventReader<NonGameInput>,
    current_transition: Res<State<MetaTransitionState>>,
    mut next_transition: ResMut<NextState<MetaTransitionState>>,
    mut stuck_birds: Query<
        (Entity, &mut DynoTran, &GlobalTransform),
        (With<Stuck>, With<FakeBird>),
    >,
    mut commands: Commands,
) {
    let input = input.read().last();
    if let Some(input) = input {
        match input {
            NonGameInput::Continue => {
                if matches!(current_transition.get(), MetaTransitionState::Stable) {
                    next_transition.set(
                        TransitionKind::FadeToBlack.to_meta_transition_state(
                            1.0,
                            TutorialState::LearnToFly.to_meta_state(),
                        ),
                    );
                }
            }
        }
    }
    for (eid, mut dyno, gt) in &mut stuck_birds {
        commands.entity(eid).remove::<Stuck>();
        dyno.vel = Vec2::new(
            thread_rng().gen_range(-160.0..160.0),
            if gt.translation().y < 0.0 {
                thread_rng().gen_range(60.0..360.0)
            } else {
                thread_rng().gen_range(-20.0..20.0)
            },
        );
    }
}

pub(super) fn register_title(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Title.to_meta_state()), setup_title);
    app.add_systems(OnExit(MenuState::Title.to_meta_state()), destroy_title);
    app.add_systems(
        Update,
        update_title.run_if(in_state(MenuState::Title.to_meta_state())),
    );
}
