use crate::prelude::*;

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
                path: "menu/title_text.png",
                size: (160, 60),
            })
            .with_render_layers(MenuCamera::render_layers())),
        ))
        .set_parent(menu_root.eid());
}

fn destroy_title(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands.entity(menu_root.eid()).despawn_descendants();
}

fn update_title(
    mut input: EventReader<NonGameInput>,
    current_transition: Res<State<MetaTransitionState>>,
    mut next_transition: ResMut<NextState<MetaTransitionState>>,
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
}

pub(super) fn register_title(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Title.to_meta_state()), setup_title);
    app.add_systems(OnExit(MenuState::Title.to_meta_state()), destroy_title);
    app.add_systems(
        Update,
        update_title.run_if(in_state(MenuState::Title.to_meta_state())),
    );
}
