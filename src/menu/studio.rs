use crate::prelude::*;

#[derive(Component)]
struct FadeTimer(Timer);

fn setup_studio(
    mut commands: Commands,
    menu_root: Res<MenuRoot>,
    mut music_manager: ResMut<MusicManager>,
) {
    music_manager.fade_to_song(MusicKind::Elegy);
    commands
        .spawn((
            Name::new("studio_animation"),
            SpatialBundle::default(),
            FadeTimer(Timer::from_seconds(3.8, TimerMode::Once)),
            multi!(anim_man!({
                first_half: {
                    path: "studio/armadillo_games_first_half.png",
                    size: (320, 180),
                    length: 49,
                    fps: 18.0,
                    next: "second_half",
                },
                second_half: {
                    path: "studio/armadillo_games_second_half.png",
                    size: (320, 180),
                    length: 49,
                    fps: 18.0,
                    next: "despawn",
                }
            })
            .with_render_layers(MenuCamera::render_layers())
            .with_offset(Vec3::Z)),
        ))
        .set_parent(menu_root.eid());
}

fn destroy_studio(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands.entity(menu_root.eid()).despawn_descendants();
}

fn update_studio(
    mut fade: Query<&mut FadeTimer>,
    mut next_transition_state: ResMut<NextState<MetaTransitionState>>,
    time: Res<Time>,
) {
    let Ok(mut fade) = fade.get_single_mut() else {
        return;
    };
    fade.0.tick(time.delta());
    if fade.0.just_finished() {
        next_transition_state.set(
            TransitionKind::FadeToBlack
                .to_meta_transition_state(2.2, MenuState::Title.to_meta_state()),
        );
    }
}

pub(super) fn register_studio(app: &mut App) {
    app.add_systems(OnEnter(MetaState::Menu(MenuState::Studio)), setup_studio);
    app.add_systems(OnExit(MetaState::Menu(MenuState::Studio)), destroy_studio);
    app.add_systems(
        Update,
        update_studio.run_if(in_state(MenuState::Studio.to_meta_state())),
    );
}
